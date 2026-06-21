//! Windows 注册表操作
//!
//! 封装 `winreg` crate，提供对以下注册表路径的读写：
//! - `HKCU\Environment`：用户环境变量（PATH 等）
//! - `HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall`：已安装软件检测
//!
//! 参考 Hok `internal/env.rs` 的 `get`/`set` 函数与 Scoop `Get-EnvVar`/`Set-EnvVar`。

use std::ffi::OsString;
use std::path::PathBuf;

use hit_common::error::{HitError, Result};
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, REG_EXPAND_SZ};
use winreg::RegKey;

/// 从 `HKCU\Environment` 读取环境变量值（不展开 `%VAR%`）
///
/// 变量不存在时返回空 `OsString`。
pub fn get_env_value(name: &str) -> Result<OsString> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey_with_flags("Environment", KEY_READ) {
        Ok(env_key) => match env_key.get_value::<OsString, _>(name) {
            Ok(val) => Ok(val),
            Err(_) => Ok(OsString::new()),
        },
        Err(_) => Ok(OsString::new()),
    }
}

/// 写入 `HKCU\Environment` 环境变量值
///
/// - 若 `value` 含 `%` 字符，使用 `REG_EXPAND_SZ` 类型（与 Scoop `Set-EnvVar` 一致）
/// - 若 `value` 为 `None`，删除该变量
/// - 否则使用 `REG_SZ` 类型
pub fn set_env_value(name: &str, value: Option<&OsString>) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env_key, _) =
        hkcu.create_subkey("Environment")
            .map_err(|e| HitError::Io {
                context: "创建注册表键 HKCU\\Environment".to_string(),
                source: e,
            })?;

    match value {
        Some(val) => {
            let s = val.to_string_lossy();
            if s.contains('%') {
                let rv = winreg::RegValue {
                    bytes: to_utf16_bytes(&s),
                    vtype: REG_EXPAND_SZ,
                };
                env_key.set_raw_value(name, &rv).map_err(|e| HitError::Io {
                    context: format!("写入注册表 REG_EXPAND_SZ: {name}"),
                    source: e,
                })?;
            } else {
                env_key
                    .set_value::<String, _>(name, &s.into_owned())
                    .map_err(|e| HitError::Io {
                        context: format!("写入注册表 REG_SZ: {name}"),
                        source: e,
                    })?;
            }
        }
        None => {
            let _ = env_key.delete_value(name);
        }
    }

    Ok(())
}

/// 读取 PATH 风格的环境变量并拆分为路径列表
pub fn get_path_entries(name: &str) -> Result<Vec<PathBuf>> {
    let val = get_env_value(name)?;
    if val.is_empty() {
        return Ok(Vec::new());
    }
    Ok(std::env::split_paths(&val).collect())
}

/// 写入路径列表到 PATH 风格的环境变量
///
/// 将路径列表用 `;` 拼接后写入注册表。
pub fn set_path_entries(name: &str, entries: &[PathBuf]) -> Result<()> {
    let joined = std::env::join_paths(entries).map_err(|e| HitError::Io {
        context: "拼接路径列表失败".into(),
        source: std::io::Error::other(e.to_string()),
    })?;
    if joined.is_empty() {
        set_env_value(name, None)
    } else {
        set_env_value(name, Some(&joined))
    }
}

/// 检查 `HKCU\...\Uninstall` 下是否存在指定软件的安装记录
///
/// 搜索 `HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\*` 子键，
/// 匹配 `DisplayName` 字段（不区分大小写前缀匹配）。
pub fn is_installed_via_registry(display_name: &str) -> Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let uninstall_path = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
    let uninstall_key = match hkcu.open_subkey_with_flags(uninstall_path, KEY_READ) {
        Ok(k) => k,
        Err(_) => return Ok(false),
    };

    let target = display_name.to_lowercase();
    for subkey_result in uninstall_key.enum_keys() {
        let subkey_name = match subkey_result {
            Ok(name) => name,
            Err(_) => continue,
        };
        if let Ok(sub_key) = uninstall_key.open_subkey_with_flags(&subkey_name, KEY_READ)
            && let Ok(name) = sub_key.get_value::<String, _>("DisplayName")
            && name.to_lowercase().starts_with(&target)
        {
            return Ok(true);
        }
    }

    Ok(false)
}

/// 将字符串转为 UTF-16 LE 字节（含 null 终止符），用于 REG_EXPAND_SZ
fn to_utf16_bytes(s: &str) -> Vec<u8> {
    let wide: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let mut bytes = Vec::with_capacity(wide.len() * 2);
    for w in wide {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    bytes
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use winreg::enums::KEY_WRITE;

    fn cleanup_key(key: &str) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(env_key) = hkcu.open_subkey_with_flags("Environment", KEY_WRITE) {
            let _ = env_key.delete_value(key);
        }
    }

    #[test]
    fn get_env_value_missing_key() {
        let result = get_env_value("__hit_definitely_does_not_exist_12345__");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn set_get_env_value_roundtrip() {
        let key = "__hit_test_roundtrip__";
        let val = OsString::from("hit_test_value_42");
        set_env_value(key, Some(&val)).unwrap();
        let read = get_env_value(key).unwrap();
        assert_eq!(read, val);
        cleanup_key(key);
    }

    #[test]
    fn set_env_value_expand_string() {
        let key = "__hit_test_expand__";
        let val = OsString::from("%USERPROFILE%\\bin");
        set_env_value(key, Some(&val)).unwrap();
        let read = get_env_value(key).unwrap();
        assert_eq!(read, val);
        cleanup_key(key);
    }

    #[test]
    fn set_env_value_delete() {
        let key = "__hit_test_delete__";
        let val = OsString::from("to_delete");
        set_env_value(key, Some(&val)).unwrap();
        set_env_value(key, None).unwrap();
        let read = get_env_value(key).unwrap();
        assert!(read.is_empty());
    }

    #[test]
    fn is_installed_via_registry_not_found() {
        let result = is_installed_via_registry("__hit_nonexistent_software_xyz__");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
