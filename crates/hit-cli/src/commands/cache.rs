//! `hit cache` — 缓存管理

use clap::{Args as ClapArgs, Subcommand};
use hit_common::Session;
use owo_colors::OwoColorize;

use crate::tables::{self, CacheRow};

/// 缓存管理参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 缓存子命令
    #[command(subcommand)]
    pub subcmd: CacheCmd,
}

/// 缓存子子命令
#[derive(Subcommand, Debug)]
pub enum CacheCmd {
    /// 列出缓存文件
    List,
    /// 清理缓存
    Clean {
        /// 指定软件名（留空清理全部）
        app: Option<String>,
    },
    /// 显示缓存目录路径
    Dir,
}

/// 执行缓存操作
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    match &args.subcmd {
        CacheCmd::List => cmd_list(session),
        CacheCmd::Clean { app } => cmd_clean(session, app.as_deref()),
        CacheCmd::Dir => cmd_dir(session),
    }
}

/// cache list — 列出缓存文件
fn cmd_list(session: &Session) -> anyhow::Result<()> {
    let entries = hit_core::download::cache::list_cache(session)?;

    if entries.is_empty() {
        println!("缓存为空");
        return Ok(());
    }

    let mut total_size: u64 = 0;
    let rows: Vec<CacheRow> = entries
        .iter()
        .map(|e| {
            total_size += e.size;
            CacheRow {
                app: e.app.clone(),
                version: e.version.clone(),
                size: format_bytes(e.size),
                path: e.path.display().to_string(),
            }
        })
        .collect();

    tables::print_cache_table(
        &rows,
        &format!("共 {} 个文件（{}）", rows.len(), format_bytes(total_size)),
    );

    Ok(())
}

/// cache clean — 清理缓存
fn cmd_clean(session: &Session, app: Option<&str>) -> anyhow::Result<()> {
    let count = hit_core::download::cache::remove_cache(session, app)?;

    if count == 0 {
        println!("没有可清理的缓存文件");
    } else {
        println!("{} 已清理 {} 个缓存文件", "✔".green(), count);
    }

    Ok(())
}

/// cache dir — 显示缓存目录路径
fn cmd_dir(session: &Session) -> anyhow::Result<()> {
    println!("{}", session.cache_path().display());
    Ok(())
}

/// 字节数格式化
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
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
    fn cache_list_empty() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            subcmd: CacheCmd::List,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn cache_dir_prints_path() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            subcmd: CacheCmd::Dir,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
