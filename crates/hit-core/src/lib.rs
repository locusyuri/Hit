//! Hit 核心业务逻辑库
//!
//! 内部模块划分：
//! - `manifest/`：Manifest 解析（schema / parser / validator / variables）
//! - `bucket/`：Bucket git 仓库克隆/更新/索引
//! - `download/`：HTTP 下载与缓存
//! - `hash/`：哈希校验（sha256/sha512/blake3）
//! - `compress/`：ZIP / 7z / TAR 解压与静默安装
//! - `store/`：JSON 文件存储（db.json）
//! - `install/`：安装流水线（controller / transaction / persist / dependency / hooks）
//! - `shim_mgmt/`：Shim 创建/移除/枚举
//! - `win/`：Windows 平台集成（`#[cfg(windows)]`）

pub mod manifest;
pub mod bucket;
pub mod download;
pub mod hash;
