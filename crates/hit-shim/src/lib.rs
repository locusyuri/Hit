//! Hit Shim 共享库
//!
//! 提供 `.shim` 文件解析和进程转发功能，供 hit-cli 的 `which` 命令使用。

pub mod parse;
pub mod process;
