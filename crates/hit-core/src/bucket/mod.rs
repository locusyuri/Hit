//! Bucket 子模块：Git 仓库克隆、更新与索引。

pub mod git_client;
pub mod index;
mod types;

pub use git_client::{clone_bucket, pull_bucket, CloneOptions};
pub use index::{build_index, PackageSummary, SoftwareIndex};
pub use types::{
    add_default_buckets, known_buckets, list_buckets, resolve_known_bucket, update_all_buckets,
    AddOutcome, AddResult, Bucket, BucketMetadata, UpdateOutcome, UpdateResult,
};

use std::path::PathBuf;

/// 解析 manifest 文件路径，兼容 Scoop v0.3.0+ 子目录布局。
///
/// 查找顺序：
/// 1. `buckets/<bucket>/bucket/<app>.json`（Scoop v0.3.0+ 布局，manifest 在 bucket/ 子目录）
/// 2. `buckets/<bucket>/<app>.json`（旧布局，manifest 在 bucket 根目录）
///
/// 返回第一个找到的路径；都不存在时返回路径 1（让后续 read 报 "文件不存在"）。
pub fn manifest_path(buckets_dir: &std::path::Path, bucket: &str, app: &str) -> PathBuf {
    let filename = format!("{app}.json");
    let subdir_path = buckets_dir.join(bucket).join("bucket").join(&filename);
    if subdir_path.exists() {
        return subdir_path;
    }
    let root_path = buckets_dir.join(bucket).join(&filename);
    if root_path.exists() {
        return root_path;
    }
    // 都不存在，返回子目录路径（后续 read 会报错，比返回错误路径好）
    subdir_path
}
