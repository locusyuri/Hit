//! Hit 基础类型库
//!
//! 提供跨 crate 共享的基础设施：
//! - `error`：统一错误类型 `HitError`（thiserror）
//! - `config`：用户配置 `HitConfig`
//! - `paths`：Scoop 兼容路径计算
//! - `log`：tracing 日志初始化
//! - `session`：`Session`/Context 模式（1.1.7）
//! - `event`：`EventBus` + `Event` 枚举（1.1.8）

pub mod config;
pub mod error;
pub mod log;
pub mod paths;

pub use config::HitConfig;
pub use error::{HitError, Result};
