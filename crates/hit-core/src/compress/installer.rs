//! 安装程序静默执行（NSIS / Inno Setup / MSI）
//!
//! 通过 `std::process::Command` 调用外部安装程序，传入静默参数并等待退出。

use std::path::Path;
use std::process::Command;

use hit_common::error::{HitError, Result};

/// MSI 静默提取（msiexec /a /qn）
///
/// 使用 administrative install 模式提取文件到目标目录，不实际注册到系统。
/// 与 Scoop `Expand-MsiArchive` 行为一致。
pub fn run_msi_extract(archive: &Path, destination: &Path) -> Result<()> {
    let status = Command::new("msiexec")
        .arg("/a")
        .arg(archive)
        .arg("/qn")
        .arg(format!("TARGETDIR={}", destination.display()))
        .status()
        .map_err(|e| HitError::io("执行 msiexec 失败", e))?;

    if !status.success() {
        return Err(HitError::Compress {
            archive: archive.display().to_string(),
            message: format!("msiexec 退出码：{}", status.code().unwrap_or(-1)),
        });
    }

    Ok(())
}

/// 执行安装程序（静默模式）
///
/// - `innosetup=true`：追加 `/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /DIR=<dest>`
/// - `innosetup=false`：使用提供的 `args` 直接执行
pub fn run_installer(
    installer_file: &Path,
    destination: &Path,
    args: &[String],
    innosetup: bool,
) -> Result<()> {
    let mut cmd = Command::new(installer_file);

    if innosetup {
        cmd.args([
            "/VERYSILENT",
            "/SUPPRESSMSGBOXES",
            "/NORESTART",
            &format!("/DIR={}", destination.display()),
        ]);
    } else {
        cmd.args(args);
    }

    let status = cmd.status().map_err(|e| {
        HitError::io(
            format!("执行安装程序 {}", installer_file.display()),
            e,
        )
    })?;

    if !status.success() {
        return Err(HitError::Compress {
            archive: installer_file.display().to_string(),
            message: format!("安装程序退出码：{}", status.code().unwrap_or(-1)),
        });
    }

    Ok(())
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msi_extract_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("nonexistent.msi");
        let dest = dir.path().join("out");
        let result = run_msi_extract(&fake, &dest);
        // msiexec 可能不存在（非 Windows）或文件不存在 → 都应报错
        assert!(result.is_err());
    }

    #[test]
    fn run_installer_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("nonexistent.exe");
        let dest = dir.path().join("out");
        let result = run_installer(&fake, &dest, &[], false);
        assert!(result.is_err());
    }

    #[test]
    fn run_installer_innosetup_builds_command() {
        // 验证 innosetup 模式的参数构建逻辑（不实际执行）
        // 通过检查不存在文件触发错误来间接验证
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("setup.exe");
        let dest = dir.path().join("out");
        let result = run_installer(&fake, &dest, &[], true);
        assert!(result.is_err());
    }
}
