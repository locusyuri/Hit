//! `hit cleanup` — 清理旧版本与缓存

use clap::Args as ClapArgs;
use hit_common::Session;
use owo_colors::OwoColorize;

/// 清理参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要清理的软件名（留空配合 --all 使用）
    pub apps: Vec<String>,

    /// 清理所有旧版本
    #[arg(short, long)]
    pub all: bool,

    /// 同时清理下载缓存
    #[arg(short, long)]
    pub cache: bool,
}

/// 执行清理
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let mut total_cleaned = 0;

    // 清理旧版本
    if args.all || !args.apps.is_empty() {
        let apps: Vec<String> = if args.all {
            // 列出所有已安装 app
            let apps_dir = session.apps_path();
            if apps_dir.exists() {
                std::fs::read_dir(apps_dir)
                    .map_err(|e| anyhow::anyhow!("读取 apps 目录失败: {e}"))?
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            args.apps.clone()
        };

        let db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;

        for app_name in &apps {
            let app_dir = session.apps_path().join(app_name);
            if !app_dir.exists() {
                continue;
            }

            // 获取当前版本
            let current_version = db.get_package(app_name).map(|p| p.version.as_str());

            // 列出所有版本目录
            let versions: Vec<_> = std::fs::read_dir(&app_dir)
                .map_err(|e| anyhow::anyhow!("读取 {} 目录失败: {e}", app_dir.display()))?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter(|e| {
                    let name = e.file_name().to_string_lossy().into_owned();
                    // 排除 current junction 和非版本目录
                    name != "current"
                        && name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                })
                .collect();

            for entry in versions {
                let dir_name = entry.file_name().to_string_lossy().into_owned();

                // 保留当前版本
                if Some(dir_name.as_str()) == current_version {
                    continue;
                }

                // 删除旧版本
                let version_path = entry.path();
                match std::fs::remove_dir_all(&version_path) {
                    Ok(()) => {
                        total_cleaned += 1;
                        println!(
                            "  {} {} {}",
                            "删除".dimmed(),
                            app_name,
                            dir_name.dimmed()
                        );
                    }
                    Err(e) => {
                        println!(
                            "  {} {} {}: {e}",
                            "跳过".yellow(),
                            app_name,
                            dir_name
                        );
                    }
                }
            }
        }
    }

    // 清理缓存
    let _cache_cleaned = if args.cache {
        let count = hit_core::download::cache::remove_cache(session, None)?;
        if count > 0 {
            println!("{} 已清理 {} 个缓存文件", "✔".green(), count);
        } else {
            println!("{} 没有缓存文件需要清理", "✔".green());
        }
        count
    } else {
        0
    };

    // 输出总结（仅清理旧版本时）
    if args.cache && total_cleaned == 0 {
        // 仅清理缓存，已在上面处理输出
    } else if total_cleaned == 0 && !args.cache {
        println!("没有需要清理的内容");
    } else if total_cleaned > 0 {
        println!("{} 已清理 {} 个旧版本", "✔".green(), total_cleaned);
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
    fn cleanup_empty() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: Vec::new(),
            all: false,
            cache: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn cleanup_all_empty_apps() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("apps")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: Vec::new(),
            all: true,
            cache: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
