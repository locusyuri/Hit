//! 诊断消息类型。
//!
//! `validate()` 返回 `Diagnostics`：结构化收集所有 error / warning / info，
//! 供 CLI 渲染或日志输出。调用方可使用 `into_result()` 在存在 error 时
//! 降级为 `Result<()>`。

use hit_common::error::{HitError, Result};

/// 诊断严重等级。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// 阻断安装/使用的错误
    Error,
    /// 不阻断但建议修复的提示
    Warning,
    /// 参考信息（如缺少可选字段）
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }
}

/// 单条诊断消息。
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    /// JSON 路径（如 `"architecture.64bit.hash"`、`"version"`）。
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity.as_str(), self.field, self.message)
    }
}

/// 诊断消息集合。
#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    pub items: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加一条错误。
    pub fn push_error(&mut self, field: impl Into<String>, msg: impl Into<String>) {
        self.items.push(Diagnostic {
            severity: Severity::Error,
            field: field.into(),
            message: msg.into(),
        });
    }

    /// 添加一条警告。
    pub fn push_warning(&mut self, field: impl Into<String>, msg: impl Into<String>) {
        self.items.push(Diagnostic {
            severity: Severity::Warning,
            field: field.into(),
            message: msg.into(),
        });
    }

    /// 添加一条信息。
    pub fn push_info(&mut self, field: impl Into<String>, msg: impl Into<String>) {
        self.items.push(Diagnostic {
            severity: Severity::Info,
            field: field.into(),
            message: msg.into(),
        });
    }

    /// 是否存在错误。
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }

    /// 是否存在警告。
    pub fn has_warnings(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Warning)
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter().filter(|d| d.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter().filter(|d| d.severity == Severity::Warning)
    }

    pub fn infos(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter().filter(|d| d.severity == Severity::Info)
    }

    /// 转换为 `Result<()>`：有 error 时返回 `Err(HitError::Manifest)`，否则 `Ok(())`。
    pub fn into_result(self, app: &str) -> Result<()> {
        if self.has_errors() {
            let summary: String = self
                .errors()
                .map(|d| format!("{}: {}", d.field, d.message))
                .collect::<Vec<_>>()
                .join("; ");
            Err(HitError::Manifest {
                app: app.to_string(),
                message: summary,
            })
        } else {
            Ok(())
        }
    }

    /// 诊断总数。
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
