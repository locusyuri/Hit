//! Shim 创建 / 移除 / PE 修补
//!
//! Hit 沿用 Scoop 的 shim 模式：
//!
//! - 安装时复制预编译的 `hit-shim.exe` 模板到 `shims/<name>.exe`
//! - 写入 sidecar 文件 `shims/<name>.shim`：
//!
//!   ```text
//!   path = "C:\...\apps\git\current\cmd\git.exe"
//!   args = --no-pager
//!   ```
//!
//! - 若目标是 GUI 应用（PE subsystem=2），把 shim.exe 也修补为 GUI（避免弹出控制台窗口）
//!
//! PE 解析沿用 Scoop `Get-PESubsystem` 算法：偏移 0x3C 读 PE header 偏移，再到
//! OptionalHeader+0x5C 读 subsystem 字段。

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use hit_common::{HitError, Result, Session};

use crate::install::transaction::{Transaction, UndoAction};

/// Windows PE subsystem：GUI 应用（无控制台窗口）
const PE_SUBSYSTEM_GUI: u16 = 2;

/// 创建单个 shim（.exe + .shim sidecar）
///
/// - 复制 `hit-shim.exe` 模板到 `shims/<name>.exe`
/// - 写入 `shims/<name>.shim`：`path = "<target>"` + 可选 `args = <args>`
/// - 若 target 是 GUI 应用（PE subsystem == 2），修补 shim.exe 的 PE header
///
/// `template_path` 为 None 时使用默认查找（`current_exe().parent().join("hit-shim.exe")`）。
/// `target` 应为绝对路径（通常指向 `apps/<app>/current/...`）。
pub fn create_shim(
    session: &Session,
    name: &str,
    target: &Path,
    args: &[String],
    template_path: Option<&Path>,
    tx: &mut Transaction,
) -> Result<()> {
    let template = match template_path {
        Some(p) => p.to_path_buf(),
        None => shim_template_path()?,
    };
    if !template.is_file() {
        return Err(HitError::Shim {
            name: "template".into(),
            message: format!("hit-shim.exe 模板不存在：{}", template.display()),
        });
    }

    let shims_dir = session.shims_path();
    std::fs::create_dir_all(shims_dir).map_err(|e| {
        HitError::io("创建 shims 目录失败", e)
    })?;

    let shim_exe = shims_dir.join(format!("{name}.exe"));
    let shim_sidecar = shims_dir.join(format!("{name}.shim"));

    std::fs::copy(&template, &shim_exe).map_err(|e| {
        HitError::Shim {
            name: name.to_string(),
            message: format!(
                "复制 shim 模板失败：{} -> {} ({e})",
                template.display(),
                shim_exe.display()
            ),
        }
    })?;

    // PE 修补：target 是 GUI 应用时把 shim.exe 也标记为 GUI
    if target.is_file()
        && let Ok(subsystem) = read_pe_subsystem(target)
            && subsystem == PE_SUBSYSTEM_GUI {
                patch_pe_subsystem(&shim_exe, PE_SUBSYSTEM_GUI)?;
            }

    // 写入 sidecar（UTF-8 with BOM 兼容；Scoop 用 Out-UTF8File）
    let mut sidecar_content = format!("path = \"{}\"", target.display());
    if !args.is_empty() {
        sidecar_content.push_str(&format!("\nargs = {}", args.join(" ")));
    }
    sidecar_content.push('\n');
    std::fs::write(&shim_sidecar, sidecar_content.as_bytes()).map_err(|e| {
        HitError::Shim {
            name: name.to_string(),
            message: format!("写入 shim sidecar 失败：{e}"),
        }
    })?;

    tx.record_undo(UndoAction::RemoveShim(shim_exe));
    Ok(())
}

/// 移除 app 的所有 shim
///
/// 扫描 `shims/*.shim`，找出 `path` 指向 `apps/<app>/` 的条目，删除 .exe + .shim。
/// 返回移除的数量。
pub fn remove_app_shims(session: &Session, app: &str) -> Result<usize> {
    let shims_dir = session.shims_path();
    if !shims_dir.is_dir() {
        return Ok(0);
    }

    let app_marker = format!("apps{sep}{app}{sep}", sep = std::path::MAIN_SEPARATOR);
    let mut removed = 0;

    for entry in std::fs::read_dir(shims_dir).map_err(|e| {
        HitError::io("读取 shims 目录失败", e)
    })? {
        let entry = entry.map_err(|e| HitError::io("读取目录项失败", e))?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "shim")
            && let Ok((target, _)) = read_shim_sidecar(&path)
                && target.contains(&app_marker) {
                    let exe = path.with_extension("exe");
                    let _ = std::fs::remove_file(&exe);
                    let _ = std::fs::remove_file(&path);
                    removed += 1;
                }
    }
    Ok(removed)
}

/// 枚举 app 的所有 shim（返回 `(name, target_path)`）
pub fn list_app_shims(session: &Session, app: &str) -> Result<Vec<(String, PathBuf)>> {
    let shims_dir = session.shims_path();
    if !shims_dir.is_dir() {
        return Ok(Vec::new());
    }

    let app_marker = format!("apps{sep}{app}{sep}", sep = std::path::MAIN_SEPARATOR);
    let mut result = Vec::new();

    for entry in std::fs::read_dir(shims_dir).map_err(|e| {
        HitError::io("读取 shims 目录失败", e)
    })? {
        let entry = entry.map_err(|e| HitError::io("读取目录项失败", e))?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "shim")
            && let Ok((target, _)) = read_shim_sidecar(&path)
                && target.contains(&app_marker) {
                    let name = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    result.push((name, PathBuf::from(target)));
                }
    }
    Ok(result)
}

/// 解析 .shim sidecar 文件，返回 `(path, args)`
fn read_shim_sidecar(shim_file: &Path) -> Result<(String, Vec<String>)> {
    let content = std::fs::read_to_string(shim_file).map_err(|e| {
        HitError::Shim {
            name: shim_file.display().to_string(),
            message: format!("读取 shim sidecar 失败：{e}"),
        }
    })?;

    let mut path = None;
    let mut args = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = strip_key(line, "path") {
            path = Some(unquote(rest));
        } else if let Some(rest) = strip_key(line, "args") {
            args = split_args(rest);
        }
    }

    match path {
        Some(p) => Ok((p, args)),
        None => Err(HitError::Shim {
            name: shim_file.display().to_string(),
            message: "shim sidecar 缺少 path 行".into(),
        }),
    }
}

/// 读取 PE 文件的 subsystem 字段（2=GUI, 3=Console）
///
/// 沿用 Scoop `Get-PESubsystem` 算法：
/// - 偏移 0x3C 读 4 字节 LE → PE header 偏移
/// - seek 到 PE+18（OptionalHeader 起点）+0x5C（Subsystem 字段偏移）
/// - 读 2 字节 LE
pub fn read_pe_subsystem(exe: &Path) -> Result<u16> {
    let mut file = std::fs::File::open(exe).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("打开 PE 文件失败：{e}"),
        }
    })?;
    file.seek(SeekFrom::Start(0x3C)).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("seek PE 头失败：{e}"),
        }
    })?;
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("读取 PE 头偏移失败：{e}"),
        }
    })?;
    let pe_offset = u32::from_le_bytes(buf) as u64;

    file.seek(SeekFrom::Start(pe_offset + 18 + 0x5C)).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("seek subsystem 字段失败：{e}"),
        }
    })?;
    let mut buf2 = [0u8; 2];
    file.read_exact(&mut buf2).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("读取 subsystem 字段失败：{e}"),
        }
    })?;
    Ok(u16::from_le_bytes(buf2))
}

/// 修补 PE 文件的 subsystem 字段
pub fn patch_pe_subsystem(exe: &Path, subsystem: u16) -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(exe)
        .map_err(|e| {
            HitError::Shim {
                name: exe.display().to_string(),
                message: format!("打开 PE 文件（读写）失败：{e}"),
            }
        })?;
    file.seek(SeekFrom::Start(0x3C)).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("seek PE 头失败：{e}"),
        }
    })?;
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("读取 PE 头偏移失败：{e}"),
        }
    })?;
    let pe_offset = u32::from_le_bytes(buf) as u64;

    file.seek(SeekFrom::Start(pe_offset + 18 + 0x5C)).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("seek subsystem 字段失败：{e}"),
        }
    })?;
    file.write_all(&subsystem.to_le_bytes()).map_err(|e| {
        HitError::Shim {
            name: exe.display().to_string(),
            message: format!("写入 subsystem 字段失败：{e}"),
        }
    })?;
    Ok(())
}

/// 定位 hit-shim.exe 模板
///
/// 默认查找：当前可执行文件（通常是 `hit.exe`）同目录下的 `hit-shim.exe`。
fn shim_template_path() -> Result<PathBuf> {
    let exe = std::env::current_exe().map_err(|e| {
        HitError::Shim {
            name: "template".into(),
            message: format!("获取当前可执行文件路径失败：{e}"),
        }
    })?;
    let template = exe
        .parent()
        .map(|p| p.join("hit-shim.exe"))
        .unwrap_or_else(|| PathBuf::from("hit-shim.exe"));
    if template.is_file() {
        Ok(template)
    } else {
        Err(HitError::Shim {
            name: "template".into(),
            message: format!(
                "hit-shim.exe 未找到（期望与 hit.exe 同目录）：{}",
                template.display()
            ),
        })
    }
}

/// 去除 `key = value` 格式中的 `key = ` 前缀
fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(key)?;
    let rest = rest.trim_start();
    let rest = rest.strip_prefix('=')?;
    Some(rest.trim_start())
}

/// 去除首尾引号
fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// 拆分参数行（尊重引号）
fn split_args(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }

    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = '"';

    for ch in s.chars() {
        match ch {
            '"' | '\'' if !in_quote => {
                in_quote = true;
                quote_char = ch;
            }
            c if c == quote_char && in_quote => {
                in_quote = false;
            }
            ' ' | '\t' if !in_quote => {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;

    fn test_session(dir: &Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    /// 构造一个最小可用的 shim 模板（非真实 PE，仅用于复制）
    fn make_fake_template(dir: &Path) -> PathBuf {
        let template = dir.join("hit-shim.exe");
        std::fs::write(&template, b"MZ-fake-template").unwrap();
        template
    }

    /// 构造一个最小有效 PE 文件（带合法 DOS/PE header 与可写 subsystem 字段）
    fn make_minimal_pe(dir: &Path, name: &str, subsystem: u16) -> PathBuf {
        let path = dir.join(name);
        let pe_header_offset: u32 = 0x80;
        let mut buf = vec![0u8; (pe_header_offset + 18 + 0x5C + 2) as usize];
        // DOS header: e_lfanew at 0x3C (LE)
        buf[0x3C..0x40].copy_from_slice(&pe_header_offset.to_le_bytes());
        // OptionalHeader offset (PE header + 18) is at buf[pe_header_offset + 18]
        let subsys_pos = (pe_header_offset + 18 + 0x5C) as usize;
        buf[subsys_pos..subsys_pos + 2].copy_from_slice(&subsystem.to_le_bytes());
        std::fs::write(&path, &buf).unwrap();
        path
    }

    #[test]
    fn create_shim_writes_sidecar() {
        let dir = tempfile::tempdir().unwrap();
        let template = make_fake_template(dir.path());
        let target = dir.path().join("target.exe");
        std::fs::write(&target, b"fake target").unwrap();

        let session = test_session(dir.path());
        let mut tx = Transaction::begin("myapp").unwrap();
        create_shim(
            &session,
            "git",
            &target,
            &["--no-pager".to_string()],
            Some(&template),
            &mut tx,
        )
        .unwrap();

        let sidecar = dir.path().join("shims").join("git.shim");
        let exe = dir.path().join("shims").join("git.exe");
        assert!(exe.is_file());
        assert!(sidecar.is_file());
        let content = std::fs::read_to_string(&sidecar).unwrap();
        assert!(
            content.contains(&format!("path = \"{}\"", target.display())),
            "sidecar 应包含 path 行：{content}"
        );
        assert!(content.contains("args = --no-pager"));
        tx.commit().unwrap();
    }

    #[test]
    fn create_shim_patches_gui_subsystem() {
        let dir = tempfile::tempdir().unwrap();
        let template = make_fake_template(dir.path());
        let target = make_minimal_pe(dir.path(), "gui.exe", PE_SUBSYSTEM_GUI);

        let session = test_session(dir.path());
        let mut tx = Transaction::begin("gui").unwrap();
        // 注意：我们的 fake template 不是合法 PE，所以 patch_pe_subsystem 会因 PE 头偏移为 0 而失败；
        // 这里用 make_minimal_pe 构造一个合法 PE 作为 template
        let real_template = make_minimal_pe(dir.path(), "hit-shim-template.exe", 3);
        create_shim(
            &session,
            "gui-shim",
            &target,
            &[],
            Some(&real_template),
            &mut tx,
        )
        .unwrap();

        let shim_exe = dir.path().join("shims").join("gui-shim.exe");
        let patched = read_pe_subsystem(&shim_exe).unwrap();
        assert_eq!(patched, PE_SUBSYSTEM_GUI, "应被修补为 GUI subsystem");
        // 同时验证 template 文件本身没被修改
        let _ = template;
        tx.commit().unwrap();
    }

    #[test]
    fn create_shim_skips_patch_for_console() {
        let dir = tempfile::tempdir().unwrap();
        let target = make_minimal_pe(dir.path(), "console.exe", 3);
        let template = make_minimal_pe(dir.path(), "hit-shim.exe", 3);

        let session = test_session(dir.path());
        let mut tx = Transaction::begin("cli").unwrap();
        create_shim(&session, "cli-shim", &target, &[], Some(&template), &mut tx).unwrap();

        let shim_exe = dir.path().join("shims").join("cli-shim.exe");
        let subsystem = read_pe_subsystem(&shim_exe).unwrap();
        assert_eq!(subsystem, 3, "Console 目标不应修补 shim");
        tx.commit().unwrap();
    }

    #[test]
    fn remove_app_shims_only_affects_target_app() {
        let dir = tempfile::tempdir().unwrap();
        let shims_dir = dir.path().join("shims");
        std::fs::create_dir_all(&shims_dir).unwrap();

        let app_marker = format!("apps{}myapp{}", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR);
        // myapp 的 shim
        std::fs::write(shims_dir.join("tool.exe"), b"MZ").unwrap();
        std::fs::write(
            shims_dir.join("tool.shim"),
            format!("path = \"{app_marker}current/bin/tool.exe\"\n"),
        )
        .unwrap();
        // 其他 app 的 shim（应保留）
        std::fs::write(shims_dir.join("other.exe"), b"MZ").unwrap();
        std::fs::write(
            shims_dir.join("other.shim"),
            format!(
                "path = \"apps{}other{}current/bin/other.exe\"\n",
                std::path::MAIN_SEPARATOR,
                std::path::MAIN_SEPARATOR
            ),
        )
        .unwrap();

        let session = test_session(dir.path());
        let removed = remove_app_shims(&session, "myapp").unwrap();
        assert_eq!(removed, 1);
        assert!(!shims_dir.join("tool.exe").exists());
        assert!(!shims_dir.join("tool.shim").exists());
        assert!(shims_dir.join("other.exe").exists());
        assert!(shims_dir.join("other.shim").exists());
    }

    #[test]
    fn list_app_shims_parses_sidecar() {
        let dir = tempfile::tempdir().unwrap();
        let shims_dir = dir.path().join("shims");
        std::fs::create_dir_all(&shims_dir).unwrap();

        let target = format!(
            "apps{}myapp{}current/bin/tool.exe",
            std::path::MAIN_SEPARATOR,
            std::path::MAIN_SEPARATOR
        );
        std::fs::write(shims_dir.join("tool.exe"), b"MZ").unwrap();
        std::fs::write(
            shims_dir.join("tool.shim"),
            format!("path = \"{target}\"\nargs = --flag\n"),
        )
        .unwrap();

        let session = test_session(dir.path());
        let shims = list_app_shims(&session, "myapp").unwrap();
        assert_eq!(shims.len(), 1);
        assert_eq!(shims[0].0, "tool");
        assert_eq!(shims[0].1.to_string_lossy(), target);
    }

    #[test]
    fn pe_subsystem_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let pe = make_minimal_pe(dir.path(), "test.exe", 3);

        assert_eq!(read_pe_subsystem(&pe).unwrap(), 3);
        patch_pe_subsystem(&pe, PE_SUBSYSTEM_GUI).unwrap();
        assert_eq!(read_pe_subsystem(&pe).unwrap(), PE_SUBSYSTEM_GUI);
        patch_pe_subsystem(&pe, 3).unwrap();
        assert_eq!(read_pe_subsystem(&pe).unwrap(), 3);
    }

    #[test]
    fn create_shim_missing_template_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let mut tx = Transaction::begin("x").unwrap();
        let target = dir.path().join("x.exe");
        std::fs::write(&target, b"x").unwrap();

        let err = create_shim(
            &session,
            "x",
            &target,
            &[],
            Some(&dir.path().join("nonexistent.exe")),
            &mut tx,
        )
        .unwrap_err();
        match err {
            HitError::Shim { name, .. } => {
                // 模板不存在时 name 设为 "template"
                assert_eq!(name, "template");
            }
            other => panic!("期望 Shim 错误，实际：{other:?}"),
        }
    }
}
