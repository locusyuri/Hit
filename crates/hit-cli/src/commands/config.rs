//! `hit config` — 管理配置

use clap::{Args as ClapArgs, Subcommand};
use rusty_rich::{Console, Text};
use hit_common::Session;

/// 配置管理参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 配置子命令
    #[command(subcommand)]
    pub subcmd: ConfigCmd,
}

/// 配置子子命令
#[derive(Subcommand, Debug)]
pub enum ConfigCmd {
    /// 显示所有配置项
    List,
    /// 修改配置项
    Set {
        /// 配置项名称
        key: String,
        /// 配置值
        value: String,
    },
}

/// 执行配置操作
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    match &args.subcmd {
        ConfigCmd::List => cmd_list(session),
        ConfigCmd::Set { key, value } => cmd_set(session, key, value),
    }
}

/// config list — 显示所有配置项
fn cmd_list(session: &Session) -> anyhow::Result<()> {
    let cfg = session.config();
    let mut console = Console::new();

    console.println(&Text::from_markup(&format!(
        "[bold]{:<30}[/bold] {}",
        "proxy",
        cfg.proxy.as_deref().unwrap_or("(未设置)")
    )));
    console.println(&Text::from_markup(&format!(
        "[bold]{:<30}[/bold] {}",
        "mirror",
        cfg.mirror.as_deref().unwrap_or("(未设置)")
    )));
    console.println(&Text::from_markup(&format!(
        "[bold]{:<30}[/bold] {}",
        "aria2_enabled",
        cfg.aria2_enabled
    )));
    console.println(&Text::from_markup(&format!(
        "[bold]{:<30}[/bold] {}",
        "no_junction",
        cfg.no_junction
    )));
    console.println(&Text::from_markup(&format!(
        "[bold]{:<30}[/bold] {}",
        "root_path",
        cfg.root_path.as_deref().unwrap_or("(未设置)")
    )));
    console.println(&Text::from_markup(&format!(
        "[bold]{:<30}[/bold] {}",
        "auto_cleanup_days",
        cfg.auto_cleanup_days
    )));
    console.println(&Text::from_markup(&format!(
        "[bold]{:<30}[/bold] {}",
        "health_check_interval_days",
        cfg.health_check_interval_days
    )));

    Ok(())
}

/// config set — 修改配置项
fn cmd_set(session: &Session, key: &str, value: &str) -> anyhow::Result<()> {
    {
        let mut cfg = session.config_mut();

        match key {
            "proxy" => {
                cfg.proxy = if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                };
            }
            "mirror" => {
                cfg.mirror = if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                };
            }
            "aria2_enabled" => {
                cfg.aria2_enabled = parse_bool(value)?;
            }
            "no_junction" => {
                cfg.no_junction = parse_bool(value)?;
            }
            "root_path" => {
                cfg.root_path = if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                };
            }
            "auto_cleanup_days" => {
                cfg.auto_cleanup_days = value.parse().map_err(|_| {
                    anyhow::anyhow!("'{}' 不是有效的数字", value)
                })?;
            }
            "health_check_interval_days" => {
                cfg.health_check_interval_days = value.parse().map_err(|_| {
                    anyhow::anyhow!("'{}' 不是有效的数字", value)
                })?;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "未知配置项 '{}'。支持的配置项：proxy, mirror, aria2_enabled, no_junction, root_path, auto_cleanup_days, health_check_interval_days",
                    key
                ));
            }
        }
    }

    session.save_config()?;
    let mut console = Console::new();
    console.println(&Text::from_markup(&format!("[green]✔[/green] 配置 '{}' 已更新为 '{}'", key, value)));

    Ok(())
}

/// 解析布尔值（支持 true/false/1/0/yes/no）
fn parse_bool(value: &str) -> anyhow::Result<bool> {
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err(anyhow::anyhow!(
            "'{}' 不是有效的布尔值（应为 true/false/1/0/yes/no）",
            value
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;

    fn test_session(dir: &std::path::Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    #[test]
    fn config_list_shows_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            subcmd: ConfigCmd::List,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn config_set_proxy() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            subcmd: ConfigCmd::Set {
                key: "proxy".into(),
                value: "http://127.0.0.1:7890".into(),
            },
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());

        // 验证配置已更新（注意：session 内部的 RefCell 已被修改）
        // 但由于 config_path 为 None（with_config 不绑定路径），save_config 不会实际写文件
    }

    #[test]
    fn config_set_invalid_key() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            subcmd: ConfigCmd::Set {
                key: "nonexistent".into(),
                value: "test".into(),
            },
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未知配置项"));
    }
}
