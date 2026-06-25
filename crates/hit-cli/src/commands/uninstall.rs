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

/// 执行卸载
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    if args.apps.is_empty() {
        return Err(anyhow::anyhow!("至少指定一个要卸载的软件名"));
    }

    for app in &args.apps {
        // 检查安装状态
        let db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;
        if !db.is_installed(app) {
            return Err(anyhow::anyhow!("'{app}' 未安装"));
        }

        println!("{} {} ...", "卸载".cyan().bold(), app);

        hit_core::install::uninstall(session, app)?;

        // --purge：删除 persist 数据
        if args.purge {
            let persist_dir = session.persist_path().join(app);
            if persist_dir.exists() {
                std::fs::remove_dir_all(&persist_dir).ok();
            }
        }

        println!("{} {} 已卸载", "✔".green().bold(), app.bold());
    }

    Ok(())
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
    fn uninstall_empty_apps_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: Vec::new(),
            purge: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("至少指定一个"));
    }

    #[test]
    fn uninstall_nonexistent_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: vec!["nonexistent".into()],
            purge: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未安装"));
    }

    #[test]
    fn purge_flag_stored() {
        // 验证 purge 标志正确传递
        let args = Args {
            apps: vec!["myapp".into()],
            purge: true,
        };
        assert!(args.purge);

        let args2 = Args {
            apps: vec!["myapp".into()],
            purge: false,
        };
        assert!(!args2.purge);
    }
}
