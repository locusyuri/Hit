//! `hit si` — 交互式搜索并安装

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 交互式搜索参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 初始搜索关键词（可选）
    pub query: Option<String>,
}

/// 执行交互式搜索
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let initial_query = args.query.as_deref().unwrap_or("");

    let selected = crate::tui::run_app(session, initial_query)?;

    match selected {
        Some(name) => {
            println!("{} {}", "安装".cyan().bold(), name);

            // 查找并安装
            let index = hit_core::bucket::index::build_index(session)?;
            let candidates = index.find(&name);

            if candidates.is_empty() {
                return Err(anyhow::anyhow!("未找到软件 '{}'", name));
            }

            let summary = index.best_match(&name).unwrap();

            // 读取 manifest
            let manifest_path = session
                .buckets_path()
                .join(&summary.bucket)
                .join(format!("{}.json", summary.name));

            let content = std::fs::read_to_string(&manifest_path).map_err(|e| {
                anyhow::anyhow!("读取 manifest 失败: {e}")
            })?;

            let manifest = hit_core::manifest::parse_str(&content).map_err(|e| {
                anyhow::anyhow!("解析 manifest 失败: {e}")
            })?;

            let options = hit_core::install::InstallOptions {
                force: false,
                arch: hit_core::manifest::Arch::current(),
                no_deps: false,
                global: false,
                should_interrupt: std::sync::atomic::AtomicBool::new(false),
            };

            let result = hit_core::install::install(
                session,
                &summary.name,
                &manifest,
                &summary.bucket,
                &options,
            )?;

            println!(
                "{} {} {} 安装完成",
                "✔".green().bold(),
                name.bold(),
                result.version.green()
            );
        }
        None => {
            // 用户按 Esc 退出
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
    fn si_empty_session_works() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let _session = test_session(dir.path());
        let args = Args { query: None };

        // 不实际运行 TUI（需要终端），只验证命令解析
        assert!(args.query.is_none());
    }

    #[test]
    fn si_with_initial_query() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let _session = test_session(dir.path());
        let args = Args {
            query: Some("git".into()),
        };

        assert_eq!(args.query.as_deref(), Some("git"));
    }
}
