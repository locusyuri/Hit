//! Bucket 子模块：Git 仓库克隆、更新与索引。

pub mod git_client;
mod types;

pub use git_client::{clone_bucket, pull_bucket, CloneOptions};
pub use types::{list_buckets, update_all_buckets, Bucket, UpdateOutcome, UpdateResult};
