//! Schema 迁移框架
//!
//! 管理 `db.json` 的版本检测与前向迁移。迁移操作在 `sonic_rs::Value` 层面执行，
//! 不依赖类型化结构体，从而支持增删字段而不需中间类型。
//!
//! 当前为 v1（首个正式版本），迁移函数为未来占位。

use hit_common::error::Result;
use sonic_rs::JsonValueMutTrait;
use sonic_rs::JsonValueTrait;
use sonic_rs::Value;

/// 当前 schema 版本
pub const CURRENT_VERSION: u32 = 1;

/// db.json 中的版本字段名
const VERSION_KEY: &str = "version";

/// 检测 db.json 的 schema 版本
///
/// - 存在 `version` 字段且为 u64 → 返回该值
/// - 缺失或类型不匹配 → 视为 v1（首个版本）
pub fn detect_version(raw: &Value) -> u32 {
    raw.get(VERSION_KEY)
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(1)
}

/// 执行前向迁移，将 raw Value 就地升级到 `CURRENT_VERSION`
///
/// - 旧版本：按顺序执行迁移函数
/// - 当前版本：no-op
/// - 未来版本：不降级，仅 warn 日志
pub fn migrate(raw: &mut Value) -> Result<u32> {
    let version = detect_version(raw);

    match version.cmp(&CURRENT_VERSION) {
        std::cmp::Ordering::Less => {
            let mut v = version;
            // 未来在此添加迁移步骤：
            // if v < 2 { v = migrate_v1_to_v2(raw); }
            // if v < 3 { v = migrate_v2_to_v3(raw); }
            let _ = &mut v; // 消除 unused_mut 警告（当前无实际迁移）

            // 写入新版本号
            if let Some(obj) = raw.as_object_mut() {
                obj.insert(&VERSION_KEY, v);
            }
            Ok(v)
        }
        std::cmp::Ordering::Equal => {
            // 确保 version 字段存在（缺失时补写）
            if raw.get(VERSION_KEY).is_none()
                && let Some(obj) = raw.as_object_mut()
            {
                obj.insert(&VERSION_KEY, version);
            }
            Ok(version)
        }
        std::cmp::Ordering::Greater => {
            tracing::warn!(
                db_version = version,
                current = CURRENT_VERSION,
                "db.json 版本高于当前 schema，部分字段可能被忽略"
            );
            Ok(version)
        }
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use sonic_rs::json;

    #[test]
    fn detect_version_missing_returns_1() {
        let raw: Value = sonic_rs::from_str(r#"{"packages": {}}"#).unwrap();
        assert_eq!(detect_version(&raw), 1);
    }

    #[test]
    fn detect_version_present() {
        let raw = json!({"version": 3, "packages": {}});
        assert_eq!(detect_version(&raw), 3);
    }

    #[test]
    fn detect_version_wrong_type_returns_1() {
        let raw = json!({"version": "not_a_number"});
        assert_eq!(detect_version(&raw), 1);
    }

    #[test]
    fn migrate_v1_noop() {
        let mut raw = json!({"version": 1, "packages": {"git": {"version": "1.0"}}});
        let result = migrate(&mut raw).unwrap();
        assert_eq!(result, CURRENT_VERSION);
    }

    #[test]
    fn migrate_missing_version_adds_field() {
        let mut raw = json!({"packages": {}});
        let result = migrate(&mut raw).unwrap();
        assert_eq!(result, CURRENT_VERSION);
        // version 字段应被写入
        assert_eq!(raw.get("version").unwrap().as_u64().unwrap(), CURRENT_VERSION as u64);
    }

    #[test]
    fn migrate_future_version_does_not_downgrade() {
        let mut raw = json!({"version": 99, "packages": {}});
        let result = migrate(&mut raw).unwrap();
        assert_eq!(result, 99);
        // version 不应被修改
        assert_eq!(raw.get("version").unwrap().as_u64().unwrap(), 99);
    }
}
