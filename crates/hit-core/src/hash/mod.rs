//! 哈希校验模块
//!
//! 支持算法：md5、sha1、sha256、sha512、blake3
//! - 流式计算（8KB buffer，避免大文件内存问题）
//! - 算法识别：`algo:` 前缀或 hex 长度推断（与 Scoop/Hok 兼容）
//! - 校验失败返回 `HitError::HashMismatch`

use std::fs;
use std::io::Read;
use std::path::Path;

use hit_common::error::{HitError, Result};
use md5::Digest;
use sha2::Sha256;

/// 哈希算法类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Blake3,
}

impl HashAlgorithm {
    /// 从 manifest 哈希字符串推断算法类型
    ///
    /// 规则（与 Scoop/Hok 兼容）：
    /// 1. 含 `:` 前缀 → 按前缀识别（md5/sha1/sha256/sha512/blake3）
    /// 2. 无前缀 → 按 hex 长度推断（40→sha1, 64→sha256, 128→sha512）
    /// 3. md5 和 blake3 必须带前缀（md5 长度 32、blake3 长度 64 无法与其他算法区分）
    pub fn from_hash_str(hash: &str) -> Option<Self> {
        if let Some((algo, value)) = hash.split_once(':') {
            // 有前缀：按前缀 + 长度验证
            match (algo.to_lowercase().as_str(), value.len()) {
                ("md5", 32) => Some(HashAlgorithm::Md5),
                ("sha1", 40) => Some(HashAlgorithm::Sha1),
                ("sha256", 64) => Some(HashAlgorithm::Sha256),
                ("sha512", 128) => Some(HashAlgorithm::Sha512),
                ("blake3", 64) => Some(HashAlgorithm::Blake3),
                _ => None,
            }
        } else {
            // 无前缀：按 hex 长度推断（md5/blake3 无法识别，需前缀）
            match hash.len() {
                40 => Some(HashAlgorithm::Sha1),
                64 => Some(HashAlgorithm::Sha256),
                128 => Some(HashAlgorithm::Sha512),
                _ => None,
            }
        }
    }

    /// 返回算法名称（用于日志/错误消息）
    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Md5 => "md5",
            HashAlgorithm::Sha1 => "sha1",
            HashAlgorithm::Sha256 => "sha256",
            HashAlgorithm::Sha512 => "sha512",
            HashAlgorithm::Blake3 => "blake3",
        }
    }
}

/// 流式计算文件哈希
///
/// 使用 8KB buffer 循环读取文件，避免大文件内存问题。
/// 返回小写 hex 字符串。
pub fn compute_file_hash(path: &Path, algorithm: HashAlgorithm) -> Result<String> {
    let mut file = fs::File::open(path)
        .map_err(|e| HitError::io(format!("打开文件失败: {}", path.display()), e))?;

    let mut buffer = [0u8; 8192];

    match algorithm {
        HashAlgorithm::Md5 => {
            let mut hasher = md5::Md5::new();
            stream_file(&mut file, &mut buffer, |chunk| hasher.update(chunk))?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::Sha1 => {
            let mut hasher = sha1::Sha1::new();
            stream_file(&mut file, &mut buffer, |chunk| hasher.update(chunk))?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            stream_file(&mut file, &mut buffer, |chunk| hasher.update(chunk))?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::Sha512 => {
            let mut hasher = sha2::Sha512::new();
            stream_file(&mut file, &mut buffer, |chunk| hasher.update(chunk))?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            stream_file(&mut file, &mut buffer, |chunk| {
                hasher.update(chunk);
            })?;
            Ok(hasher.finalize().to_hex().to_string())
        }
    }
}

/// 循环读取文件并通过回调更新 hasher
fn stream_file<F>(file: &mut fs::File, buffer: &mut [u8], mut update: F) -> Result<()>
where
    F: FnMut(&[u8]),
{
    loop {
        let n = file
            .read(buffer)
            .map_err(|e| HitError::io("读取文件失败", e))?;
        if n == 0 {
            break;
        }
        update(&buffer[..n]);
    }
    Ok(())
}

/// 校验文件哈希
///
/// - 推断算法 → 计算实际哈希 → 与期望值比较
/// - 匹配返回 Ok(())，不匹配返回 `HitError::HashMismatch`
/// - 无法识别算法时返回 `HitError::InvalidArgument`
pub fn verify_file_hash(path: &Path, expected: &str) -> Result<()> {
    let algorithm = HashAlgorithm::from_hash_str(expected).ok_or_else(|| HitError::InvalidArgument {
        message: format!("无法识别哈希算法: {}", expected),
    })?;

    // 去除 algo: 前缀，取纯 hex 部分，转为小写
    let expected_hash = expected
        .split_once(':')
        .map(|(_, v)| v)
        .unwrap_or(expected)
        .to_lowercase();

    let actual_hash = compute_file_hash(path, algorithm)?;

    if actual_hash != expected_hash {
        return Err(HitError::HashMismatch {
            path: path.to_path_buf(),
            expected: expected_hash,
            actual: actual_hash,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── 已知测试向量 ──────────────────────────────────────────────
    // 空文件（0 字节）
    const EMPTY_SHA256: &str =
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const EMPTY_BLAKE3: &str =
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
    // "hello"
    const HELLO_SHA256: &str =
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    const HELLO_SHA1: &str = "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d";
    const HELLO_MD5: &str = "5d41402abc4b2a76b9719d911017c592";
    const HELLO_SHA512: &str = "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043";

    // ── 算法识别测试 ──────────────────────────────────────────────

    #[test]
    fn from_hash_str_sha256_no_prefix() {
        let hash = "a".repeat(64);
        assert_eq!(
            HashAlgorithm::from_hash_str(&hash),
            Some(HashAlgorithm::Sha256)
        );
    }

    #[test]
    fn from_hash_str_sha256_prefixed() {
        let hash = format!("sha256:{}", "a".repeat(64));
        assert_eq!(
            HashAlgorithm::from_hash_str(&hash),
            Some(HashAlgorithm::Sha256)
        );
    }

    #[test]
    fn from_hash_str_sha512() {
        let hash = "a".repeat(128);
        assert_eq!(
            HashAlgorithm::from_hash_str(&hash),
            Some(HashAlgorithm::Sha512)
        );
    }

    #[test]
    fn from_hash_str_sha1() {
        let hash = "a".repeat(40);
        assert_eq!(
            HashAlgorithm::from_hash_str(&hash),
            Some(HashAlgorithm::Sha1)
        );
    }

    #[test]
    fn from_hash_str_md5() {
        // md5 必须带前缀（bare 32-char hex 不被识别）
        let hash = format!("md5:{}", "a".repeat(32));
        assert_eq!(
            HashAlgorithm::from_hash_str(&hash),
            Some(HashAlgorithm::Md5)
        );
    }

    #[test]
    fn from_hash_str_blake3_prefixed() {
        let hash = format!("blake3:{}", "a".repeat(64));
        assert_eq!(
            HashAlgorithm::from_hash_str(&hash),
            Some(HashAlgorithm::Blake3)
        );
    }

    #[test]
    fn from_hash_str_blake3_no_prefix_defaults_sha256() {
        // blake3 无前缀时长度为 64，与 sha256 无法区分，默认 sha256
        let hash = "a".repeat(64);
        assert_eq!(
            HashAlgorithm::from_hash_str(&hash),
            Some(HashAlgorithm::Sha256)
        );
    }

    #[test]
    fn from_hash_str_invalid() {
        assert_eq!(HashAlgorithm::from_hash_str("not_a_hash"), None);
        assert_eq!(HashAlgorithm::from_hash_str(""), None);
        // 长度不匹配
        assert_eq!(HashAlgorithm::from_hash_str("sha256:abc"), None);
        assert_eq!(HashAlgorithm::from_hash_str("blake3:abc"), None);
    }

    // ── 哈希计算测试 ──────────────────────────────────────────────

    #[test]
    fn compute_file_hash_sha256() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        // 空文件
        let hash = compute_file_hash(tmp.path(), HashAlgorithm::Sha256).unwrap();
        assert_eq!(hash, EMPTY_SHA256);

        // "hello" 内容
        fs::write(tmp.path(), b"hello").unwrap();
        let hash = compute_file_hash(tmp.path(), HashAlgorithm::Sha256).unwrap();
        assert_eq!(hash, HELLO_SHA256);
    }

    #[test]
    fn compute_file_hash_sha512() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), b"hello").unwrap();
        let hash = compute_file_hash(tmp.path(), HashAlgorithm::Sha512).unwrap();
        assert_eq!(hash, HELLO_SHA512);
    }

    #[test]
    fn compute_file_hash_blake3() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        // 空文件
        let hash = compute_file_hash(tmp.path(), HashAlgorithm::Blake3).unwrap();
        assert_eq!(hash, EMPTY_BLAKE3);
    }

    #[test]
    fn compute_file_hash_sha1() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), b"hello").unwrap();
        let hash = compute_file_hash(tmp.path(), HashAlgorithm::Sha1).unwrap();
        assert_eq!(hash, HELLO_SHA1);
    }

    #[test]
    fn compute_file_hash_md5() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), b"hello").unwrap();
        let hash = compute_file_hash(tmp.path(), HashAlgorithm::Md5).unwrap();
        assert_eq!(hash, HELLO_MD5);
    }

    #[test]
    fn compute_file_hash_nonexistent() {
        let result = compute_file_hash(
            Path::new("/nonexistent/path/file.bin"),
            HashAlgorithm::Sha256,
        );
        assert!(result.is_err());
    }

    // ── 哈希校验测试 ──────────────────────────────────────────────

    #[test]
    fn verify_file_hash_match() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), b"hello").unwrap();

        // 有前缀
        assert!(verify_file_hash(tmp.path(), &format!("sha256:{}", HELLO_SHA256)).is_ok());
        // 无前缀
        assert!(verify_file_hash(tmp.path(), HELLO_SHA256).is_ok());
    }

    #[test]
    fn verify_file_hash_mismatch() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), b"hello").unwrap();

        let wrong_hash = "0".repeat(64);
        let result = verify_file_hash(tmp.path(), &wrong_hash);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            HitError::HashMismatch {
                expected,
                actual,
                path,
            } => {
                assert_eq!(expected, wrong_hash);
                assert_eq!(actual, HELLO_SHA256);
                assert_eq!(path, tmp.path());
            }
            _ => panic!("期望 HashMismatch 错误"),
        }
    }

    #[test]
    fn verify_file_hash_no_prefix() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), b"hello").unwrap();

        // 无前缀 sha256 校验
        assert!(verify_file_hash(tmp.path(), HELLO_SHA256).is_ok());

        // 无前缀 sha1 校验
        assert!(verify_file_hash(tmp.path(), HELLO_SHA1).is_ok());

        // md5 必须带前缀（bare 32-char hex 不被识别）
        assert!(verify_file_hash(tmp.path(), &format!("md5:{}", HELLO_MD5)).is_ok());
    }

    #[test]
    fn verify_file_hash_unknown_algorithm() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), b"hello").unwrap();

        let result = verify_file_hash(tmp.path(), "unknown:abc123");
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            HitError::InvalidArgument { message } => {
                assert!(message.contains("无法识别哈希算法"));
            }
            _ => panic!("期望 InvalidArgument 错误"),
        }
    }
}
