//! Hit 命令行入口
//!
//! 主要模块：
//! - `cli`：clap 命令树（含 alias：i/s/u/rm/ls/st/b/c）
//! - `progress`：EventBus 订阅 → indicatif / colored 渲染
//! - `commands/`：各子命令实现

mod cli;
mod commands;
mod output;
mod progress;
mod welcome;
mod tables;

use std::process::ExitCode;

use clap::Parser;
use rusty_rich::{Console, Text};

use cli::{Cli, Command};
use progress::ProgressRenderer;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let mut console = Console::new();
            let err_str = e.to_string();
            let err_str = filter_clap_suggestion(&err_str);
            console.println(&Text::from_markup(&format!(
                "[red]错误[/red]: {}",
                err_str
            )));
            for cause in e.chain().skip(1) {
                let cause_str = cause.to_string();
                let cause_str = filter_clap_suggestion(&cause_str);
                console.println(&Text::from_markup(&format!(
                    "  [red]原因[/red]: {}",
                    cause_str
                )));
            }
            ExitCode::FAILURE
        }
    }
}

/// 过滤 clap 错误信息中的相似子命令建议，避免误导用户
fn filter_clap_suggestion(msg: &str) -> String {
    msg.lines()
        .filter(|line| !line.starts_with("  tip: a similar subcommand exists"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::try_parse().map_err(|e| {
        let err_str = e.to_string();
        let filtered = filter_clap_suggestion(&err_str);
        anyhow::anyhow!("{}", filtered)
    })?;

    init_tracing(cli.verbose);

    let session = hit_common::Session::new()?;

    // 首次启动引导：仅在 clap 成功解析出子命令后才检查，
    // 避免污染所有命令的输出（修 BUGS.md "Welcome 引导错误触发"）
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
        Command::Config(args) => commands::config::execute(args, &session),
        Command::Doctor(args) => commands::doctor::execute(args, &session),
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
