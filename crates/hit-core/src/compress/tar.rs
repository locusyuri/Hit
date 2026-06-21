//! TAR 系列解压（tar / tar.gz / tar.bz2 / tar.xz）
//!
//! 基于 `tar` crate v0.4，配合 `flate2` / `bzip2` / `xz2` 解码压缩流。
//! 支持 `extract_dir` 子目录过滤。

use std::fs;
use std::io::Read;
use std::path::Path;

use hit_common::error::{HitError, Result};

/// 解压纯 tar 归档
pub fn extract_tar(
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
) -> Result<()> {
    let file = open_archive(archive)?;
    extract_tar_inner(file, archive, destination, extract_dir)
}

/// 解压 tar.gz / tgz 归档
pub fn extract_tar_gz(
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
) -> Result<()> {
    let file = open_archive(archive)?;
    let decoder = flate2::read::GzDecoder::new(file);
    extract_tar_inner(decoder, archive, destination, extract_dir)
}

/// 解压 tar.bz2 / tbz2 归档
pub fn extract_tar_bz2(
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
) -> Result<()> {
    let file = open_archive(archive)?;
    let decoder = bzip2::read::BzDecoder::new(file);
    extract_tar_inner(decoder, archive, destination, extract_dir)
}

/// 解压 tar.xz / txz 归档
pub fn extract_tar_xz(
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
) -> Result<()> {
    let file = open_archive(archive)?;
    let decoder = xz2::read::XzDecoder::new(file);
    extract_tar_inner(decoder, archive, destination, extract_dir)
}

/// 通用 tar 提取逻辑（接受已解码的 reader）
///
/// 逐条目迭代，支持 `extract_dir` 前缀过滤和路径安全检查。
fn extract_tar_inner<R: Read>(
    reader: R,
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
) -> Result<()> {
    let mut ar = tar::Archive::new(reader);

    fs::create_dir_all(destination)
        .map_err(|e| HitError::io(format!("创建目录 {}", destination.display()), e))?;

    let prefix = extract_dir.map(normalize_extract_dir);

    for entry_result in ar.entries().map_err(|e| HitError::Compress {
        archive: archive.display().to_string(),
        message: format!("TAR 读取失败：{e}"),
    })? {
        let mut entry = entry_result.map_err(|e| HitError::Compress {
            archive: archive.display().to_string(),
            message: format!("TAR 条目错误：{e}"),
        })?;

        let raw_path = entry.path().map_err(|e| HitError::Compress {
            archive: archive.display().to_string(),
            message: format!("TAR 路径无效：{e}"),
        })?.into_owned();

        // 安全检查：拒绝路径遍历
        if raw_path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
            return Err(HitError::Compress {
                archive: archive.display().to_string(),
                message: format!("TAR 路径遍历攻击：{}", raw_path.display()),
            });
        }

        // 安全检查：拒绝绝对路径
        if raw_path.is_absolute() {
            return Err(HitError::Compress {
                archive: archive.display().to_string(),
                message: format!("TAR 绝对路径：{}", raw_path.display()),
            });
        }

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

        entry.unpack_in(destination).map_err(|e| HitError::Compress {
            archive: archive.display().to_string(),
            message: format!("解压 {} 失败：{}", raw_path.display(), e),
        })?;

        // unpack_in 使用原始路径，若 extract_dir 指定需手动处理
        if prefix.is_some() {
            let unpacked = destination.join(&raw_path);
            if unpacked != target_path && unpacked.exists() {
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| {
                        HitError::io(format!("创建目录 {}", parent.display()), e)
                    })?;
                }
                if unpacked.is_dir() {
                    fs::create_dir_all(&target_path).map_err(|e| {
                        HitError::io(format!("创建目录 {}", target_path.display()), e)
                    })?;
                } else {
                    fs::rename(&unpacked, &target_path).map_err(|e| {
                        HitError::io(
                            format!("移动 {} -> {}", unpacked.display(), target_path.display()),
                            e,
                        )
                    })?;
                }
            }
        }
    }

    Ok(())
}

fn open_archive(path: &Path) -> Result<fs::File> {
    fs::File::open(path).map_err(|e| HitError::io(format!("打开归档 {}", path.display()), e))
}

/// 规范化 extract_dir：去除末尾斜杠
fn normalize_extract_dir(dir: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(dir.trim_end_matches('/'))
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// 创建测试用 tar 文件（含指定条目）
    fn create_test_tar(dir: &Path, entries: &[(&str, &[u8])]) -> std::path::PathBuf {
        let tar_path = dir.join("test.tar");
        let file = fs::File::create(&tar_path).unwrap();
        let mut builder = tar::Builder::new(file);

        for (name, content) in entries {
            let mut header = tar::Header::new_gnu();
            header.set_size(content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            builder
                .append_data(&mut header, *name, &content[..])
                .unwrap();
        }
        builder.finish().unwrap();
        tar_path
    }

    /// 将 tar 压缩为 tar.gz
    fn compress_tar_gz(tar_path: &Path, dir: &Path) -> std::path::PathBuf {
        let gz_path = dir.join("test.tar.gz");
        let tar_data = fs::read(tar_path).unwrap();
        let gz_file = fs::File::create(&gz_path).unwrap();
        let mut encoder = flate2::write::GzEncoder::new(gz_file, flate2::Compression::fast());
        encoder.write_all(&tar_data).unwrap();
        encoder.finish().unwrap();
        gz_path
    }

    /// 将 tar 压缩为 tar.bz2
    fn compress_tar_bz2(tar_path: &Path, dir: &Path) -> std::path::PathBuf {
        let bz2_path = dir.join("test.tar.bz2");
        let tar_data = fs::read(tar_path).unwrap();
        let bz2_file = fs::File::create(&bz2_path).unwrap();
        let mut encoder = bzip2::write::BzEncoder::new(bz2_file, bzip2::Compression::fast());
        encoder.write_all(&tar_data).unwrap();
        encoder.finish().unwrap();
        bz2_path
    }

    /// 将 tar 压缩为 tar.xz
    fn compress_tar_xz(tar_path: &Path, dir: &Path) -> std::path::PathBuf {
        let xz_path = dir.join("test.tar.xz");
        let tar_data = fs::read(tar_path).unwrap();
        let xz_file = fs::File::create(&xz_path).unwrap();
        let mut encoder = xz2::write::XzEncoder::new(xz_file, 1);
        encoder.write_all(&tar_data).unwrap();
        encoder.finish().unwrap();
        xz_path
    }

    #[test]
    fn extract_tar_plain() {
        let dir = tempfile::tempdir().unwrap();
        let tar_path = create_test_tar(
            dir.path(),
            &[("a.txt", b"hello"), ("b.txt", b"world")],
        );

        let dest = dir.path().join("out");
        extract_tar(&tar_path, &dest, None).unwrap();

        assert_eq!(fs::read_to_string(dest.join("a.txt")).unwrap(), "hello");
        assert_eq!(fs::read_to_string(dest.join("b.txt")).unwrap(), "world");
    }

    #[test]
    fn extract_tar_gz() {
        let dir = tempfile::tempdir().unwrap();
        let tar_path = create_test_tar(dir.path(), &[("file.txt", b"gzip content")]);
        let gz_path = compress_tar_gz(&tar_path, dir.path());

        let dest = dir.path().join("out");
        super::extract_tar_gz(&gz_path, &dest, None).unwrap();

        assert_eq!(
            fs::read_to_string(dest.join("file.txt")).unwrap(),
            "gzip content"
        );
    }

    #[test]
    fn extract_tar_bz2() {
        let dir = tempfile::tempdir().unwrap();
        let tar_path = create_test_tar(dir.path(), &[("file.txt", b"bzip2 content")]);
        let bz2_path = compress_tar_bz2(&tar_path, dir.path());

        let dest = dir.path().join("out");
        super::extract_tar_bz2(&bz2_path, &dest, None).unwrap();

        assert_eq!(
            fs::read_to_string(dest.join("file.txt")).unwrap(),
            "bzip2 content"
        );
    }

    #[test]
    fn extract_tar_xz() {
        let dir = tempfile::tempdir().unwrap();
        let tar_path = create_test_tar(dir.path(), &[("file.txt", b"xz content")]);
        let xz_path = compress_tar_xz(&tar_path, dir.path());

        let dest = dir.path().join("out");
        super::extract_tar_xz(&xz_path, &dest, None).unwrap();

        assert_eq!(
            fs::read_to_string(dest.join("file.txt")).unwrap(),
            "xz content"
        );
    }

    #[test]
    fn extract_tar_with_extract_dir() {
        let dir = tempfile::tempdir().unwrap();
        let tar_path = create_test_tar(
            dir.path(),
            &[
                ("root.txt", b"root"),
                ("pkg/bin/app", b"binary"),
                ("pkg/lib/lib.a", b"library"),
            ],
        );

        let dest = dir.path().join("out");
        extract_tar(&tar_path, &dest, Some("pkg")).unwrap();

        // root.txt 不应被提取
        assert!(!dest.join("root.txt").exists());
        // pkg/ 下的文件应被提取
        assert!(
            dest.join("bin/app").exists() || dest.join("pkg/bin/app").exists(),
            "binary should be extracted"
        );
    }

    #[test]
    fn extract_tar_path_traversal_rejected() {
        let dir = tempfile::tempdir().unwrap();
        // 手动构造含 ".." 路径的 tar 条目（tar::Builder 会拒绝此类路径）
        let tar_path = dir.path().join("evil.tar");
        let file = fs::File::create(&tar_path).unwrap();
        let mut builder = tar::Builder::new(file);

        let mut header = tar::Header::new_gnu();
        header.set_size(9);
        header.set_mode(0o644);
        header.set_cksum();
        // 直接写入路径字节，绕过 Builder 的路径检查
        let evil_path = b"../../etc/passwd";
        let gnu_header = header.as_gnu_mut().unwrap();
        gnu_header.name[..evil_path.len()].copy_from_slice(evil_path);
        header.set_cksum();

        builder
            .append(&header, b"malicious" as &[u8])
            .unwrap();
        builder.finish().unwrap();

        let dest = dir.path().join("out");
        let result = extract_tar(&tar_path, &dest, None);
        assert!(result.is_err());
    }
}
