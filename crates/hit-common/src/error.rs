//! Hit 统一错误类型
//!
//! 使用 `thiserror` 定义 `HitError` 枚举，覆盖 IO、Manifest、Bucket、Download、
//! Install、Config 等错误类别。对外暴露 `type Result<T> = std::result::Result<T, HitError>`。

use std::io;
use std::path::PathBuf;

/// Hit 顶层错误枚举
///
/// 每个变体对应一个核心模块的错误类别。所有变体携带足够的上下文信息用于
/// 向用户渲染可读的错误消息与修复建议。
#[derive(Debug, thiserror::Error)]
pub enum HitError {
    /// IO 错误（文件系统、网络底层等）
    #[error("IO 错误：{context}：{source}")]
    Io {
        context: String,
        #[source]
        source: io::Error,
    },

    /// 配置加载/保存错误
    #[error("配置错误：{message}")]
    Config { message: String },

    /// Manifest 解析/验证错误
    #[error("Manifest 错误（{app}）：{message}")]
    Manifest { app: String, message: String },

    /// Bucket 仓库错误（克隆、拉取、索引）
    #[error("Bucket '{bucket}' 错误：{message}")]
    Bucket { bucket: String, message: String },

    /// HTTP 下载错误
    #[error("下载失败（{url}）：{message}")]
    Download { url: String, message: String },

    /// 哈希校验失败
    #[error("哈希不匹配（{path}）：期望 {expected}，实际 {actual}")]
    HashMismatch {
        path: PathBuf,
        expected: String,
        actual: String,
    },

    /// 解压错误（ZIP / 7z / TAR / 安装程序）
    #[error("解压失败（{archive}）：{message}")]
    Compress { archive: String, message: String },

    /// 安装流水线错误（事务、回滚、依赖、persist）
    #[error("安装 '{app}' 失败：{message}")]
    Install { app: String, message: String },

    /// 卸载错误
    #[error("卸载 '{app}' 失败：{message}")]
    Uninstall { app: String, message: String },

    /// Shim 创建/移除错误
    #[error("Shim 错误（{name}）：{message}")]
    Shim { name: String, message: String },

    /// Persist（symlink / junction）错误
    #[error("Persist 错误（{app}）：{message}")]
    Persist { app: String, message: String },

    /// 软件/版本未找到
    #[error("未找到 {kind} '{name}'{extra}")]
    NotFound {
        kind: String,
        name: String,
        extra: String,
    },

    /// 非法参数（CLI 输入、版本约束等）
    #[error("非法参数：{message}")]
    InvalidArgument { message: String },

    /// 权限不足（需要 UAC 提升或开发者模式）
    #[error("权限不足：{message}")]
    Permission { message: String },

    /// 事务性安装回滚
    #[error("安装已回滚（{app}）：{reason}")]
    Rollback { app: String, reason: String },

    /// 通用上下文错误（anyhow 兜底，便于跨 crate 传播）
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Hit 模块统一 Result 别名
pub type Result<T> = std::result::Result<T, HitError>;

impl HitError {
    /// 便捷构造：IO 错误（带上下文）
    pub fn io(context: impl Into<String>, source: io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }

    /// 便捷构造：软件未找到
    pub fn app_not_found(app: impl Into<String>) -> Self {
        Self::NotFound {
            kind: "软件".into(),
            name: app.into(),
            extra: String::new(),
        }
    }

    /// 便捷构造：Bucket 未找到
    pub fn bucket_not_found(bucket: impl Into<String>) -> Self {
        Self::NotFound {
            kind: "Bucket".into(),
            name: bucket.into(),
            extra: String::new(),
        }
    }
}

impl From<io::Error> for HitError {
    fn from(err: io::Error) -> Self {
        Self::Io {
            context: "未指定上下文".into(),
            source: err,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_alias_works() {
        let ok: Result<i32> = Ok(42);
        assert!(ok.is_ok());
        assert!(matches!(ok, Ok(42)));
    }

    #[test]
    fn error_display_includes_context() {
        let err = HitError::Config {
            message: "字段缺失".into(),
        };
        assert!(err.to_string().contains("字段缺失"));
    }

    #[test]
    fn app_not_found_helper() {
        let err = HitError::app_not_found("git");
        match err {
            HitError::NotFound { kind, name, .. } => {
                assert_eq!(kind, "软件");
                assert_eq!(name, "git");
            }
            _ => panic!("期望 NotFound 变体"),
        }
    }
}
