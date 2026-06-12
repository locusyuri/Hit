//! Hit 基础类型库
//!
//! 提供跨 crate 共享的基础设施：
//! - `error`：统一错误类型 `HitError`（thiserror）
//! - `config`：用户配置 `Config`
//! - `paths`：Scoop 兼容路径计算
//! - `session`：`Session`/Context 模式
//! - `event`：`EventBus` + `Event` 枚举
