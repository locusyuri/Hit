//! ZIP 解压（基于 `zip` crate v2）
//!
//! 支持 `extract_dir` 子目录过滤：仅提取指定子目录下的文件并去除前缀。

use std::fs;
use std::io::{self, BufWriter};
use std::path::Path;

use hit_common::error::{HitError, Result};

/// 解压 ZIP 归档到目标目录
///
/// - `extract_dir`：若指定，仅提取该子目录下的文件并去除前缀
/// - 覆盖已存在文件（与 Scoop Expand-ZipArchive -Force 行为一致）
/// - 跳过目录条目（仅创建文件所需的父目录）
pub fn extract_zip(
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
) -> Result<()> {
    let file = fs::File::open(archive)
        .map_err(|e| HitError::io(format!("打开 ZIP {}", archive.display()), e))?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| HitError::Compress {
        archive: archive.display().to_string(),
        message: format!("ZIP 解析失败：{e}"),
    })?;

    fs::create_dir_all(destination)
        .map_err(|e| HitError::io(format!("创建目录 {}", destination.display()), e))?;

    let prefix = extract_dir.map(normalize_extract_dir);

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| HitError::Compress {
            archive: archive.display().to_string(),
            message: format!("读取条目失败：{e}"),
        })?;

        let raw_path = match entry.enclosed_name() {
            Some(p) => p.to_path_buf(),
            None => continue,
        };

        let target_path = match &prefix {
            Some(pfx) => match raw_path.strip_prefix(pfx) {
                Ok(stripped) => {
                    if stripped.as_os_str().is_empty() {
                        continue;
                    }
                    destination.join(stripped)
                }
                Err(_) => continue,
            },
            None => destination.join(&raw_path),
        };

        if entry.is_dir() {
            fs::create_dir_all(&target_path)
                .map_err(|e| HitError::io(format!("创建目录 {}", target_path.display()), e))?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    HitError::io(format!("创建目录 {}", parent.display()), e)
                })?;
            }
            let mut out = BufWriter::new(fs::File::create(&target_path).map_err(|e| {
                HitError::io(format!("创建文件 {}", target_path.display()), e)
            })?);
            io::copy(&mut entry, &mut out).map_err(|e| {
                HitError::io(format!("写入文件 {}", target_path.display()), e)
            })?;
        }
    }

    Ok(())
}

/// 规范化 extract_dir：确保以 `/` 结尾（用于 strip_prefix 匹配）
fn normalize_extract_dir(dir: &str) -> std::path::PathBuf {
    let trimmed = dir.trim_end_matches('/');
    std::path::PathBuf::from(trimmed)
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    /// 创建测试用 ZIP 文件
    fn create_test_zip(dir: &Path, entries: &[(&str, &[u8])]) -> std::path::PathBuf {
        let zip_path = dir.join("test.zip");
        let file = fs::File::create(&zip_path).unwrap();
        let mut writer = ZipWriter::new(file);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);

        for (name, content) in entries {
            writer.start_file(*name, options).unwrap();
            writer.write_all(content).unwrap();
        }
        writer.finish().unwrap();
        zip_path
    }

    #[test]
    fn extract_zip_flat() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = create_test_zip(
            dir.path(),
            &[
                ("a.txt", b"hello"),
                ("b.txt", b"world"),
                ("c.txt", b"!"),
            ],
        );

        let dest = dir.path().join("out");
        extract_zip(&zip_path, &dest, None).unwrap();

        assert_eq!(fs::read_to_string(dest.join("a.txt")).unwrap(), "hello");
        assert_eq!(fs::read_to_string(dest.join("b.txt")).unwrap(), "world");
        assert_eq!(fs::read_to_string(dest.join("c.txt")).unwrap(), "!");
    }

    #[test]
    fn extract_zip_nested() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = create_test_zip(
            dir.path(),
            &[
                ("root.txt", b"root"),
                ("sub/nested.txt", b"nested"),
                ("sub/deep/file.txt", b"deep"),
            ],
        );

        let dest = dir.path().join("out");
        extract_zip(&zip_path, &dest, None).unwrap();

        assert_eq!(fs::read_to_string(dest.join("root.txt")).unwrap(), "root");
        assert_eq!(
            fs::read_to_string(dest.join("sub/nested.txt")).unwrap(),
            "nested"
        );
        assert_eq!(
            fs::read_to_string(dest.join("sub/deep/file.txt")).unwrap(),
            "deep"
        );
    }

    #[test]
    fn extract_zip_with_extract_dir() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = create_test_zip(
            dir.path(),
            &[
                ("root.txt", b"root"),
                ("pkg/bin/app.exe", b"binary"),
                ("pkg/lib/lib.a", b"library"),
                ("pkg/README.md", b"readme"),
            ],
        );

        let dest = dir.path().join("out");
        extract_zip(&zip_path, &dest, Some("pkg")).unwrap();

        // root.txt 不应被提取（不在 pkg/ 下）
        assert!(!dest.join("root.txt").exists());
        // pkg/ 下的文件应被提取，去除 pkg/ 前缀
        assert_eq!(fs::read_to_string(dest.join("bin/app.exe")).unwrap(), "binary");
        assert_eq!(fs::read_to_string(dest.join("lib/lib.a")).unwrap(), "library");
        assert_eq!(fs::read_to_string(dest.join("README.md")).unwrap(), "readme");
    }

    #[test]
    fn extract_zip_empty() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = create_test_zip(dir.path(), &[]);

        let dest = dir.path().join("out");
        extract_zip(&zip_path, &dest, None).unwrap();
        assert!(dest.exists());
    }

    #[test]
    fn extract_zip_corrupt() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = dir.path().join("corrupt.zip");
        fs::write(&zip_path, b"this is not a zip file at all").unwrap();

        let dest = dir.path().join("out");
        let result = extract_zip(&zip_path, &dest, None);
        assert!(result.is_err());
    }
}
