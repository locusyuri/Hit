//! `hit cleanup` — 清理旧版本与缓存

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 清理参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要清理的软件名（留空配合 --all 使用）
    pub apps: Vec<String>,

    /// 清理所有旧版本
    #[arg(short, long)]
    pub all: bool,

    /// 同时清理下载缓存
    #[arg(short, long)]
    pub cache: bool,
}

/// 执行清理（Phase 2 实现）
pub fn execute(_args: &Args, _session: &Session) -> anyhow::Result<()> {
    println!("{} cleanup 尚未实现", "[stub]".yellow());
    Ok(())
}
