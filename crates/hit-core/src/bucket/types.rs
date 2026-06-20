//! Bucket 类型定义与列表管理
//!
//! `Bucket` 结构体表示一个本地 bucket 目录，提供 remote URL 读取与
//! manifest 计数等查询方法。`list_buckets` 枚举本地 bucket，
//! `update_all_buckets` 编排批量更新。

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

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
    /// 从目录名和路径构造
    pub fn new(name: impl Into<String>, path: PathBuf) -> Self {
        Self {
            name: name.into(),
            path,
        }
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
            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
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
}
