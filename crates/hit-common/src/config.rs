//! Hit 用户配置
//!
//! 管理 `~/.hit/config.json`，支持加载、保存与默认值。
//! 配置字段保持 Scoop 兼容，同时支持 Hit 扩展项（如 `no_junction`）。

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{HitError, Result};
use crate::paths;

/// 链接模式（符号链接 vs 目录连接）
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkMode {
    /// Windows 原生符号链接（需要开发者模式或管理员权限）
    #[default]
    Symlink,
    /// 目录连接 junction（无需特殊权限）
    Junction,
    /// 硬链接（文件）
    Hardlink,
}

/// Hit 用户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HitConfig {
    /// HTTP/HTTPS 代理（如 `http://127.0.0.1:7890`）
    pub proxy: Option<String>,

    /// 默认镜像源 URL（用于 Bucket 与下载加速）
    pub mirror: Option<String>,

    /// 是否启用 aria2 多线程下载（Phase 2+ 生效）
    pub aria2_enabled: bool,

    /// 是否禁用 junction 回退（兼容 Scoop 同名配置项）
    pub no_junction: bool,

    /// 链接模式（symlink / junction / hardlink）
    pub link_mode: LinkMode,

    /// 旧版本自动清理天数（0 表示不自动清理）
    pub auto_cleanup_days: u32,

    /// 健康检查间隔天数（0 表示禁用自动检查）
    pub health_check_interval_days: u32,
}

impl Default for HitConfig {
    fn default() -> Self {
        Self {
            proxy: None,
            mirror: None,
            aria2_enabled: false,
            no_junction: false,
            link_mode: LinkMode::default(),
            auto_cleanup_days: 30,
            health_check_interval_days: 7,
        }
    }
}

impl HitConfig {
    /// 配置文件的默认路径：`~/.hit/config.json`
    pub fn default_path() -> PathBuf {
        paths::root_path().join("config.json")
    }

    /// 从指定路径加载配置；文件不存在时返回默认配置
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        sonic_rs::from_str(&content)
            .map_err(|e| HitError::Config {
                message: format!("解析 {} 失败：{e}", path.display()),
            })
    }

    /// 保存到指定路径（覆盖写入）
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = sonic_rs::to_string_pretty(self)
            .map_err(|e| HitError::Config {
                message: format!("序列化配置失败：{e}"),
            })?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
