//! Manifest 验证器骨架。
//!
//! 本阶段仅做必填字段非空校验。
//! 深度规则（hash 与 url 数组长度一致、license 合法性、变量语法等）留待 1.2.5。

use crate::manifest::schema::Manifest;
use hit_common::error::Result;

/// 校验 Manifest 必填字段。
///
/// 失败时使用 `HitError::Other(anyhow)` 兜底，调用方可再用
/// `HitError::Manifest { app, message }` 包裹提供上下文。
pub fn validate(m: &Manifest) -> Result<()> {
    if m.version.is_empty() {
        return Err(anyhow::anyhow!("manifest 缺少 version 字段").into());
    }
    if m.description.is_empty() {
        return Err(anyhow::anyhow!("manifest 缺少 description 字段").into());
    }
    if m.homepage.is_empty() {
        return Err(anyhow::anyhow!("manifest 缺少 homepage 字段").into());
    }
    if m.license.identifier().is_empty() {
        return Err(anyhow::anyhow!("manifest 缺少 license 字段").into());
    }
    Ok(())
}
