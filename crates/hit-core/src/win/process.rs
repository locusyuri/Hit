//! 进程检测与终止
//!
//! 使用 `sysinfo` crate 检测指定路径前缀下正在运行的进程，
//! 并提供终止和等待退出的能力。与 Scoop `test_running_process`
//! 和 Hok `running_apps` 行为一致。

use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::thread;
use std::time::Duration;

use hit_common::error::{HitError, Result};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

/// 进程信息快照
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe_path: PathBuf,
}

/// 全局 System 单例，避免每次检测都重新创建
static SYSINFO: LazyLock<Mutex<System>> = LazyLock::new(|| Mutex::new(System::new_all()));

/// 检测指定路径前缀下正在运行的进程
///
/// 遍历系统全部进程，筛选 `exe_path` 以 `prefix` 开头的进程。
pub fn find_running_processes(prefix: &Path) -> Result<Vec<ProcessInfo>> {
    let mut sys = SYSINFO
        .lock()
        .map_err(|e| HitError::Io {
            context: "锁定 sysinfo 单例失败".into(),
            source: std::io::Error::other(e.to_string()),
        })?;

    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_exe(UpdateKind::Always),
    );

    let prefix_str = prefix.to_string_lossy();
    let mut result = Vec::new();

    for process in sys.processes().values() {
        if let Some(exe) = process.exe() {
            let exe_str = exe.to_string_lossy();
            if exe_str.starts_with(prefix_str.as_ref()) {
                result.push(ProcessInfo {
                    pid: process.pid().as_u32(),
                    name: process.name().to_string_lossy().into_owned(),
                    exe_path: exe.to_path_buf(),
                });
            }
        }
    }

    Ok(result)
}

/// 终止指定进程（Windows 上为 TerminateProcess）
///
/// 进程不存在时返回 `Ok(false)`。
pub fn kill_process(pid: u32) -> Result<bool> {
    let sys = SYSINFO
        .lock()
        .map_err(|e| HitError::Io {
            context: "锁定 sysinfo 单例失败".into(),
            source: std::io::Error::other(e.to_string()),
        })?;

    let pid = sysinfo::Pid::from_u32(pid);
    match sys.process(pid) {
        Some(process) => {
            process.kill();
            Ok(true)
        }
        None => Ok(false),
    }
}

/// 等待指定进程退出，超时后返回 `false`
pub fn wait_for_exit(pid: u32, timeout_ms: u64) -> Result<bool> {
    let sys = SYSINFO
        .lock()
        .map_err(|e| HitError::Io {
            context: "锁定 sysinfo 单例失败".into(),
            source: std::io::Error::other(e.to_string()),
        })?;

    let sysinfo_pid = sysinfo::Pid::from_u32(pid);
    if sys.process(sysinfo_pid).is_none() {
        return Ok(true);
    }
    drop(sys);

    let deadline = Duration::from_millis(timeout_ms);
    let interval = Duration::from_millis(100);
    let mut elapsed = Duration::ZERO;

    while elapsed < deadline {
        thread::sleep(interval);
        elapsed += interval;

        let mut sys = SYSINFO
            .lock()
            .map_err(|e| HitError::Io {
                context: "锁定 sysinfo 单例失败".into(),
                source: std::io::Error::other(e.to_string()),
            })?;

        sys.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[sysinfo_pid]),
            true,
            ProcessRefreshKind::nothing(),
        );

        if sys.process(sysinfo_pid).is_none() {
            return Ok(true);
        }
    }

    Ok(false)
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_running_processes_self() {
        let current_exe = std::env::current_exe().unwrap();
        let prefix = current_exe.parent().unwrap();
        let result = find_running_processes(prefix);
        assert!(result.is_ok());
        let procs = result.unwrap();
        assert!(
            !procs.is_empty(),
            "当前测试进程应被检测到"
        );
    }

    #[test]
    fn find_running_processes_empty() {
        let prefix = Path::new(r"Z:\nonexistent\path\that\does\not\exist");
        let result = find_running_processes(prefix);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn kill_process_nonexistent() {
        let result = kill_process(u32::MAX);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
