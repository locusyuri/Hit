//! clap 命令树定义
//!
//! 使用 `#[derive(Parser)]` + `#[derive(Subcommand)]` 定义 Hit 的子命令结构。
//! 支持 8 个子命令及其快捷 alias：install(i), search(s), update(u), uninstall(rm),
//! list(ls), status(st), bucket(b), cleanup(c)。

use clap::{Parser, Subcommand};

use crate::commands;

/// Hit — Scoop 兼容的 Windows 包管理器
#[derive(Parser)]
#[command(
    name = "hit",
    version,
    about = "Hit — Scoop 兼容的 Windows 包管理器",
    subcommand_required = true,
    arg_required_else_help = true,
    max_term_width = 100,
)]
pub struct Cli {
    /// 子命令
    #[command(subcommand)]
    pub command: Command,

    /// 日志级别（-v / -vv / -vvv）
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

/// 子命令枚举
#[derive(Subcommand)]
pub enum Command {
    /// 安装软件包
    #[clap(alias = "i")]
    Install(commands::install::Args),

    /// 搜索软件包
    #[clap(alias = "s")]
    Search(commands::search::Args),

    /// 更新已安装软件
    #[clap(alias = "u")]
    Update(commands::update::Args),

    /// 卸载软件
    #[clap(alias = "rm")]
    Uninstall(commands::uninstall::Args),

    /// 列出已安装软件
    #[clap(alias = "ls")]
    List(commands::list::Args),

    /// 查看系统状态
    #[clap(alias = "st")]
    Status(commands::status::Args),

    /// 管理 Bucket 仓库
    #[clap(alias = "b")]
    Bucket(commands::bucket::Args),

    /// 查看软件包详情
    Info(commands::info::Args),

    /// 切换软件版本
    #[clap(alias = "r")]
    Reset(commands::reset::Args),

    /// 管理下载缓存
    Cache(commands::cache::Args),

    /// 打开软件主页
    Home(commands::home::Args),

    /// 清理旧版本与缓存
    #[clap(alias = "c")]
    Cleanup(commands::cleanup::Args),

    /// 查找命令对应的 shim 路径
    Which(commands::which::Args),

    /// 显示安装路径
    Prefix(commands::prefix::Args),

    /// 锁定软件版本（update 时跳过）
    Hold(commands::hold::Args),

    /// 解除版本锁定
    Unhold(commands::unhold::Args),

    /// 管理配置
    Config(commands::config::Args),

    /// 健康检查与修复
    Doctor(commands::doctor::Args),

    /// 交互式搜索并安装
    Si(commands::si::Args),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_install_with_apps() {
        let cli = Cli::try_parse_from(["hit", "install", "git", "curl"]).unwrap();
        match cli.command {
            Command::Install(args) => assert_eq!(args.apps, vec!["git", "curl"]),
            _ => panic!("期望 Install 子命令"),
        }
    }

    #[test]
    fn alias_i_resolves_to_install() {
        let cli = Cli::try_parse_from(["hit", "i", "git"]).unwrap();
        assert!(matches!(cli.command, Command::Install(_)));
    }

    #[test]
    fn alias_s_resolves_to_search() {
        let cli = Cli::try_parse_from(["hit", "s", "curl"]).unwrap();
        assert!(matches!(cli.command, Command::Search(_)));
    }

    #[test]
    fn alias_u_resolves_to_update() {
        let cli = Cli::try_parse_from(["hit", "u"]).unwrap();
        assert!(matches!(cli.command, Command::Update(_)));
    }

    #[test]
    fn alias_rm_resolves_to_uninstall() {
        let cli = Cli::try_parse_from(["hit", "rm", "git"]).unwrap();
        assert!(matches!(cli.command, Command::Uninstall(_)));
    }

    #[test]
    fn alias_ls_resolves_to_list() {
        let cli = Cli::try_parse_from(["hit", "ls"]).unwrap();
        assert!(matches!(cli.command, Command::List(_)));
    }

    #[test]
    fn alias_st_resolves_to_status() {
        let cli = Cli::try_parse_from(["hit", "st"]).unwrap();
        assert!(matches!(cli.command, Command::Status(_)));
    }

    #[test]
    fn alias_b_resolves_to_bucket() {
        let cli = Cli::try_parse_from(["hit", "b", "list"]).unwrap();
        assert!(matches!(cli.command, Command::Bucket(_)));
    }

    #[test]
    fn alias_c_resolves_to_cleanup() {
        let cli = Cli::try_parse_from(["hit", "c"]).unwrap();
        assert!(matches!(cli.command, Command::Cleanup(_)));
    }

    #[test]
    fn verbose_flag_counts() {
        let cli = Cli::try_parse_from(["hit", "-vv", "ls"]).unwrap();
        assert_eq!(cli.verbose, 2);
    }

    #[test]
    fn install_force_flag() {
        let cli = Cli::try_parse_from(["hit", "install", "git", "--force"]).unwrap();
        match cli.command {
            Command::Install(args) => {
                assert_eq!(args.apps, vec!["git"]);
                assert!(args.force);
            }
            _ => panic!("期望 Install 子命令"),
        }
    }

    #[test]
    fn no_subcommand_shows_help() {
        let result = Cli::try_parse_from(["hit"]);
        assert!(result.is_err());
    }
}
