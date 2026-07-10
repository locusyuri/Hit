//! `hit reset` — 版本切换

use clap::Args as ClapArgs;
use hit_common::Session;
use owo_colors::OwoColorize;

/// 版本切换参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 软件名称
    pub app: String,

    /// 目标版本号
    pub version: String,
}

/// 执行版本切换
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let version_dir = session.apps_path().join(&args.app).join(&args.version);

    if !version_dir.exists() {
        return Err(anyhow::anyhow!(
            "版本 '{}' 不存在（{}）",
            args.version,
            args.app
        ));
    }

    println!(
        "{} {} → {} ...",
        "切换".cyan().bold(),
        args.app,
        args.version.green()
    );

    hit_core::install::reset_version(session, &args.app, &args.version)?;

    println!(
        "{} {} 已切换到 {}",
        "✔".green().bold(),
        args.app,
        args.version.green()
    );

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
    fn reset_nonexistent_version_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            app: "myapp".into(),
            version: "9.9.9".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("不存在"));
    }
}
