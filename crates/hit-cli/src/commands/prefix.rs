//! `hit prefix` — 显示安装路径

use clap::Args as ClapArgs;
use hit_common::Session;

/// prefix 参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 软件名称（留空显示根目录）
    pub app: Option<String>,
}

/// 执行显示路径
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    match &args.app {
        Some(app) => {
            let path = session.apps_path().join(app);
            if path.exists() {
                println!("{}", path.display());
            } else {
                return Err(anyhow::anyhow!("'{}' 未安装", app));
            }
        }
        None => {
            println!("{}", session.root_path().display());
        }
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
    fn prefix_shows_root() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args { app: None };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn prefix_with_app() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        // 创建 app 目录
        std::fs::create_dir_all(session.apps_path().join("myapp")).unwrap();

        let args = Args {
            app: Some("myapp".into()),
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn prefix_nonexistent_app_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            app: Some("nonexistent".into()),
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未安装"));
    }
}
