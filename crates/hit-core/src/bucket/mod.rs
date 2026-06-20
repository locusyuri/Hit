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
