//! `hit uninstall` — 卸载软件

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 卸载参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要卸载的软件名（支持多个）
    pub apps: Vec<String>,

    /// 同时删除 persist 数据
    #[arg(short, long)]
    pub purge: bool,
}

/// 执行卸载（Phase 1.10.3 实现）
pub fn execute(_args: &Args, _session: &Session) -> anyhow::Result<()> {
    println!("{} uninstall 尚未实现", "[stub]".yellow());
    Ok(())
}
