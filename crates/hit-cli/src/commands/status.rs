//! `hit status` — 查看系统状态

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 状态参数
#[derive(ClapArgs, Debug)]
pub struct Args;

/// 执行状态检查（Phase 2 实现）
pub fn execute(_args: &Args, _session: &Session) -> anyhow::Result<()> {
    println!("{} status 尚未实现", "[stub]".yellow());
    Ok(())
}
