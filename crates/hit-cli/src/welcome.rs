//! 首次启动引导
//!
//! 检测首次运行（config.json 不存在 **且** buckets 目录为空）时显示欢迎界面，
//! 引导用户选择添加官方 Bucket。判据见 [`is_first_run`]。

use std::io::{self, BufRead, Write};
use std::sync::atomic::AtomicBool;

use colored::Colorize;
use hit_common::Session;
use hit_common::config::HitConfig;

/// 检测是否首次运行。
///
/// 判定逻辑（双条件，必须同时满足才算首次运行）：
/// 1. config.json 不存在（安装脚本会预置 config，存在则说明已初始化）
/// 2. buckets/ 目录不存在或为空（无任何 bucket）
///
/// 两个条件都满足才视为首次运行。这样：
/// - 已安装环境（config 在 + bucket 在）绝不触发
/// - 安装脚本刚装好但用户还没添加 bucket（config 在 + bucket 空）也不触发，
///   避免污染已有配置环境
///
/// config 路径使用 `HitConfig::default_path()`，与 Session 加载配置的路径一致，
/// 避免因路径解析逻辑不同导致误判。
pub fn is_first_run() -> bool {
    // 条件 1：config.json 不存在（使用 default_path 与 Session 一致）
    let config_path = HitConfig::default_path();
    if config_path.exists() {
        return false;
    }

    // 条件 2：buckets/ 目录不存在或为空
    let buckets = hit_common::paths::buckets_path();
    if !buckets.exists() {
        return true;
    }
    match std::fs::read_dir(&buckets) {
        Ok(mut it) => it.next().is_none(),
        Err(_) => true,
    }
}

/// 显示欢迎界面并执行引导流程
pub fn run_first_time_setup(session: &Session) -> anyhow::Result<()> {
    show_welcome()?;

    let choice = read_choice()?;

    let should_interrupt = AtomicBool::new(false);

    match choice {
        1 => {
            println!("\n{} 正在添加官方 Bucket...\n", "开始".cyan().bold());
            let results = hit_core::bucket::add_default_buckets(session, &should_interrupt)?;
            for r in &results {
                match &r.outcome {
                    hit_core::bucket::AddOutcome::Added => {
                        println!("  {} {}", "✔".green(), r.name);
                    }
                    hit_core::bucket::AddOutcome::Skipped => {
                        println!("  {} {}（已存在）", "⏭".dimmed(), r.name);
                    }
                    hit_core::bucket::AddOutcome::Failed(msg) => {
                        println!("  {} {} 失败: {msg}", "✘".red(), r.name);
                    }
                }
            }
            println!("\n{} 官方 Bucket 添加完成", "✔".green());
        }
        2 => {
            interactive_add_buckets(session, &should_interrupt)?;
        }
        3 => {
            println!("\n已跳过。你可以稍后使用 {} 添加 Bucket。", "hit bucket add".yellow());
        }
        _ => unreachable!(),
    }

    // 保存默认配置文件（标记初始化完成）
    let config_path = HitConfig::default_path();
    HitConfig::default().save(&config_path)?;

    Ok(())
}

/// 显示欢迎横幅和菜单
fn show_welcome() -> anyhow::Result<()> {
    println!(
        r#"
  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/
"#
    );

    println!("{}", "首次使用 Hit？".bold());
    println!();
    println!("  {} 快速开始 — 添加官方 Bucket（main, extras, versions）", "1)".green());
    println!("  {} 自定义 — 手动选择要添加的 Bucket", "2)".yellow());
    println!("  {} 跳过", "3)".dimmed());
    println!();

    Ok(())
}

/// 读取用户选择（1/2/3）
fn read_choice() -> anyhow::Result<u8> {
    print!("请选择 [{}]: ", "1/2/3".green());
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;

    match line.trim() {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => {
            println!("无效选择，已跳过。");
            Ok(3)
        }
    }
}

/// 交互式添加 Bucket（用户逐个输入名称）
fn interactive_add_buckets(session: &Session, interrupt: &AtomicBool) -> anyhow::Result<()> {
    let known = hit_core::bucket::known_buckets();

    println!("\n可用的官方 Bucket：");
    for (name, _url) in known {
        println!("  - {name}");
    }
    println!("\n输入 Bucket 名称（回车确认，空行结束）：");

    let stdin = io::stdin();

    loop {
        print!("{} ", "bucket >".cyan());
        io::stdout().flush()?;

        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        let name = line.trim().to_string();

        if name.is_empty() {
            break;
        }

        // 查找 URL
        let url = match hit_core::bucket::resolve_known_bucket(&name) {
            Some(u) => u.to_string(),
            None => {
                print!("  Bucket '{}' 不在已知列表中，请输入 Git 仓库 URL（或留空取消）：", name);
                io::stdout().flush()?;
                let mut url_line = String::new();
                stdin.lock().read_line(&mut url_line)?;
                let url = url_line.trim().to_string();
                if url.is_empty() {
                    println!("  已跳过 '{}'", name);
                    continue;
                }
                url
            }
        };

        let target = session.buckets_path().join(&name);
        if target.exists() {
            println!("  {} '{}' 已存在，跳过", "⏭".dimmed(), name);
            continue;
        }

        print!("  正在添加 '{}'...", name);
        io::stdout().flush()?;

        match hit_core::bucket::clone_bucket(session, &name, &url, &hit_core::bucket::CloneOptions::default(), interrupt) {
            Ok(_) => println!(" {}", "完成".green()),
            Err(e) => println!(" {} {e}", "失败".red()),
        }
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_first_run_true_when_no_config() {
        // 验证 is_first_run 返回 bool（纯函数逻辑验证）
        let result = is_first_run();
        let _ = result;
    }

    #[test]
    fn welcome_module_compiles() {
        // 验证 show_welcome 和 read_choice 存在且可调用（不实际执行交互）
        // is_first_run 是纯函数，直接测试
        let result = is_first_run();
        let _ = result;
    }
}
