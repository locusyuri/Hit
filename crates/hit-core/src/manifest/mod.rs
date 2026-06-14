//! Manifest 子模块：Scoop Manifest 的 serde 数据结构、解析器与验证器。
//!
//! 结构定义遵循 Scoop 官方 Manifest JSON 格式，并预留了 Hit 扩展字段占位。
//! 多态字段（`bin` / `persist` / `shortcuts` / `license` / `hash` / `checkver` 等）
//! 使用手写 `Deserialize` 或 `#[serde(untagged)]` 建模。

mod parser;
mod schema;
mod validator;

pub use parser::parse_str;
pub use schema::*;
pub use validator::validate;
