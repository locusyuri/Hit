//! Bucket 类型定义与列表管理
//!
//! `Bucket` 结构体表示一个本地 bucket 目录，提供 remote URL 读取与
//! manifest 计数等查询方法。`list_buckets` 枚举本地 bucket，
//! `update_all_buckets` 编排批量更新。

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use serde::{Deserialize, Serialize};

use hit_common::error::{HitError, Result};
use hit_common::event::Event;
use hit_common::Session;

use crate::bucket::git_client;

/// 本地 Bucket 信息
#[derive(Debug, Clone)]
pub struct Bucket {
    /// bucket 名称（目录名）
    pub name: String,
    /// 本地路径 `<buckets_path>/<name>/`
    pub path: PathBuf,
    /// bucket.json 元数据（文件不存在或解析失败时为 None）
    pub metadata: Option<BucketMetadata>,
}

/// Bucket 元数据（对应 bucket.json）
///
/// Hit 扩展概念：bucket 仓库根目录下的 `bucket.json` 文件，
/// 描述 bucket 自身信息。Scoop 无此标准，不影响兼容性。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BucketMetadata {
    /// Bucket 显示名称
    pub name: String,
    /// Bucket 描述
    pub description: String,
    /// Bucket 维护者
    pub maintainer: Option<String>,
    /// Bucket 主页 URL
    pub homepage: Option<String>,
}

/// 单个 bucket 更新结果
#[derive(Debug)]
pub struct UpdateResult {
    /// bucket 名称
    pub name: String,
    /// 更新结果
    pub outcome: UpdateOutcome,
}

/// Bucket 更新结果枚举
#[derive(Debug)]
pub enum UpdateOutcome {
    /// 更新成功
    Updated,
    /// 无远程 URL（跳过）
    Skipped,
    /// 更新失败（携带错误描述）
    Failed(String),
}

impl Bucket {
    /// 从目录名和路径构造（自动尝试加载 `bucket.json` 元数据）
    pub fn new(name: impl Into<String>, path: PathBuf) -> Self {
        let metadata = Self::load_metadata(&path);
        Self {
            name: name.into(),
            path,
            metadata,
        }
    }

    /// 尝试从 `<path>/bucket.json` 加载元数据
    fn load_metadata(path: &Path) -> Option<BucketMetadata> {
        let file = path.join("bucket.json");
        let content = std::fs::read_to_string(&file).ok()?;
        sonic_rs::from_str(&content).ok()
    }

    /// 读取 origin remote URL（通过 gix）
    ///
    /// 非 git 仓库或无 origin 时返回 None。
    pub fn remote_url(&self) -> Option<String> {
        let repo = gix::open(&self.path).ok()?;
        let remote = repo.find_remote("origin").ok()?;
        remote
            .url(gix::remote::Direction::Fetch)
            .map(|url| url.to_bstring().to_string())
    }

    /// 统计 bucket 目录下 .json manifest 文件数量
    ///
    /// 支持 Scoop v0.3.0+ 子目录布局（`bucket/` 子目录下的 .json 也被计数）。
    pub fn manifest_count(&self) -> Result<usize> {
        let mut count = 0;

        // 根目录下的 .json 文件
        let entries = std::fs::read_dir(&self.path)
            .map_err(|e| HitError::io("读取 bucket 目录", e))?;
        for entry in entries {
            let entry = entry.map_err(|e| HitError::io("读取目录项", e))?;
            let path = entry.path();
            if path.is_file()
                && path.extension().is_some_and(|ext| ext == "json")
                && path.file_name().is_some_and(|f| f != "bucket.json")
            {
                count += 1;
            }
        }

        // bucket/ 子目录下的 .json 文件（Scoop v0.3.0+ 布局）
        let sub_dir = self.path.join("bucket");
        if sub_dir.is_dir() {
            let sub_entries = std::fs::read_dir(&sub_dir)
                .map_err(|e| HitError::io("读取 bucket/bucket 目录", e))?;
            for entry in sub_entries {
                let entry = entry.map_err(|e| HitError::io("读取目录项", e))?;
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

/// 枚举所有本地 Bucket（按名称排序）
///
/// `buckets_path()` 目录不存在时返回空 Vec。
pub fn list_buckets(session: &Session) -> Result<Vec<Bucket>> {
    let buckets_dir = session.buckets_path();
    if !buckets_dir.exists() {
        return Ok(Vec::new());
    }

    let mut buckets = Vec::new();
    let entries = std::fs::read_dir(buckets_dir)
        .map_err(|e| HitError::io("读取 buckets 目录", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| HitError::io("读取目录项", e))?;
        let path = entry.path();
        if path.is_dir() {
            let name = entry
                .file_name()
                .to_string_lossy()
                .into_owned();
            buckets.push(Bucket::new(name, path));
        }
    }

    buckets.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(buckets)
}

/// 更新所有本地 Bucket
///
/// 遍历本地 bucket，逐个 pull。跳过无 remote URL 的 bucket。
/// 每个 bucket 完成后 emit `BucketUpdateProgress` 事件。
pub fn update_all_buckets(
    session: &Session,
    should_interrupt: &AtomicBool,
) -> Result<Vec<UpdateResult>> {
    let buckets = list_buckets(session)?;
    let total = buckets.len();
    let mut results = Vec::with_capacity(total);

    for (i, bucket) in buckets.iter().enumerate() {
        if should_interrupt.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        session.emit(Event::BucketUpdateProgress {
            bucket: bucket.name.clone(),
            processed: i + 1,
            total,
        });

        let outcome = if bucket.remote_url().is_none() {
            session.emit(Event::LogInfo {
                message: format!("跳过 '{}'（无远程 URL）", bucket.name),
            });
            UpdateOutcome::Skipped
        } else {
            match git_client::pull_bucket(session, &bucket.name, should_interrupt) {
                Ok(_) => {
                    session.emit(Event::LogInfo {
                        message: format!("'{}' 更新完成", bucket.name),
                    });
                    UpdateOutcome::Updated
                }
                Err(e) => {
                    let msg = e.to_string();
                    session.emit(Event::LogWarn {
                        message: format!("'{}' 更新失败：{msg}", bucket.name),
                    });
                    UpdateOutcome::Failed(msg)
                }
            }
        };

        results.push(UpdateResult {
            name: bucket.name.clone(),
            outcome,
        });
    }

    Ok(results)
}

/// 已知 Scoop 官方 Bucket（名称 → Git URL）
///
/// 参考 Scoop `buckets.json`（10 个）和 Hok `BUILTIN_BUCKET_LIST`（7 个），
/// Hit 选取最常用的 3 个作为默认推荐。
pub static KNOWN_BUCKETS: &[(&str, &str)] = &[
    ("main", "https://github.com/ScoopInstaller/Main"),
    ("extras", "https://github.com/ScoopInstaller/Extras"),
    ("versions", "https://github.com/ScoopInstaller/Versions"),
];

/// 查询已知 bucket URL（按名称）
pub fn resolve_known_bucket(name: &str) -> Option<&'static str> {
    KNOWN_BUCKETS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, url)| *url)
}

/// 返回所有已知 bucket 列表
pub fn known_buckets() -> &'static [(&'static str, &'static str)] {
    KNOWN_BUCKETS
}

/// 单个 bucket 添加结果
#[derive(Debug)]
pub struct AddResult {
    /// bucket 名称
    pub name: String,
    /// 添加结果
    pub outcome: AddOutcome,
}

/// Bucket 添加结果枚举
#[derive(Debug)]
pub enum AddOutcome {
    /// 添加成功
    Added,
    /// 已存在（跳过）
    Skipped,
    /// 添加失败（携带错误描述）
    Failed(String),
}

/// 添加所有默认官方 Bucket（跳过已存在的）
pub fn add_default_buckets(
    session: &Session,
    should_interrupt: &AtomicBool,
) -> Result<Vec<AddResult>> {
    let buckets_dir = session.buckets_path();
    let mut results = Vec::with_capacity(KNOWN_BUCKETS.len());

    for (name, url) in KNOWN_BUCKETS {
        if should_interrupt.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        let target = buckets_dir.join(name);
        if target.exists() {
            session.emit(Event::LogInfo {
                message: format!("跳过 '{name}'（已存在）"),
            });
            results.push(AddResult {
                name: name.to_string(),
                outcome: AddOutcome::Skipped,
            });
            continue;
        }

        session.emit(Event::LogInfo {
            message: format!("正在添加 bucket '{name}'..."),
        });

        match git_client::clone_bucket(
            session,
            name,
            url,
            &git_client::CloneOptions::default(),
            should_interrupt,
        ) {
            Ok(_) => {
                results.push(AddResult {
                    name: name.to_string(),
                    outcome: AddOutcome::Added,
                });
            }
            Err(e) => {
                let msg = e.to_string();
                session.emit(Event::LogWarn {
                    message: format!("添加 '{name}' 失败：{msg}"),
                });
                results.push(AddResult {
                    name: name.to_string(),
                    outcome: AddOutcome::Failed(msg),
                });
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;

    fn test_session(dir: &std::path::Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    #[test]
    fn list_buckets_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets")).unwrap();
        let session = test_session(dir.path());
        let buckets = list_buckets(&session).unwrap();
        assert!(buckets.is_empty());
    }

    #[test]
    fn list_buckets_nonexistent_dir() {
        let dir = tempfile::tempdir().unwrap();
        // 不创建 buckets 目录
        let session = test_session(dir.path());
        let buckets = list_buckets(&session).unwrap();
        assert!(buckets.is_empty());
    }

    #[test]
    fn list_buckets_sorts_by_name() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        std::fs::create_dir_all(buckets_dir.join("extras")).unwrap();
        std::fs::create_dir_all(buckets_dir.join("main")).unwrap();
        std::fs::create_dir_all(buckets_dir.join("alpha")).unwrap();

        let session = test_session(dir.path());
        let buckets = list_buckets(&session).unwrap();

        assert_eq!(buckets.len(), 3);
        assert_eq!(buckets[0].name, "alpha");
        assert_eq!(buckets[1].name, "extras");
        assert_eq!(buckets[2].name, "main");
    }

    #[test]
    fn list_buckets_ignores_files() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        std::fs::create_dir_all(&buckets_dir).unwrap();
        std::fs::create_dir_all(buckets_dir.join("real-bucket")).unwrap();
        std::fs::write(buckets_dir.join("notes.txt"), "ignore me").unwrap();

        let session = test_session(dir.path());
        let buckets = list_buckets(&session).unwrap();

        assert_eq!(buckets.len(), 1);
        assert_eq!(buckets[0].name, "real-bucket");
    }

    #[test]
    fn bucket_manifest_count_root_json() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("my-bucket");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        std::fs::write(bucket_dir.join("git.json"), "{}").unwrap();
        std::fs::write(bucket_dir.join("python.json"), "{}").unwrap();
        std::fs::write(bucket_dir.join("readme.md"), "# readme").unwrap();

        let bucket = Bucket::new("my-bucket", bucket_dir);
        assert_eq!(bucket.manifest_count().unwrap(), 2);
    }

    #[test]
    fn bucket_manifest_count_subdir_layout() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("my-bucket");
        let sub_dir = bucket_dir.join("bucket");
        std::fs::create_dir_all(&sub_dir).unwrap();
        std::fs::write(bucket_dir.join("root.json"), "{}").unwrap();
        std::fs::write(sub_dir.join("sub1.json"), "{}").unwrap();
        std::fs::write(sub_dir.join("sub2.json"), "{}").unwrap();

        let bucket = Bucket::new("my-bucket", bucket_dir);
        assert_eq!(bucket.manifest_count().unwrap(), 3);
    }

    #[test]
    fn bucket_remote_url_non_git_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("not-a-repo");
        std::fs::create_dir_all(&bucket_dir).unwrap();

        let bucket = Bucket::new("not-a-repo", bucket_dir);
        assert!(bucket.remote_url().is_none());
    }

    #[test]
    #[ignore = "需要网络访问"]
    fn bucket_remote_url_from_clone() {
        let dir = tempfile::tempdir().unwrap();
        let config = HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let interrupt = AtomicBool::new(false);

        let url = "https://github.com/ScoopInstaller/Main.git";
        git_client::clone_bucket(
            &session,
            "main",
            url,
            &git_client::CloneOptions::default(),
            &interrupt,
        )
        .expect("克隆应成功");

        let buckets = list_buckets(&session).unwrap();
        assert_eq!(buckets.len(), 1);
        let remote = buckets[0].remote_url();
        assert!(remote.is_some(), "应能读取 remote URL");
        assert!(
            remote.unwrap().contains("ScoopInstaller/Main"),
            "remote URL 应包含仓库名"
        );
    }

    // ── BucketMetadata 测试 ──

    #[test]
    fn bucket_metadata_parse_valid() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("test-bucket");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        std::fs::write(
            bucket_dir.join("bucket.json"),
            r#"{"name":"Test","description":"A test bucket","maintainer":"alice","homepage":"https://example.com"}"#,
        )
        .unwrap();

        let bucket = Bucket::new("test-bucket", bucket_dir);
        let meta = bucket.metadata.expect("应成功解析 metadata");
        assert_eq!(meta.name, "Test");
        assert_eq!(meta.description, "A test bucket");
        assert_eq!(meta.maintainer.as_deref(), Some("alice"));
        assert_eq!(meta.homepage.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn bucket_metadata_missing_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("no-meta");
        std::fs::create_dir_all(&bucket_dir).unwrap();

        let bucket = Bucket::new("no-meta", bucket_dir);
        assert!(bucket.metadata.is_none());
    }

    #[test]
    fn bucket_metadata_malformed_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("bad-meta");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        std::fs::write(bucket_dir.join("bucket.json"), "not json at all").unwrap();

        let bucket = Bucket::new("bad-meta", bucket_dir);
        assert!(bucket.metadata.is_none());
    }

    #[test]
    fn bucket_metadata_partial_fields() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("partial");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        // 只有必填字段（name/description 有默认空字符串），无可选字段
        std::fs::write(bucket_dir.join("bucket.json"), r#"{"name":"P"}"#).unwrap();

        let bucket = Bucket::new("partial", bucket_dir);
        let meta = bucket.metadata.expect("应成功解析");
        assert_eq!(meta.name, "P");
        assert_eq!(meta.description, "");
        assert!(meta.maintainer.is_none());
    }

    #[test]
    fn manifest_count_excludes_bucket_json() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("my-bucket");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        std::fs::write(bucket_dir.join("bucket.json"), r#"{"name":"x"}"#).unwrap();
        std::fs::write(bucket_dir.join("git.json"), "{}").unwrap();
        std::fs::write(bucket_dir.join("python.json"), "{}").unwrap();

        let bucket = Bucket::new("my-bucket", bucket_dir);
        assert_eq!(bucket.manifest_count().unwrap(), 2);
    }

    // ── KNOWN_BUCKETS 测试 ──

    #[test]
    fn resolve_known_bucket_main() {
        let url = resolve_known_bucket("main");
        assert_eq!(url, Some("https://github.com/ScoopInstaller/Main"));
    }

    #[test]
    fn resolve_known_bucket_extras() {
        let url = resolve_known_bucket("extras");
        assert_eq!(url, Some("https://github.com/ScoopInstaller/Extras"));
    }

    #[test]
    fn resolve_known_bucket_unknown() {
        assert_eq!(resolve_known_bucket("nonexistent"), None);
    }

    #[test]
    fn known_buckets_returns_three() {
        let list = known_buckets();
        assert_eq!(list.len(), 3);
        assert!(list.iter().any(|(n, _)| *n == "main"));
        assert!(list.iter().any(|(n, _)| *n == "extras"));
        assert!(list.iter().any(|(n, _)| *n == "versions"));
    }

    #[test]
    fn add_default_buckets_skips_existing() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        // 预先创建所有已知 bucket 目录（全部跳过，无需网络）
        for (name, _) in KNOWN_BUCKETS {
            std::fs::create_dir_all(buckets_dir.join(name)).unwrap();
        }

        let session = test_session(dir.path());
        let interrupt = AtomicBool::new(false);

        let results = add_default_buckets(&session, &interrupt).unwrap();
        assert_eq!(results.len(), KNOWN_BUCKETS.len());
        for result in &results {
            assert!(
                matches!(result.outcome, AddOutcome::Skipped),
                "'{}' 应被跳过",
                result.name
            );
        }
    }
}
