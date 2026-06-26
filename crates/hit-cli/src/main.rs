//! Hit 命令行入口
//!
//! 主要模块：
//! - `cli`：clap 命令树（含 alias：i/s/u/rm/ls/st/b/c）
//! - `progress`：EventBus 订阅 → indicatif / colored 渲染
//! - `commands/`：各子命令实现

mod cli;
mod commands;
mod progress;
mod welcome;

use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;

use cli::{Cli, Command};
use progress::ProgressRenderer;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}: {e}", "错误".red());
            for cause in e.chain().skip(1) {
                eprintln!("  {}: {cause}", "原因".red());
            }
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    init_tracing(cli.verbose);

    let session = hit_common::Session::new()?;

    // 首次启动引导
    if welcome::is_first_run() {
        welcome::run_first_time_setup(&session)?;
    }

    let progress = ProgressRenderer::start(&session);

    let result = match &cli.command {
        Command::Install(args) => commands::install::execute(args, &session),
        Command::Search(args) => commands::search::execute(args, &session),
        Command::Update(args) => commands::update::execute(args, &session),
        Command::Uninstall(args) => commands::uninstall::execute(args, &session),
        Command::List(args) => commands::list::execute(args, &session),
        Command::Status(args) => commands::status::execute(args, &session),
        Command::Bucket(args) => commands::bucket::execute(args, &session),
        Command::Info(args) => commands::info::execute(args, &session),
        Command::Reset(args) => commands::reset::execute(args, &session),
        Command::Cache(args) => commands::cache::execute(args, &session),
        Command::Home(args) => commands::home::execute(args, &session),
        Command::Cleanup(args) => commands::cleanup::execute(args, &session),
        Command::Which(args) => commands::which::execute(args, &session),
        Command::Prefix(args) => commands::prefix::execute(args, &session),
        Command::Hold(args) => commands::hold::execute(args, &session),
        Command::Unhold(args) => commands::unhold::execute(args, &session),
    };

    progress.stop();
    result
}

/// 根据 -v 计数初始化 tracing 日志级别
fn init_tracing(verbose: u8) {
    let level = match verbose {
        0 => tracing::Level::WARN,
        1 => tracing::Level::INFO,
        2 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();
}
