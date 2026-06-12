//! Hit 共享测试 fixture
//!
//! 为 hit-core / hit-cli 的集成测试提供 dev-dependency 工具库：
//! - `mock_config()`：构造测试用 `HitConfig`（确定值、零副作用）
//! - `sample_manifest()`：构造样例 Manifest JSON 字符串（Scoop 兼容格式）
//! - `temp_scoop_root()`：创建临时 Scoop 目录结构（`apps/` / `shims/` / `cache/` / `persist/` / `buckets/`）

use std::path::{Path, PathBuf};

use tempfile::TempDir;

use hit_common::{HitConfig, LinkMode};

/// 构造测试用 `HitConfig`
///
/// 所有字段均为确定值（无 None），避免测试分支抖动；关闭网络相关特性
/// （proxy/mirror/aria2）确保测试环境零外部依赖。
pub fn mock_config() -> HitConfig {
    HitConfig {
        proxy: None,
        mirror: None,
        aria2_enabled: false,
        no_junction: false,
        link_mode: LinkMode::Symlink,
        auto_cleanup_days: 0,
        health_check_interval_days: 0,
    }
}

/// 样例 Manifest JSON（参考 `ref/Main/bucket/git.json` 的最小可解析结构）
///
/// 包含 Scoop 标准字段（version / description / homepage / license / architecture /
/// bin / shortcuts / persist / checkver / autoupdate），可用于 hit-core/manifest
/// 的解析/校验/变量替换单元测试。
pub fn sample_manifest() -> &'static str {
    r#"{
    "version": "2.45.1",
    "description": "Distributed version control system",
    "homepage": "https://git-scm.com",
    "license": "GPL-2.0-only",
    "architecture": {
        "64bit": {
            "url": "https://example.com/git-2.45.1-64-bit.7z",
            "hash": "0000000000000000000000000000000000000000000000000000000000000000"
        },
        "32bit": {
            "url": "https://example.com/git-2.45.1-32-bit.7z",
            "hash": "1111111111111111111111111111111111111111111111111111111111111111"
        }
    },
    "bin": [
        "cmd/git.exe",
        "cmd/gitk.exe",
        "cmd/git-gui.exe"
    ],
    "shortcuts": [
        ["cmd/gitk.exe", "Git Gitk"],
        ["cmd/git-gui.exe", "Git GUI"]
    ],
    "persist": ["etc"],
    "checkver": {
        "github": "https://github.com/git-for-windows/git"
    },
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://example.com/git-$version-64-bit.7z"
            },
            "32bit": {
                "url": "https://example.com/git-$version-32-bit.7z"
            }
        }
    }
}"#
}

/// 创建临时 Scoop 目录结构（返回 `TempDir`，离开作用域自动清理）
///
/// 目录布局：
/// ```text
/// <temp>/
/// ├── apps/
/// ├── shims/
/// ├── cache/
/// ├── persist/
/// ├── buckets/
/// ├── logs/
/// └── config.json (可选，写入默认 HitConfig 序列化结果)
/// ```
///
/// 返回 `(TempDir, PathBuf)`：`TempDir` 持有生命周期，`PathBuf` 是根目录路径。
pub fn temp_scoop_root() -> std::io::Result<(TempDir, PathBuf)> {
    let dir = TempDir::new()?;
    let root = dir.path().to_path_buf();
    create_scoop_layout(&root)?;
    Ok((dir, root))
}

/// 在指定路径创建 Scoop 目录布局（不写 config.json）
pub fn create_scoop_layout(root: &Path) -> std::io::Result<()> {
    for sub in ["apps", "shims", "cache", "persist", "buckets", "logs"] {
        std::fs::create_dir_all(root.join(sub))?;
    }
    Ok(())
}

/// 在指定根目录写入默认 `config.json`（使用 `mock_config()`）
pub fn write_mock_config(root: &Path) -> hit_common::Result<()> {
    let path = root.join("config.json");
    mock_config().save(&path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sonic_rs::JsonValueTrait;

    #[test]
    fn mock_config_is_deterministic() {
        let a = mock_config();
        let b = mock_config();
        assert_eq!(a.link_mode, b.link_mode);
        assert_eq!(a.no_junction, b.no_junction);
        assert!(a.proxy.is_none());
    }

    #[test]
    fn sample_manifest_is_valid_json() {
        let value: sonic_rs::Value = sonic_rs::from_str(sample_manifest()).unwrap();
        assert_eq!(value["version"].as_str().unwrap(), "2.45.1");
        assert!(value["architecture"]["64bit"]["url"].as_str().is_some());
    }

    #[test]
    fn temp_scoop_root_creates_layout() {
        let (_dir, root) = temp_scoop_root().unwrap();
        for sub in ["apps", "shims", "cache", "persist", "buckets", "logs"] {
            assert!(root.join(sub).is_dir(), "缺少子目录 {sub}");
        }
    }

    #[test]
    fn write_mock_config_roundtrip() {
        let (_dir, root) = temp_scoop_root().unwrap();
        write_mock_config(&root).unwrap();
        let loaded = HitConfig::load(&root.join("config.json")).unwrap();
        assert_eq!(loaded.link_mode, LinkMode::Symlink);
        assert!(!loaded.no_junction);
    }
}
