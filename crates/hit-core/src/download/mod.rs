//! 下载与缓存模块
//!
//! - `http`：基于 reqwest blocking client 的 HTTP 下载，支持 proxy 配置和进度事件上报
//! - `cache`：Scoop/Hok 兼容的缓存管理（路径计算、命中检查、列举与清理）

pub mod cache;
pub mod http;

pub use cache::{
    cache_exists, cache_path, download_to_cache, list_cache, remove_cache, CacheEntry,
};
pub use http::{build_client, download_file};
