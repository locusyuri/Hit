//! tracing 日志初始化
//!
//! 提供统一的日志入口，供 hit-cli / hit-shim 在启动时调用一次。

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// 初始化全局 tracing 日志
///
/// - `level`：日志级别（`Level::INFO` / `DEBUG` / `TRACE` 等）
/// - 使用 `FmtSubscriber`，输出到 stderr
/// - 重复调用会被 `try_init` 忽略
pub fn init(level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}
