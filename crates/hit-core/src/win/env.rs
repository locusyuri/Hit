//! PATH 管理与环境变量广播
//!
//! 提供对用户 PATH 环境变量的增删操作，并通过 `WM_SETTINGCHANGE`
//! 广播通知其他进程环境变量已更新。
//!
//! 参考 Scoop `system.ps1` 的 `add_first_in_path` / `rm_from_path` / `broadcast_env_update`。

use std::path::Path;

use hit_common::error::Result;

use super::registry;

/// 将路径添加到用户 PATH
///
/// 操作顺序：注册表去重 → 前置插入 → 当前进程 → 广播
pub fn add_to_path(paths: &[&Path], env_name: &str) -> Result<()> {
    let mut entries = registry::get_path_entries(env_name)?;

    for &p in paths.iter().rev() {
        let pb = p.to_path_buf();
        let pb_lower = pb.to_string_lossy().to_lowercase();
        entries.retain(|e| e.to_string_lossy().to_lowercase() != pb_lower);
        entries.insert(0, pb);
    }

    registry::set_path_entries(env_name, &entries)?;

    let new_path = std::env::join_paths(&entries).unwrap_or_default();
    // SAFETY: 仅在 Windows 平台上调用，单线程修改环境变量
    unsafe { std::env::set_var(env_name, &new_path) };

    broadcast_env_change()?;
    Ok(())
}

/// 从用户 PATH 移除匹配的路径
///
/// `patterns` 为路径字符串片段，包含任一片段的路径条目将被移除。
pub fn remove_from_path(patterns: &[&str], env_name: &str) -> Result<()> {
    let entries = registry::get_path_entries(env_name)?;
    let filtered: Vec<_> = entries
        .into_iter()
        .filter(|e| {
            let s = e.to_string_lossy();
            !patterns.iter().any(|p| s.contains(p))
        })
        .collect();

    registry::set_path_entries(env_name, &filtered)?;

    let new_path = std::env::join_paths(&filtered).unwrap_or_default();
    // SAFETY: 仅在 Windows 平台上调用，单线程修改环境变量
    unsafe { std::env::set_var(env_name, &new_path) };

    broadcast_env_change()?;
    Ok(())
}

/// 确保 Hit shims 目录在用户 PATH 中（幂等）
pub fn ensure_shims_in_path(shims_dir: &Path) -> Result<()> {
    let entries = registry::get_path_entries("PATH")?;

    let target_lower = shims_dir.to_string_lossy().to_lowercase();
    if entries.iter().any(|e| e.to_string_lossy().to_lowercase() == target_lower) {
        return Ok(());
    }

    add_to_path(&[shims_dir], "PATH")
}

/// 从用户 PATH 中移除 Hit shims 目录
pub fn remove_shims_from_path(shims_dir: &Path) -> Result<()> {
    let shims_str = shims_dir.to_string_lossy().into_owned();
    remove_from_path(&[&shims_str], "PATH")
}

/// 广播 `WM_SETTINGCHANGE` 通知所有窗口环境变量已更新
///
/// `HWND_BROADCAST(0xFFFF)` + `WM_SETTINGCHANGE(0x1A)` + `lParam="Environment"`
/// `SMTO_ABORTIFHUNG` + 5000ms 超时
pub fn broadcast_env_change() -> Result<()> {
    use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        SMTO_ABORTIFHUNG, SendMessageTimeoutW,
    };
    use windows::core::PCWSTR;

    let env_wide: Vec<u16> = "Environment"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut result = 0;
        let _ = SendMessageTimeoutW(
            HWND(0xFFFF as _),
            0x001A, // WM_SETTINGCHANGE
            WPARAM(0),
            LPARAM(PCWSTR(env_wide.as_ptr()).0 as isize),
            SMTO_ABORTIFHUNG,
            5000,
            Some(&mut result),
        );
    }

    Ok(())
}

/// 设置单个环境变量（写注册表 + 当前进程 + 广播）
///
/// `value` 为 `None` 时删除该变量。
pub fn set_env_var(name: &str, value: Option<&str>) -> Result<()> {
    use std::ffi::OsString;

    match value {
        Some(v) => {
            let os = OsString::from(v);
            registry::set_env_value(name, Some(&os))?;
            // SAFETY: 仅在 Windows 平台上调用，单线程修改环境变量
            unsafe { std::env::set_var(name, v) };
        }
        None => {
            registry::set_env_value(name, None)?;
            // SAFETY: 仅在 Windows 平台上调用，单线程修改环境变量
            unsafe { std::env::remove_var(name) };
        }
    }

    broadcast_env_change()
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broadcast_env_change_no_panic() {
        let result = broadcast_env_change();
        assert!(result.is_ok());
    }

    #[test]
    fn add_to_path_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let test_path = dir.path().join("hit_test_shims");
        std::fs::create_dir_all(&test_path).unwrap();

        // 使用临时变量名避免干扰系统 PATH
        let env_name = "__HIT_TEST_PATH__";
        // 确保初始状态干净
        registry::set_env_value(env_name, None).ok();

        add_to_path(&[&test_path], env_name).unwrap();
        let entries1 = registry::get_path_entries(env_name).unwrap();

        add_to_path(&[&test_path], env_name).unwrap();
        let entries2 = registry::get_path_entries(env_name).unwrap();

        assert_eq!(entries1.len(), entries2.len(), "重复添加不应产生重复条目");

        // 清理
        registry::set_env_value(env_name, None).ok();
    }

    #[test]
    fn remove_from_path_pattern() {
        let env_name = "__HIT_TEST_PATH_RM__";
        registry::set_env_value(env_name, None).ok();

        let dir = tempfile::tempdir().unwrap();
        let p1 = dir.path().join("keep_this");
        let p2 = dir.path().join("remove_this_hit_test");
        std::fs::create_dir_all(&p1).unwrap();
        std::fs::create_dir_all(&p2).unwrap();

        registry::set_path_entries(env_name, &[p1.clone(), p2.clone()]).unwrap();

        remove_from_path(&["remove_this_hit_test"], env_name).unwrap();

        let remaining = registry::get_path_entries(env_name).unwrap();
        assert_eq!(remaining.len(), 1);

        // 清理
        registry::set_env_value(env_name, None).ok();
    }
}
