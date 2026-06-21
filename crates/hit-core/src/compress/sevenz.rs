//! 7-Zip 解压（基于 `sevenz-rust` crate）
//!
//! 支持：
//! - `extract_dir` 子目录提取（extract-then-move 策略）
//! - tar-in-7z 递归（提取后扫描 .tar 文件并递归提取）

use std::fs;
use std::path::Path;

use hit_common::error::{HitError, Result};

/// 解压 7z 归档到目标目录
///
/// - `extract_dir`：先全量解压到临时目录，再移动子目录内容到目标
/// - tar-in-7z：提取后扫描 `.tar` 文件，递归调用 `extract_tar` 并删除中间文件
pub fn extract_7z(
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
) -> Result<()> {
    if let Some(sub_dir) = extract_dir {
        extract_7z_with_dir(archive, destination, sub_dir)
    } else {
        extract_7z_full(archive, destination)
    }
}

/// 全量解压 7z 归档
fn extract_7z_full(archive: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .map_err(|e| HitError::io(format!("创建目录 {}", destination.display()), e))?;

    sevenz_rust2::decompress_file(archive, destination).map_err(|e| HitError::Compress {
        archive: archive.display().to_string(),
        message: format!("7z 解压失败：{e}"),
    })?;

    extract_inner_tars(destination, archive)
}

/// 带 extract_dir 的 7z 解压：先全量解压到临时目录，再移动子目录
fn extract_7z_with_dir(
    archive: &Path,
    destination: &Path,
    extract_dir: &str,
) -> Result<()> {
    let temp = tempfile::tempdir().map_err(|e| HitError::io("创建临时目录失败", e))?;

    sevenz_rust2::decompress_file(archive, temp.path()).map_err(|e| HitError::Compress {
        archive: archive.display().to_string(),
        message: format!("7z 解压失败：{e}"),
    })?;

    let source = temp.path().join(extract_dir.trim_end_matches('/'));
    if !source.is_dir() {
        return Err(HitError::Compress {
            archive: archive.display().to_string(),
            message: format!("extract_dir '{}' 不存在于归档中", extract_dir),
        });
    }

    fs::create_dir_all(destination)
        .map_err(|e| HitError::io(format!("创建目录 {}", destination.display()), e))?;

    // 移动子目录内容到目标
    for entry in fs::read_dir(&source)
        .map_err(|e| HitError::io(format!("读取目录 {}", source.display()), e))?
    {
        let entry =
            entry.map_err(|e| HitError::io(format!("读取条目 {}", source.display()), e))?;
        let target = destination.join(entry.file_name());
        fs::rename(entry.path(), &target).map_err(|e| {
            HitError::io(
                format!(
                    "移动 {} -> {}",
                    entry.path().display(),
                    target.display()
                ),
                e,
            )
        })?;
    }

    // temp 自动清理

    extract_inner_tars(destination, archive)
}

/// 扫描目标目录中的 .tar 文件，递归提取并删除中间文件
fn extract_inner_tars(destination: &Path, _archive: &Path) -> Result<()> {
    let tar_files: Vec<_> = fs::read_dir(destination)
        .map_err(|e| HitError::io(format!("读取目录 {}", destination.display()), e))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "tar")
        })
        .map(|e| e.path())
        .collect();

    for tar_path in tar_files {
        // 提取到 .tar 同目录
        super::tar::extract_tar(&tar_path, destination, None)?;
        fs::remove_file(&tar_path).map_err(|e| {
            HitError::io(
                format!("删除中间 tar {}", tar_path.display()),
                e,
            )
        })?;
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
    fn extract_7z_nonexistent_file() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("nonexistent.7z");
        let dest = dir.path().join("out");
        let result = extract_7z(&fake, &dest, None);
        assert!(result.is_err());
    }

    #[test]
    fn extract_7z_invalid_file() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("bad.7z");
        fs::write(&fake, b"not a 7z file").unwrap();
        let dest = dir.path().join("out");
        let result = extract_7z(&fake, &dest, None);
        assert!(result.is_err());
    }
}
