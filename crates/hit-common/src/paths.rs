//! Scoop 兼容路径计算
//!
//! 所有路径默认基于 `~/.hit/`（即 `%USERPROFILE%\.hit`），与 Scoop 的 `~/scoop/`
//! 布局保持一致，便于迁移与兼容。

use std::path::PathBuf;

/// Hit 根目录：`~/.hit/`（如不存在则返回默认路径，不自动创建）
pub fn root_path() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".hit")
}

/// 下载缓存目录：`~/.hit/cache/`
pub fn cache_path() -> PathBuf {
    root_path().join("cache")
}

/// 软件安装根目录：`~/.hit/apps/`
pub fn apps_path() -> PathBuf {
    root_path().join("apps")
}

/// 特定软件的安装目录：`~/.hit/apps/<app>/`
pub fn app_path(app: &str) -> PathBuf {
    apps_path().join(app)
}

/// 特定软件特定版本的安装目录：`~/.hit/apps/<app>/<version>/`
pub fn app_version_path(app: &str, version: &str) -> PathBuf {
    app_path(app).join(version)
}

/// `current` 符号链接路径：`~/.hit/apps/<app>/current`
pub fn app_current_path(app: &str) -> PathBuf {
    app_path(app).join("current")
}

/// Shim 目录：`~/.hit/shims/`（需加入 PATH）
pub fn shims_path() -> PathBuf {
    root_path().join("shims")
}

/// 持久化数据目录：`~/.hit/persist/`
pub fn persist_path() -> PathBuf {
    root_path().join("persist")
}

/// 特定软件的持久化目录：`~/.hit/persist/<app>/`
pub fn app_persist_path(app: &str) -> PathBuf {
    persist_path().join(app)
}

/// Bucket 仓库目录：`~/.hit/buckets/`
pub fn buckets_path() -> PathBuf {
    root_path().join("buckets")
}

/// 特定 Bucket 的目录：`~/.hit/buckets/<name>/`
pub fn bucket_path(name: &str) -> PathBuf {
    buckets_path().join(name)
}

/// 日志目录：`~/.hit/logs/`
pub fn logs_path() -> PathBuf {
    root_path().join("logs")
}
