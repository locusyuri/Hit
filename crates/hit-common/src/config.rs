//! Hit 用户配置
//!
//! 管理 `~/.hit/config.json`，支持加载、保存与默认值。
//! 配置字段保持 Scoop 兼容，同时支持 Hit 扩展项（如 `no_junction`、`root_path`）。
//!
//! **链接策略**：Hit 仅使用 **Junction**（与 Scoop 保持一致）。
//! `no_junction = true` 表示不创建 `apps/<app>/current` junction，shim 直接指向
//! 具体版本目录（通过 db.json 读取版本号）。

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{HitError, Result};
use crate::paths;

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

    /// 是否禁用 junction（兼容 Scoop 同名配置项 `NO_JUNCTION`）。
    /// 启用后不创建 `apps/<app>/current` junction，shim 从 db.json 读版本号直接指向版本目录。
    pub no_junction: bool,

    /// Hit 根目录绝对路径。未设置时回退到环境变量或默认 `~/.hit/`
    /// （详见 `paths::root_path` 的回退链）。
    pub root_path: Option<String>,

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
            root_path: None,
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
        let content = std::fs::read_to_string(path).map_err(|e| {
            HitError::io(format!("读取配置文件 {}", path.display()), e)
        })?;
        sonic_rs::from_str(&content).map_err(|e| HitError::Config {
            message: format!("解析 {} 失败：{e}", path.display()),
        })
    }

    /// 保存到指定路径（覆盖写入）
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                HitError::io(format!("创建配置目录 {}", parent.display()), e)
            })?;
        }
        let content = sonic_rs::to_string_pretty(self).map_err(|e| HitError::Config {
            message: format!("序列化配置失败：{e}"),
        })?;
        std::fs::write(path, content).map_err(|e| {
            HitError::io(format!("写入配置文件 {}", path.display()), e)
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_roundtrip_preserves_fields() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        let cfg = HitConfig {
            proxy: Some("http://127.0.0.1:7890".into()),
            mirror: Some("tuna".into()),
            aria2_enabled: true,
            no_junction: true,
            root_path: Some("D:\\hit".into()),
            auto_cleanup_days: 14,
            health_check_interval_days: 3,
        };
        cfg.save(&path).unwrap();
        let loaded = HitConfig::load(&path).unwrap();
        assert_eq!(loaded.proxy, cfg.proxy);
        assert_eq!(loaded.mirror, cfg.mirror);
        assert_eq!(loaded.aria2_enabled, cfg.aria2_enabled);
        assert_eq!(loaded.no_junction, cfg.no_junction);
        assert_eq!(loaded.root_path, cfg.root_path);
        assert_eq!(loaded.auto_cleanup_days, cfg.auto_cleanup_days);
        assert_eq!(loaded.health_check_interval_days, cfg.health_check_interval_days);
    }

    #[test]
    fn config_load_missing_returns_default() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");
        let cfg = HitConfig::load(&path).unwrap();
        assert_eq!(cfg.proxy, None);
        assert!(!cfg.no_junction);
        assert_eq!(cfg.auto_cleanup_days, 30);
    }

    #[test]
    fn config_deserialize_ignores_unknown_fields() {
        // 未来新增字段不应破坏旧配置文件
        let json = r#"{"proxy": null, "link_mode": "symlink", "some_future_field": 42}"#;
        let cfg: HitConfig = sonic_rs::from_str(json).unwrap();
        assert_eq!(cfg.proxy, None);
    }
}
