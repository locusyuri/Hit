//! 解压模块
//!
//! 统一入口 `decompress()` 根据文件 magic bytes 自动检测格式，
//! 路由到对应子模块。支持 ZIP / 7z / TAR(.gz/.bz2/.xz) / MSI。
//!
//! 子模块：
//! - `zip`：ZIP 归档解压
//! - `sevenz`：7z 归档解压（含 tar-in-7z 递归）
//! - `tar`：TAR 系列（tar / tar.gz / tar.bz2 / tar.xz）解压
//! - `installer`：NSIS / Inno Setup / MSI 安装程序静默执行

pub mod installer;
pub mod sevenz;
pub mod tar;
pub mod zip;

use std::fs;
use std::io::Read;
use std::path::Path;

use hit_common::error::{HitError, Result};
use hit_common::event::{Event, InstallPhase};
use hit_common::session::Session;

pub use self::installer::{run_installer, run_msi_extract};
pub use self::sevenz::extract_7z;
pub use self::tar::{extract_tar, extract_tar_bz2, extract_tar_gz, extract_tar_xz};
pub use self::zip::extract_zip;

/// 归档格式（由 magic bytes 检测得出）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    /// ZIP 归档（PK\x03\x04）
    Zip,
    /// 7-Zip 归档（37 7A BC AF 27 1C）
    SevenZip,
    /// gzip 压缩（1F 8B）— 可能是 .tar.gz 或纯 .gz
    Gzip,
    /// bzip2 压缩（42 5A 68）— 可能是 .tar.bz2 或纯 .bz2
    Bzip2,
    /// xz 压缩（FD 37 7A 58 5A 00）— 可能是 .tar.xz 或纯 .xz
    Xz,
    /// POSIX tar（offset 257 处 "ustar"）
    Tar,
    /// MSI 安装包（D0 CF 11 E0 — OLE compound document）
    Msi,
    /// PE 可执行文件（4D 5A）— 需配合 manifest.innosetup/installer 判断
    Exe,
}

impl ArchiveFormat {
    /// 返回可读格式名称
    pub fn name(&self) -> &'static str {
        match self {
            Self::Zip => "zip",
            Self::SevenZip => "7z",
            Self::Gzip => "gzip",
            Self::Bzip2 => "bzip2",
            Self::Xz => "xz",
            Self::Tar => "tar",
            Self::Msi => "msi",
            Self::Exe => "exe",
        }
    }
}

/// 从文件 magic bytes 检测归档格式
///
/// 读取前 512 字节，按签名匹配。TAR 的 "ustar" 在 offset 257。
pub fn detect_format(path: &Path) -> Result<ArchiveFormat> {
    let mut buf = [0u8; 512];
    let mut file = fs::File::open(path)
        .map_err(|e| HitError::io(format!("打开归档 {}", path.display()), e))?;
    let n = file
        .read(&mut buf)
        .map_err(|e| HitError::io("读取 magic bytes 失败", e))?;

    if n < 4 {
        return Err(HitError::Compress {
            archive: path.display().to_string(),
            message: "文件过小，无法识别格式".into(),
        });
    }

    // 按特异性从高到低排列（长签名优先）
    if n >= 6 && buf[0..6] == [0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C] {
        return Ok(ArchiveFormat::SevenZip);
    }
    if n >= 6 && buf[0..6] == [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00] {
        return Ok(ArchiveFormat::Xz);
    }
    if buf[0..4] == [0x50, 0x4B, 0x03, 0x04] || buf[0..4] == [0x50, 0x4B, 0x05, 0x06] {
        return Ok(ArchiveFormat::Zip);
    }
    if n >= 3 && buf[0..3] == [0x42, 0x5A, 0x68] {
        return Ok(ArchiveFormat::Bzip2);
    }
    if n >= 2 && buf[0..2] == [0x1F, 0x8B] {
        return Ok(ArchiveFormat::Gzip);
    }
    // TAR: "ustar" at offset 257
    if n >= 263 && &buf[257..262] == b"ustar" {
        return Ok(ArchiveFormat::Tar);
    }
    // MSI / OLE compound document
    if n >= 8 && buf[0..8] == [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1] {
        return Ok(ArchiveFormat::Msi);
    }
    if buf[0..2] == [0x4D, 0x5A] {
        return Ok(ArchiveFormat::Exe);
    }

    Err(HitError::Compress {
        archive: path.display().to_string(),
        message: format!(
            "无法识别的归档格式（前 4 字节：{:02X}{:02X}{:02X}{:02X}）",
            buf[0], buf[1], buf[2], buf[3]
        ),
    })
}

/// 统一解压入口
///
/// 1. `detect_format` 识别格式
/// 2. 路由到对应提取函数
/// 3. 发送 `ExtractStart` + `InstallPhase` 事件
///
/// `Exe` 格式分两种处理：
/// - `innosetup=true`：调用 `run_installer` 静默解压
/// - 无 `innosetup`（单 exe 即程序，如 jq）：直接复制到目标目录，
///   文件名取 URL `#` 后的提示名（如 `...#/jq.exe` → `jq.exe`），
///   无提示时用缓存文件原名
pub fn decompress(
    session: &Session,
    app: &str,
    archive: &Path,
    destination: &Path,
    extract_dir: Option<&str>,
    url: Option<&str>,
    innosetup: bool,
) -> Result<()> {
    session.emit(Event::ExtractStart {
        app: app.to_string(),
        archive: archive.to_path_buf(),
    });
    session.emit(Event::InstallPhaseStart {
        app: app.to_string(),
        phase: InstallPhase::Extract,
    });

    let format = detect_format(archive)?;

    match format {
        ArchiveFormat::Zip => extract_zip(archive, destination, extract_dir)?,
        ArchiveFormat::SevenZip => extract_7z(archive, destination, extract_dir)?,
        ArchiveFormat::Tar => extract_tar(archive, destination, extract_dir)?,
        ArchiveFormat::Gzip => extract_tar_gz(archive, destination, extract_dir)?,
        ArchiveFormat::Bzip2 => extract_tar_bz2(archive, destination, extract_dir)?,
        ArchiveFormat::Xz => extract_tar_xz(archive, destination, extract_dir)?,
        ArchiveFormat::Msi => run_msi_extract(archive, destination)?,
        ArchiveFormat::Exe => {
            if innosetup {
                run_installer(archive, destination, &[], true)?;
            } else {
                // 单 exe 即程序（如 jq）：直接复制到目标目录
                // 文件名优先取 URL `#/` 后的提示名，否则用缓存文件原名
                let dest_name = url
                    .and_then(|u| u.split('#').nth(1))
                    .and_then(|frag| frag.strip_prefix('/'))
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| archive.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default());

                if dest_name.is_empty() {
                    return Err(HitError::Compress {
                        archive: archive.display().to_string(),
                        message: "无法确定单 exe 的目标文件名（URL 无 #/ 提示且缓存文件无名）".into(),
                    });
                }

                let dest_file = destination.join(&dest_name);
                fs::copy(archive, &dest_file).map_err(|e| HitError::io(
                    format!("复制单 exe 失败：{} -> {}", archive.display(), dest_file.display()),
                    e,
                ))?;
            }
        }
    }

    session.emit(Event::InstallPhaseEnd {
        app: app.to_string(),
        phase: InstallPhase::Extract,
    });

    Ok(())
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// 写入指定字节到临时文件，返回路径
    fn write_magic_file(dir: &Path, name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let path = dir.join(name);
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(bytes).unwrap();
        path
    }

    /// 构造足够大的 buffer（512 字节），在指定偏移处写入签名
    fn make_buf_with_sig(sig: &[u8], offset: usize) -> Vec<u8> {
        let mut buf = vec![0u8; 512];
        buf[offset..offset + sig.len()].copy_from_slice(sig);
        buf
    }

    #[test]
    fn detect_format_zip() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0x50, 0x4B, 0x03, 0x04], 0);
        let path = write_magic_file(dir.path(), "test.zip", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::Zip);
    }

    #[test]
    fn detect_format_7z() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C], 0);
        let path = write_magic_file(dir.path(), "test.7z", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::SevenZip);
    }

    #[test]
    fn detect_format_gzip() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0x1F, 0x8B], 0);
        let path = write_magic_file(dir.path(), "test.gz", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::Gzip);
    }

    #[test]
    fn detect_format_bzip2() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0x42, 0x5A, 0x68], 0);
        let path = write_magic_file(dir.path(), "test.bz2", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::Bzip2);
    }

    #[test]
    fn detect_format_xz() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00], 0);
        let path = write_magic_file(dir.path(), "test.xz", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::Xz);
    }

    #[test]
    fn detect_format_tar() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(b"ustar", 257);
        let path = write_magic_file(dir.path(), "test.tar", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::Tar);
    }

    #[test]
    fn detect_format_msi() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1], 0);
        let path = write_magic_file(dir.path(), "test.msi", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::Msi);
    }

    #[test]
    fn detect_format_exe() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0x4D, 0x5A], 0);
        let path = write_magic_file(dir.path(), "test.exe", &data);
        assert_eq!(detect_format(&path).unwrap(), ArchiveFormat::Exe);
    }

    #[test]
    fn detect_format_unknown() {
        let dir = tempfile::tempdir().unwrap();
        let data = make_buf_with_sig(&[0xDE, 0xAD, 0xBE, 0xEF], 0);
        let path = write_magic_file(dir.path(), "test.bin", &data);
        assert!(detect_format(&path).is_err());
    }

    #[test]
    fn detect_format_too_small() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_magic_file(dir.path(), "tiny.bin", &[0x01, 0x02]);
        assert!(detect_format(&path).is_err());
    }
}
