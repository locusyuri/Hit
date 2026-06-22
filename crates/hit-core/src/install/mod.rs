//! 安装子模块：编排完整安装流水线（解析 → 依赖 → 下载 → 校验 → 解压 → shim → persist → commit）
//!
//! 子模块划分：
//! - `transaction`：RAII 事务管理（staging + 原子移动 + 回滚）
//! - `dependency`：依赖图拓扑排序与循环检测
//! - `persist`：配置文件/目录的持久化链接（junction / hard link）
//! - `shim`：shim 代理的创建 / 移除 / PE 修补
//! - `controller`：安装 / 卸载 / 版本切换的流水线控制器

pub mod transaction;
mod dependency;
mod persist;
mod shim;
mod controller;

pub use transaction::{Transaction, TxState, UndoAction};
pub use dependency::{parse_dep_spec, resolve_dependencies, ResolvedDep};
pub use persist::{link_persist, relink_persist, unlink_persist};
pub use shim::{create_shim, list_app_shims, remove_app_shims};
pub use controller::{install, reset_version, uninstall, InstallOptions, InstallResult};
