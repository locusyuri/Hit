//! Manifest 解析器（薄封装）。
//!
//! 本阶段仅暴露 `parse_str`：调用 `sonic_rs::from_str` 反序列化。
//! 后续任务（1.2.3 变量替换 / 1.2.4 parser 业务逻辑）会在此扩展。

use crate::manifest::schema::Manifest;
use hit_common::error::Result;

/// 从字符串解析 Manifest。
///
/// 失败时使用 `HitError::Other(anyhow)` 兜底，安装流水线调用点会再用
/// `HitError::Manifest { app, message }` 包裹以提供应用名上下文。
pub fn parse_str(input: &str) -> Result<Manifest> {
    sonic_rs::from_str::<Manifest>(input)
        .map_err(|e| anyhow::anyhow!("manifest JSON 解析失败：{e}").into())
}
