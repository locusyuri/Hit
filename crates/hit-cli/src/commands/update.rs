//! `hit update` — 更新已安装软件

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 更新参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要更新的软件名（留空配合 --all 使用）
    pub apps: Vec<String>,

    /// 更新所有已安装软件
    #[arg(short, long)]
    pub all: bool,

    /// 强制更新（忽略版本比较）
    #[arg(short, long)]
    pub force: bool,
}

/// 执行更新（Phase 1.10.7 实现）
pub fn execute(_args: &Args, _session: &Session) -> anyhow::Result<()> {
    println!("{} update 尚未实现", "[stub]".yellow());
    Ok(())
}
