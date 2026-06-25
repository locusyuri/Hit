//! `hit list` — 列出已安装软件

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 列表参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 按名称过滤（可选）
    pub filter: Option<String>,
}

/// 执行列表（Phase 1.10.4 实现）
pub fn execute(_args: &Args, _session: &Session) -> anyhow::Result<()> {
    println!("{} list 尚未实现", "[stub]".yellow());
    Ok(())
}
