//! 文件系统链接操作：Junction（目录）与 HardLink（文件）
//!
//! 与 Hok 的 symlink-first-then-fallback 策略不同，Hit 采用 junction-only 策略：
//! - 目录链接：`junction::create`（底层调用 `FSCTL_SET_REPARSE_POINT`）
//! - 文件链接：`std::fs::hard_link`
//! - **不使用** symlink（避免开发者模式/管理员权限要求）
//!
//! 参考 Scoop `install.ps1` 的 `create_junction` / `rm_junction`。

use std::fs;
use std::path::{Path, PathBuf};

use hit_common::error::{HitError, Result};

/// 创建目录 Junction 链接
///
/// `lnk` 已存在时先尝试移除，创建后设置 readonly 属性
/// （与 Scoop `attrib +R /L` 一致）。使用多级 fallback 确保删除：
/// 1. `junction::delete()`（最快，但可能因 readonly 或损坏的 junction 失败）
/// 2. `cmd.exe /c rmdir`（Windows 原生，正确删除 junction reparse point）
/// 3. `pwsh -Command Remove-Item -Force -LiteralPath`（最可靠，处理各种边缘情况）
/// 4. `fs::remove_dir()`（不跟随 junction 的目录删除）
pub fn create_junction(src: &Path, lnk: &Path) -> Result<()> {
    if symlink_metadata_exists(lnk) {
        remove_junction_readonly(lnk);
        let deleted = junction::delete(lnk).is_ok()
            || rmdir_junction(lnk)
            || powershell_remove_item(lnk)
            || fs::remove_dir(lnk).is_ok();
        if !deleted {
            tracing::warn!("移除旧 junction 失败: {}，尝试覆盖创建", lnk.display());
        }
    }

    junction::create(src, lnk).map_err(|e| HitError::Io {
        context: format!("创建 Junction: {} -> {}", lnk.display(), src.display()),
        source: std::io::Error::other(e.to_string()),
    })?;

    set_readonly(lnk);
    Ok(())
}

/// 用 `pwsh Remove-Item -Force -LiteralPath` 删除 junction。
/// PowerShell 的 Remove-Item 正确处理各种 reparse point 类型，
/// 且 -Force 跳过 readonly 和隐藏属性检查。
fn powershell_remove_item(lnk: &Path) -> bool {
    let path = lnk.to_str().unwrap_or_default().replace('\'', "''");
    let script = format!("Remove-Item -LiteralPath '{}' -Force -ErrorAction SilentlyContinue", path);
    std::process::Command::new("pwsh")
        .args(["-NoProfile", "-Command", &script])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// 用 `cmd.exe /c rmdir` 删除 junction（Windows 原生，最可靠）。
/// rmdir 在 junction 上只删除 reparse point，不跟随到目标目录。
fn rmdir_junction(lnk: &Path) -> bool {
    std::process::Command::new("cmd.exe")
        .args(["/C", "rmdir", lnk.to_str().unwrap_or_default()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// 用 `cmd.exe /c attrib -R` 清除 junction 的 readonly 属性。
/// Scoop 的 `attrib +R /L` 设置 readonly 在 junction 点自身而非目标目录，
/// 但 Rust 的 `fs::metadata` 在 junction 上会跟随到目标目录，
/// 导致 `fs::set_permissions` 设错目标。因此用 cmd.exe 来操作。
fn remove_junction_readonly(lnk: &Path) {
    let _ = std::process::Command::new("cmd.exe")
        .args(["/C", "attrib", "-R", lnk.to_str().unwrap_or_default()])
        .status();
}

/// 移除目录 Junction 链接
///
/// 移除前清除 readonly 属性（与 Scoop `attrib -R /L` 一致）。
/// 如果 junction 已损坏为普通目录（如之前 `remove_dir` 兜底后的残留），
/// 回退到 `fs::remove_dir` 删除。
pub fn remove_junction(lnk: &Path) -> Result<()> {
    if !lnk.exists() {
        return Ok(());
    }
    remove_junction_readonly(lnk);
    if junction::delete(lnk).is_err() {
        // junction 可能已损坏为普通目录，回退到 remove_dir
        fs::remove_dir(lnk).map_err(|e| HitError::Io {
            context: format!("移除旧目录（非 junction）: {}", lnk.display()),
            source: e,
        })?;
    }
    Ok(())
}

/// 创建文件硬链接
pub fn create_hard_link(src: &Path, lnk: &Path) -> Result<()> {
    if lnk.exists() {
        fs::remove_file(lnk).map_err(|e| HitError::Io {
            context: format!("移除已有文件: {}", lnk.display()),
            source: e,
        })?;
    }
    fs::hard_link(src, lnk).map_err(|e| HitError::Io {
        context: format!("创建硬链接: {} -> {}", lnk.display(), src.display()),
        source: e,
    })
}

/// 移除文件硬链接
pub fn remove_hard_link(lnk: &Path) -> Result<()> {
    if !lnk.exists() {
        return Ok(());
    }
    fs::remove_file(lnk).map_err(|e| HitError::Io {
        context: format!("移除硬链接: {}", lnk.display()),
        source: e,
    })
}

/// 创建 `apps/<app>/current` Junction
///
/// `no_junction=true` 时跳过创建，直接返回 `version_dir` 本身。
/// 否则在 `version_dir` 的父目录下创建名为 `current` 的 Junction。
pub fn link_current(version_dir: &Path, no_junction: bool) -> Result<PathBuf> {
    if no_junction {
        return Ok(version_dir.to_path_buf());
    }

    let parent = version_dir.parent().ok_or_else(|| HitError::Io {
        context: "version_dir 没有父目录".into(),
        source: std::io::Error::other("invalid path"),
    })?;
    let current = parent.join("current");
    create_junction(version_dir, &current)?;
    Ok(current)
}

/// 移除 `apps/<app>/current` Junction
///
/// `no_junction=true` 时跳过。返回 `current` 路径（如果已移除）或 `None`。
pub fn unlink_current(version_dir: &Path, no_junction: bool) -> Result<Option<PathBuf>> {
    if no_junction {
        return Ok(None);
    }

    let parent = version_dir.parent().ok_or_else(|| HitError::Io {
        context: "version_dir 没有父目录".into(),
        source: std::io::Error::other("invalid path"),
    })?;
    let current = parent.join("current");
    remove_junction(&current)?;
    Ok(Some(current))
}

/// 创建 Persist 链接
///
/// 目录使用 Junction，文件使用硬链接。
pub fn create_persist_link(source: &Path, persist_target: &Path) -> Result<()> {
    if source.is_dir() {
        create_junction(source, persist_target)
    } else {
        create_hard_link(source, persist_target)
    }
}

/// 移除 Persist 链接（不删除 persist 目录中的数据）
pub fn remove_persist_link(source: &Path) -> Result<()> {
    if source.is_dir() {
        remove_junction(source)
    } else {
        remove_hard_link(source)
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 使用 symlink_metadata 检测路径是否存在（不跟随 junction）
/// 
/// `fs::exists()` 会跟随 junction 到目标目录，当目标不存在时返回 false，
/// 但 junction reparse point 本身可能仍然存在。此函数直接检测 reparse point，
/// 用于在创建新 junction 前正确清理旧的 reparse point。
fn symlink_metadata_exists(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}

/// 设置目录的 readonly 属性
fn set_readonly(path: &Path) {
    if let Ok(meta) = fs::metadata(path) {
        let mut perms = meta.permissions();
        perms.set_readonly(true);
        let _ = fs::set_permissions(path, perms);
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_junction_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("source");
        let lnk = dir.path().join("link");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("test.txt"), "hello").unwrap();

        create_junction(&src, &lnk).unwrap();
        assert!(lnk.exists());
        assert_eq!(fs::read_to_string(lnk.join("test.txt")).unwrap(), "hello");

        remove_junction(&lnk).unwrap();
    }

    #[test]
    fn remove_junction_cleanup() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("source");
        let lnk = dir.path().join("link");
        fs::create_dir_all(&src).unwrap();

        create_junction(&src, &lnk).unwrap();
        assert!(lnk.exists());

        remove_junction(&lnk).unwrap();
        // Junction 被移除，但源目录仍然存在
        assert!(src.exists());
    }

    #[test]
    fn create_hard_link_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("source.txt");
        let lnk = dir.path().join("link.txt");
        fs::write(&src, "hard link test").unwrap();

        create_hard_link(&src, &lnk).unwrap();
        assert!(lnk.exists());
        assert_eq!(fs::read_to_string(&lnk).unwrap(), "hard link test");

        // 修改硬链接文件，原文件也应同步
        fs::write(&lnk, "modified").unwrap();
        assert_eq!(fs::read_to_string(&src).unwrap(), "modified");
    }

    #[test]
    fn remove_hard_link_cleanup() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("source.txt");
        let lnk = dir.path().join("link.txt");
        fs::write(&src, "data").unwrap();

        create_hard_link(&src, &lnk).unwrap();
        remove_hard_link(&lnk).unwrap();
        assert!(!lnk.exists());
        assert!(src.exists());
    }

    #[test]
    fn link_current_creates_junction() {
        let dir = tempfile::tempdir().unwrap();
        let app_dir = dir.path().join("app");
        let version_dir = app_dir.join("1.0.0");
        fs::create_dir_all(&version_dir).unwrap();

        let current = link_current(&version_dir, false).unwrap();
        assert_eq!(current, app_dir.join("current"));
        assert!(current.exists());

        remove_junction(&current).unwrap();
    }

    #[test]
    fn link_current_skips_when_no_junction() {
        let dir = tempfile::tempdir().unwrap();
        let version_dir = dir.path().join("app").join("1.0.0");
        fs::create_dir_all(&version_dir).unwrap();

        let result = link_current(&version_dir, true).unwrap();
        assert_eq!(result, version_dir);
    }

    #[test]
    fn unlink_current_removes_junction() {
        let dir = tempfile::tempdir().unwrap();
        let app_dir = dir.path().join("app");
        let version_dir = app_dir.join("1.0.0");
        fs::create_dir_all(&version_dir).unwrap();

        link_current(&version_dir, false).unwrap();
        let current_path = app_dir.join("current");
        assert!(current_path.exists());

        let removed = unlink_current(&version_dir, false).unwrap();
        assert_eq!(removed, Some(current_path));
    }

    #[test]
    fn create_persist_link_dir() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("persist_source");
        let target = dir.path().join("persist_target");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("data.json"), "{}").unwrap();

        create_persist_link(&source, &target).unwrap();
        assert!(target.exists());
        assert_eq!(
            fs::read_to_string(target.join("data.json")).unwrap(),
            "{}"
        );

        remove_persist_link(&target).unwrap();
    }
}
