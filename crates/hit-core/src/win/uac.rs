//! UAC 管理员检测与权限提升
//!
//! 使用 `windows` crate 调用 Win32 API：
//! - `is_admin`：通过 `OpenProcessToken` + `GetTokenInformation(TokenElevation)` 检测
//! - `elevate_self`：通过 `ShellExecuteW` + `"runas"` 动词触发 UAC 提示

use hit_common::error::{HitError, Result};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;

/// 检查当前进程是否以管理员权限运行
///
/// 使用 `OpenProcessToken` + `GetTokenInformation(TokenElevation)` 检测。
pub fn is_admin() -> Result<bool> {
    unsafe {
        let mut token = windows::Win32::Foundation::HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).map_err(|e| {
            HitError::Permission {
                message: format!("OpenProcessToken 失败: {e}"),
            }
        })?;

        let mut elevation = TOKEN_ELEVATION::default();
        let mut ret_len = 0u32;
        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut ret_len,
        );

        let _ = windows::Win32::Foundation::CloseHandle(token);

        match result {
            Ok(()) => Ok(elevation.TokenIsElevated != 0),
            Err(e) => Err(HitError::Permission {
                message: format!("GetTokenInformation 失败: {e}"),
            }),
        }
    }
}

/// 以管理员身份重新启动当前可执行文件
///
/// 通过 `ShellExecuteW` + `"runas"` 动词触发 UAC 提示框。
/// `Ok(true)` 表示提升成功，`Ok(false)` 表示用户拒绝。
pub fn elevate_self(args: &[&str]) -> Result<bool> {
    let exe = std::env::current_exe().map_err(|e| HitError::Io {
        context: "获取当前可执行文件路径".into(),
        source: e,
    })?;

    let exe_wide: Vec<u16> = exe
        .to_string_lossy()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let verb_wide: Vec<u16> = "runas".encode_utf16().chain(std::iter::once(0)).collect();

    let args_str = args.join(" ");
    let args_wide: Vec<u16> = if args_str.is_empty() {
        vec![0]
    } else {
        args_str.encode_utf16().chain(std::iter::once(0)).collect()
    };

    unsafe {
        let result = ShellExecuteW(
            HWND::default(),
            PCWSTR(verb_wide.as_ptr()),
            PCWSTR(exe_wide.as_ptr()),
            PCWSTR(if args_str.is_empty() {
                std::ptr::null()
            } else {
                args_wide.as_ptr()
            }),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        );

        // ShellExecuteW 返回值 > 32 表示成功
        Ok(result.0 as i32 > 32)
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_admin_returns_bool() {
        // 在普通测试运行器中通常为 false，但以管理员运行时为 true
        let result = is_admin();
        assert!(result.is_ok());
    }
}
