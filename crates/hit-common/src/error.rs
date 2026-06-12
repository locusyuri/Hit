//! Hit 统一错误类型
//!
//! 使用 `thiserror` 定义 `HitError` 枚举，覆盖 IO、Manifest、Bucket、Download、
//! Install、Config 等错误类别。对外暴露 `type Result<T> = std::result::Result<T, HitError>`。
//!
//! 1.1.5 阶段仅放置骨架，具体变体将在 1.1.6 任务中填充。

use std::io;

/// Hit 顶层错误枚举
#[derive(Debug, thiserror::Error)]
pub enum HitError {
    /// IO 错误（文件系统、网络底层等）
    #[error("IO 错误：{0}")]
    Io(#[from] io::Error),

    /// 配置加载/保存错误
    #[error("配置错误：{message}")]
    Config { message: String },

    /// Manifest 解析/验证错误（占位，详见 1.1.6）
    #[error("Manifest 错误：{message}")]
    Manifest { message: String },

    /// 通用上下文错误（anyhow 兜底）
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Hit 模块统一 Result 别名
pub type Result<T> = std::result::Result<T, HitError>;
