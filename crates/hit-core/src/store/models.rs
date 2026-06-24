//! 数据存储模型
//!
//! 定义 `db.json` 中的核心数据结构：
//! - `InstalledPackage`：已安装软件的完整记录
//! - `BucketInfo`：已注册 bucket 的元信息

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// 已安装软件记录（`db.json` → `packages` → 值）
///
/// 包含卸载时所需的全部信息：manifest 原文、环境变量、shim 列表等。
/// `#[serde(default)]` 保证旧版 db.json 缺失字段时使用零值默认。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InstalledPackage {
    /// 已安装版本号
    pub version: String,
    /// 来源 bucket 名称（如 `"main"`、`"extras"`）
    pub bucket: String,
    /// 安装时间（ISO 8601 UTC，如 `"2024-01-15T10:30:00Z"`）
    pub install_date: String,
    /// 安装时选择的架构（Scoop key：`"64bit"` / `"32bit"` / `"arm64"`）
    pub architecture: String,
    /// 创建的 shim 文件名列表（如 `["git.exe", "gitk.exe"]`）
    pub shims: Vec<String>,
    /// persist 项的源路径列表（如 `["etc", "data/config.ini"]`）
    pub persist_files: Vec<String>,
    /// 版本锁定标记（`true` 时 autoupdate 跳过）
    pub held: bool,
    /// 已添加到 PATH 的绝对路径列表（安装时已解析，卸载可直接移除）
    pub env_add_path: Vec<String>,
    /// 已设置的环境变量键值对（安装时已解析，卸载可直接移除）
    pub env_set: BTreeMap<String, String>,
    /// 原始 manifest JSON（卸载时用于重建 FlatManifest 以执行脚本）
    pub raw_manifest: String,
}

/// 已注册 bucket 的元信息（`db.json` → `buckets` → 值）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BucketInfo {
    /// bucket 名称
    pub name: String,
    /// Git 仓库 URL
    pub url: String,
    /// 最后更新时间（ISO 8601 UTC）
    pub last_update: String,
}

/// 生成当前时间的 ISO 8601 UTC 字符串
///
/// 格式：`"2024-01-15T10:30:00Z"`。使用 `SystemTime` 手动格式化，
/// 避免引入 chrono 依赖。精度到秒，足够人类可读用途。
pub fn now_iso8601() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // 手动分解 UTC 时间分量
    let (year, month, day, hour, min, sec) = epoch_to_utc(secs);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}Z")
}

/// Unix epoch 秒数 → (year, month, day, hour, min, sec)
fn epoch_to_utc(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let sec = secs % 60;
    let min = (secs / 60) % 60;
    let hour = (secs / 3600) % 24;

    // 天数（从 1970-01-01 起）
    let mut days = secs / 86400;

    // 计算年份
    let mut year = 1970;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    // 计算月份和日
    let leap = is_leap_year(year);
    let month_days = [
        31,
        if leap { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let mut month = 1u64;
    for &md in &month_days {
        if days < md {
            break;
        }
        days -= md;
        month += 1;
    }
    let day = days + 1;

    (year, month, day, hour, min, sec)
}

fn is_leap_year(y: u64) -> bool {
    (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400)
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installed_package_default_values() {
        let pkg = InstalledPackage::default();
        assert!(pkg.version.is_empty());
        assert!(pkg.bucket.is_empty());
        assert!(pkg.shims.is_empty());
        assert!(pkg.persist_files.is_empty());
        assert!(!pkg.held);
        assert!(pkg.env_add_path.is_empty());
        assert!(pkg.env_set.is_empty());
        assert!(pkg.raw_manifest.is_empty());
    }

    #[test]
    fn installed_package_serde_roundtrip() {
        let mut pkg = InstalledPackage {
            version: "2.45.1".into(),
            bucket: "main".into(),
            install_date: "2024-06-15T10:00:00Z".into(),
            architecture: "64bit".into(),
            shims: vec!["git.exe".into(), "gitk.exe".into()],
            persist_files: vec!["etc".into()],
            held: true,
            env_add_path: vec!["C:\\scoop\\apps\\git\\current\\bin".into()],
            ..Default::default()
        };
        pkg.env_set.insert("GIT_HOME".into(), "C:\\scoop\\apps\\git\\current".into());

        let json = sonic_rs::to_string_pretty(&pkg).unwrap();
        let deserialized: InstalledPackage = sonic_rs::from_str(&json).unwrap();

        assert_eq!(deserialized.version, "2.45.1");
        assert_eq!(deserialized.bucket, "main");
        assert_eq!(deserialized.shims.len(), 2);
        assert!(deserialized.held);
        assert_eq!(deserialized.env_set["GIT_HOME"], "C:\\scoop\\apps\\git\\current");
    }

    #[test]
    fn installed_package_missing_fields_get_defaults() {
        // 最简 JSON（只有 version）→ 其他字段用默认值
        let json = r#"{"version": "1.0", "bucket": "main"}"#;
        let pkg: InstalledPackage = sonic_rs::from_str(json).unwrap();
        assert_eq!(pkg.version, "1.0");
        assert!(pkg.shims.is_empty());
        assert!(!pkg.held);
        assert!(pkg.env_add_path.is_empty());
    }

    #[test]
    fn installed_package_unknown_fields_ignored() {
        // 包含未知字段 "future_field" → 反序列化不报错
        let json = r#"{"version": "1.0", "bucket": "main", "future_field": 42}"#;
        let pkg: InstalledPackage = sonic_rs::from_str(json).unwrap();
        assert_eq!(pkg.version, "1.0");
    }

    #[test]
    fn bucket_info_roundtrip() {
        let info = BucketInfo {
            name: "main".into(),
            url: "https://github.com/ScoopInstaller/Main".into(),
            last_update: "2024-06-15T12:00:00Z".into(),
        };
        let json = sonic_rs::to_string_pretty(&info).unwrap();
        let deserialized: BucketInfo = sonic_rs::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "main");
        assert_eq!(deserialized.url, "https://github.com/ScoopInstaller/Main");
    }

    #[test]
    fn now_iso8601_format() {
        let ts = now_iso8601();
        // 格式验证：YYYY-MM-DDTHH:MM:SSZ（20 字符）
        assert_eq!(ts.len(), 20, "ISO 8601 应为 20 字符：{ts}");
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
        assert_eq!(&ts[19..20], "Z");
    }

    #[test]
    fn epoch_to_utc_known_values() {
        // 1970-01-01T00:00:00Z
        assert_eq!(epoch_to_utc(0), (1970, 1, 1, 0, 0, 0));
        // 2024-01-01T00:00:00Z = 1704067200
        assert_eq!(epoch_to_utc(1704067200), (2024, 1, 1, 0, 0, 0));
        // 2024-06-15T09:50:45Z = 1718445045
        assert_eq!(epoch_to_utc(1718445045), (2024, 6, 15, 9, 50, 45));
    }
}
