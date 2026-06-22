//! 进程启动与转发
//!
//! 使用 `std::process::Command` 启动目标进程，
//! 继承 stdin/stdout/stderr 并返回退出码。

use std::process::{Command, ExitCode};

/// Windows `CREATE_NEW_PROCESS_GROUP` 标志
#[cfg(windows)]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;

/// 启动目标进程并等待退出
///
/// - 继承 stdin/stdout/stderr（`Command` 默认行为）
/// - Windows 下使用 `CREATE_NEW_PROCESS_GROUP` 标志
/// - 返回子进程的 `ExitCode`
pub fn run_target(path: &str, args: &[String]) -> ExitCode {
    let mut cmd = Command::new(path);
    cmd.args(args);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NEW_PROCESS_GROUP);
    }

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(status.code().unwrap_or(1) as u8)
            }
        }
        Err(e) => {
            eprintln!("hit-shim: 无法启动 '{path}': {e}");
            ExitCode::FAILURE
        }
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_target_cmd_exit_zero() {
        let code = run_target("cmd", &["/c".into(), "exit".into(), "0".into()]);
        assert_eq!(code, ExitCode::SUCCESS);
    }

    #[test]
    fn run_target_nonexistent() {
        let code = run_target(r"Z:\nonexistent\program.exe", &[]);
        assert_eq!(code, ExitCode::FAILURE);
    }
}
