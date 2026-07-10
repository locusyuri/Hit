//! `hit status` — 查看系统状态

use clap::Args as ClapArgs;
use hit_common::Session;
use owo_colors::OwoColorize;

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

    // 输出（使用 Unicode 显示宽度对齐，中文字符占 2 列）
    println!(
        "{} {}",
        "Hit".bold().cyan(),
        env!("CARGO_PKG_VERSION")
    );
    println!();
    let rows: Vec<(&str, String)> = vec![
        ("已安装软件:", installed_count.to_string()),
        ("Bucket 数量:", bucket_count.to_string()),
        ("可用软件总数:", total_packages.to_string()),
        ("缓存文件:", format!("{} ({})", cache_count, format_bytes(cache_size))),
        ("根目录:", session.root_path().display().to_string()),
    ];
    // 计算标签列最大显示宽度
    let max_label_w = rows.iter().map(|(l, _)| display_width(l)).max().unwrap_or(0);
    for (label, value) in &rows {
        let pad = max_label_w - display_width(label);
        println!("  {}{}  {}", label.bold(), " ".repeat(pad), value);
    }

    Ok(())
}

/// 计算字符串的终端显示宽度（CJK 全角字符占 2，其余占 1）。
/// 避免引入 unicode-width 依赖的轻量实现。
fn display_width(s: &str) -> usize {
    s.chars().map(|c| if (c as u32) > 0x2E80 { 2 } else { 1 }).sum()
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
