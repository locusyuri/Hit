//! 缓存管理
//!
//! 提供 Scoop/Hok 兼容的缓存文件管理：
//! - 缓存命名格式：`{app}#{version}#{sha256_7}{ext}`
//! - 缓存命中检查（文件存在性）
//! - 下载到缓存（封装 `download_file`，命中时跳过）
//! - 缓存列举与清理

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use hit_common::error::{HitError, Result};
use hit_common::Session;
use sha2::{Digest, Sha256};

use super::http::download_file;

/// 缓存文件信息（用于 `hit cache` 命令展示）
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// 软件名称
    pub app: String,
    /// 版本号
    pub version: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 缓存文件完整路径
    pub path: PathBuf,
}

/// 计算缓存文件名
///
/// 格式：`{app}#{version}#{sha256_7}{ext}`
/// - `sha256_7`：对完整 URL 字符串求 SHA256，取前 7 位 hex（小写）
/// - `ext`：从 URL 路径提取扩展名（含 `.`），无扩展名时为空
///
/// 与 Scoop `cache_path` 和 Hok `download_filenames` 命名格式完全兼容。
fn cache_filename(app: &str, version: &str, url: &str) -> String {
    let hash = Sha256::digest(url.as_bytes());
    let hex = format!("{:x}", hash);
    let sha7 = &hex[..7];

    let ext = Path::new(url)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    format!("{}#{}#{}{}", app, version, sha7, ext)
}

/// 计算缓存文件的完整路径
pub fn cache_path(session: &Session, app: &str, version: &str, url: &str) -> PathBuf {
    session.cache_path().join(cache_filename(app, version, url))
}

/// 检查缓存文件是否存在
///
/// MVP 阶段仅检查文件存在性（与 Scoop 一致）。
/// 后续可通过 HTTP HEAD 比较远程/本地大小进一步验证（参考 Hok）。
pub fn cache_exists(session: &Session, app: &str, version: &str, url: &str) -> bool {
    cache_path(session, app, version, url).exists()
}

/// 下载到缓存（缓存命中时直接返回路径，未命中时下载）
///
/// - 缓存命中：跳过下载，直接返回缓存文件路径
/// - 缓存未命中：调用 `download_file` 下载到缓存路径
/// - 返回缓存文件的完整路径
pub fn download_to_cache(
    session: &Session,
    app: &str,
    version: &str,
    url: &str,
    should_interrupt: &AtomicBool,
) -> Result<PathBuf> {
    let target = cache_path(session, app, version, url);
    if target.exists() {
        return Ok(target);
    }
    download_file(session, url, app, &target, should_interrupt)?;
    Ok(target)
}

/// 列出所有缓存条目
///
/// 遍历 cache 目录，解析文件名中的 app/version，收集文件大小。
/// 无法解析的文件（非 `#` 分隔格式）静默跳过。
/// 目录不存在时返回空列表。
pub fn list_cache(session: &Session) -> Result<Vec<CacheEntry>> {
    let cache_dir = session.cache_path();
    if !cache_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(cache_dir).map_err(|e| HitError::io("读取缓存目录失败", e))? {
        let entry = entry.map_err(|e| HitError::io("读取缓存条目失败", e))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let filename = entry.file_name();
        let filename = filename.to_string_lossy();
        if let Some((app, version)) = parse_cache_filename(&filename) {
            let size = entry
                .metadata()
                .map_err(|e| HitError::io("读取缓存文件元数据失败", e))?
                .len();
            entries.push(CacheEntry {
                app: app.to_string(),
                version: version.to_string(),
                size,
                path,
            });
        }
    }

    entries.sort_by(|a, b| a.app.cmp(&b.app).then(a.version.cmp(&b.version)));
    Ok(entries)
}

/// 删除缓存文件
///
/// - `app = None`：清空所有缓存
/// - `app = Some(name)`：删除指定 app 的所有缓存（不区分大小写）
///
/// 返回已删除文件数。单个文件删除失败不中断，跳过继续。
pub fn remove_cache(session: &Session, app: Option<&str>) -> Result<usize> {
    let entries = list_cache(session)?;
    let to_remove: Vec<&CacheEntry> = match app {
        None => entries.iter().collect(),
        Some(name) => entries.iter().filter(|e| e.app.eq_ignore_ascii_case(name)).collect(),
    };

    let mut count = 0;
    for entry in to_remove {
        if fs::remove_file(&entry.path).is_ok() {
            count += 1;
        }
    }
    Ok(count)
}

/// 从缓存文件名中解析 app 和 version
///
/// 格式：`{app}#{version}#{...}`，按 `#` 分割取前两段。
/// 三段均为非空才返回有效结果。
fn parse_cache_filename(filename: &str) -> Option<(&str, &str)> {
    let mut parts = filename.splitn(3, '#');
    let app = parts.next()?;
    let version = parts.next()?;
    let rest = parts.next()?;
    if app.is_empty() || version.is_empty() || rest.is_empty() {
        return None;
    }
    Some((app, version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::HitConfig;

    /// 创建指向临时目录的 Session（cache 目录为 `<temp>/cache/`）
    fn make_session(root: &Path) -> Session {
        Session::with_config(HitConfig {
            root_path: Some(root.to_string_lossy().to_string()),
            ..HitConfig::default()
        })
    }

    // ── cache_filename 测试 ──────────────────────────────────────

    #[test]
    fn cache_filename_format() {
        let name = cache_filename("git", "2.43.0", "https://example.com/git.zip");
        // 格式：git#2.43.0#{7位hex}.zip
        assert!(name.starts_with("git#2.43.0#"));
        assert!(name.ends_with(".zip"));

        // sha256_7 部分应为 7 位 hex
        let sha_part = name.split('#').nth(2).unwrap();
        assert_eq!(sha_part.len(), 11); // 7 hex + ".zip" = 11
        assert!(sha_part[..7].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn cache_filename_no_extension() {
        let name = cache_filename("app", "1.0", "https://example.com/file");
        assert!(name.starts_with("app#1.0#"));
        let sha_part = name.split('#').nth(2).unwrap();
        assert_eq!(sha_part.len(), 7); // 7 hex, no extension
    }

    #[test]
    fn cache_filename_url_with_fragment() {
        // URL 含 #/dl.7z fragment — 扩展名应为 .7z（与 Scoop/Hok 一致）
        let name = cache_filename("app", "1.0", "https://example.com/file.zip#/dl.7z");
        assert!(name.ends_with(".7z"));
    }

    #[test]
    fn cache_filename_deterministic() {
        let a = cache_filename("git", "2.43.0", "https://example.com/git.zip");
        let b = cache_filename("git", "2.43.0", "https://example.com/git.zip");
        assert_eq!(a, b);
    }

    #[test]
    fn cache_filename_different_urls_produce_different_hashes() {
        let a = cache_filename("git", "2.43.0", "https://a.com/git.zip");
        let b = cache_filename("git", "2.43.0", "https://b.com/git.zip");
        assert_ne!(a, b);
    }

    // ── cache_path 测试 ─────────────────────────────────────────

    #[test]
    fn cache_path_uses_session_cache_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let path = cache_path(&session, "git", "1.0", "https://example.com/git.zip");
        assert!(path.starts_with(session.cache_path()));
        assert!(path.starts_with(tmp.path().join("cache")));
    }

    // ── cache_exists 测试 ────────────────────────────────────────

    #[test]
    fn cache_exists_false_when_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        assert!(!cache_exists(&session, "git", "1.0", "https://example.com/git.zip"));
    }

    #[test]
    fn cache_exists_true_when_present() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let path = cache_path(&session, "git", "1.0", "https://example.com/git.zip");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, b"fake content").unwrap();
        assert!(cache_exists(&session, "git", "1.0", "https://example.com/git.zip"));
    }

    // ── list_cache 测试 ─────────────────────────────────────────

    #[test]
    fn list_cache_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        // cache 目录不存在时应返回空列表
        let entries = list_cache(&session).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn list_cache_parses_entries() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let cache_dir = session.cache_path();
        fs::create_dir_all(cache_dir).unwrap();

        // 创建两个缓存文件
        let p1 = cache_path(&session, "git", "2.43.0", "https://a.com/git.zip");
        let p2 = cache_path(&session, "python", "3.12.0", "https://b.com/python.exe");
        fs::write(&p1, b"git content").unwrap();
        fs::write(&p2, b"python content here").unwrap();

        let entries = list_cache(&session).unwrap();
        assert_eq!(entries.len(), 2);
        // 按 app 排序：git 在 python 前
        assert_eq!(entries[0].app, "git");
        assert_eq!(entries[0].version, "2.43.0");
        assert_eq!(entries[0].size, 11);
        assert_eq!(entries[1].app, "python");
        assert_eq!(entries[1].version, "3.12.0");
        assert_eq!(entries[1].size, 19);
    }

    #[test]
    fn list_cache_skips_non_cache_files() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let cache_dir = session.cache_path();
        fs::create_dir_all(cache_dir).unwrap();

        // 非 # 分隔的文件应被跳过
        fs::write(cache_dir.join("random_file.txt"), b"not a cache file").unwrap();
        // 只有两段的 # 文件也应跳过
        fs::write(cache_dir.join("incomplete#file"), b"bad").unwrap();

        let entries = list_cache(&session).unwrap();
        assert!(entries.is_empty());
    }

    // ── remove_cache 测试 ────────────────────────────────────────

    #[test]
    fn remove_cache_by_app() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let cache_dir = session.cache_path();
        fs::create_dir_all(cache_dir).unwrap();

        // 创建 git 的两个版本 + python 一个版本
        let g1 = cache_path(&session, "git", "2.43.0", "https://a.com/git.zip");
        let g2 = cache_path(&session, "git", "2.42.0", "https://b.com/git.zip");
        let py = cache_path(&session, "python", "3.12.0", "https://c.com/python.exe");
        fs::write(&g1, b"1").unwrap();
        fs::write(&g2, b"2").unwrap();
        fs::write(&py, b"3").unwrap();

        let count = remove_cache(&session, Some("git")).unwrap();
        assert_eq!(count, 2);
        assert!(!g1.exists());
        assert!(!g2.exists());
        assert!(py.exists()); // python 应保留
    }

    #[test]
    fn remove_cache_all() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let cache_dir = session.cache_path();
        fs::create_dir_all(cache_dir).unwrap();

        let g1 = cache_path(&session, "git", "1.0", "https://a.com/git.zip");
        let g2 = cache_path(&session, "python", "2.0", "https://b.com/py.exe");
        fs::write(&g1, b"1").unwrap();
        fs::write(&g2, b"2").unwrap();

        let count = remove_cache(&session, None).unwrap();
        assert_eq!(count, 2);
        assert!(!g1.exists());
        assert!(!g2.exists());
    }

    #[test]
    fn remove_cache_nonexistent_no_error() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let count = remove_cache(&session, Some("nonexistent")).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn remove_cache_case_insensitive() {
        let tmp = tempfile::tempdir().unwrap();
        let session = make_session(tmp.path());
        let cache_dir = session.cache_path();
        fs::create_dir_all(cache_dir).unwrap();

        let p = cache_path(&session, "Git", "1.0", "https://a.com/git.zip");
        fs::write(&p, b"content").unwrap();

        let count = remove_cache(&session, Some("git")).unwrap();
        assert_eq!(count, 1);
        assert!(!p.exists());
    }
}
