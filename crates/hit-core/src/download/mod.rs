//! HTTP 下载模块
//!
//! 提供基于 reqwest blocking client 的 HTTP 下载能力，支持 proxy 配置和进度事件上报。

pub mod http;

pub use http::{build_client, download_file};
