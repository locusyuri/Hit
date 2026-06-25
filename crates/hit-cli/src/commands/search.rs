//! `hit search` — 搜索软件包

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 搜索参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 搜索关键词
    pub query: String,

    /// 限定搜索的 Bucket 名称
    #[arg(short, long)]
    pub bucket: Option<String>,
}

/// 执行搜索（Phase 1.10.5 实现）
pub fn execute(_args: &Args, _session: &Session) -> anyhow::Result<()> {
    println!("{} search 尚未实现", "[stub]".yellow());
    Ok(())
}
