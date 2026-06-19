//! Bucket 子模块：Git 仓库克隆、更新与索引。

pub mod git_client;

pub use git_client::{clone_bucket, CloneOptions};
