//! Windows 平台集成模块
//!
//! 仅在 `#[cfg(windows)]` 下编译，提供：
//! - 进程检测与终止（sysinfo）
//! - 注册表读写（winreg）
//! - Junction / HardLink 文件系统链接（junction）
//! - UAC 管理员检测与权限提升（windows crate）
//! - PATH 管理与环境变量广播（windows crate）

pub mod env;
pub mod fs;
pub mod process;
pub mod registry;
pub mod uac;
