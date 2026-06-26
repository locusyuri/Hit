//! `hit status` — 查看系统状态

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 状态参数
#[derive(ClapArgs, Debug)]
pub struct Args;

/// 执行状态检查
pub fn execute(_args: &Args, session: &Session) -> anyhow::Result<()> {
    // 已安装软件数
    let db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;
    let installed_count = db.package_count();

    // Bucket 数量
    let buckets = hit_core::bucket::list_buckets(session)?;
    let bucket_count = buckets.len();

    // 可用软件总数
    let index = hit_core::bucket::index::build_index(session)?;
    let total_packages = index.total_packages();

    // 缓存统计
    let cache_entries = hit_core::download::cache::list_cache(session)?;
    let cache_count = cache_entries.len();
    let cache_size: u64 = cache_entries.iter().map(|e| e.size).sum();

    // 输出
    println!(
        "{} {}",
        "Hit".bold().cyan(),
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!(
        "  {:<16} {}",
        "已安装软件:".bold(),
        installed_count
    );
    println!(
        "  {:<16} {}",
        "Bucket 数量:".bold(),
        bucket_count
    );
    println!(
        "  {:<16} {}",
        "可用软件总数:".bold(),
        total_packages
    );
    println!(
        "  {:<16} {} ({})",
        "缓存文件:".bold(),
        cache_count,
        format_bytes(cache_size)
    );
    println!(
        "  {:<16} {}",
        "根目录:".bold(),
        session.root_path().display()
    );

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
    fn status_empty_system() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args;

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
