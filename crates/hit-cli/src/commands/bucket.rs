//! `hit bucket` — 管理 Bucket 仓库

use std::sync::atomic::AtomicBool;

use clap::{Args as ClapArgs, Subcommand};
use colored::Colorize;
use hit_common::Session;

/// Bucket 管理参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Bucket 子命令
    #[command(subcommand)]
    pub subcmd: BucketCmd,
}

/// Bucket 子子命令
#[derive(Subcommand, Debug)]
pub enum BucketCmd {
    /// 添加 Bucket
    Add {
        /// Bucket 名称
        name: String,
        /// Bucket Git 仓库 URL（可选，默认使用已知 Bucket）
        url: Option<String>,
    },
    /// 移除 Bucket
    #[clap(alias = "rm")]
    Remove {
        /// Bucket 名称
        name: String,
    },
    /// 列出所有 Bucket
    #[clap(alias = "ls")]
    List,
    /// 更新 Bucket
    Update {
        /// 指定 Bucket 名称（留空则更新全部）
        name: Option<String>,
    },
}

/// 执行 Bucket 操作
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    match &args.subcmd {
        BucketCmd::Add { name, url } => cmd_add(session, name, url.as_deref()),
        BucketCmd::Remove { name } => cmd_remove(session, name),
        BucketCmd::List => cmd_list(session),
        BucketCmd::Update { name } => cmd_update(session, name.as_deref()),
    }
}

/// bucket add — 添加新 Bucket
fn cmd_add(session: &Session, name: &str, url: Option<&str>) -> anyhow::Result<()> {
    // 查找 URL：优先使用参数，其次查找已知 bucket
    let bucket_url = match url {
        Some(u) => u.to_string(),
        None => hit_core::bucket::resolve_known_bucket(name)
            .ok_or_else(|| anyhow::anyhow!(
                "未知 bucket '{name}'，请提供 Git 仓库 URL\n  示例：hit bucket add {name} https://github.com/<user>/<bucket>.git"
            ))?
            .to_string(),
    };

    let should_interrupt = AtomicBool::new(false);
    let target = session.buckets_path().join(name);

    if target.exists() {
        return Err(anyhow::anyhow!("Bucket '{name}' 已存在"));
    }

    println!("{} 正在添加 bucket '{}'...", "添加".cyan().bold(), name);
    hit_core::bucket::clone_bucket(session, name, &bucket_url, &hit_core::bucket::CloneOptions::default(), &should_interrupt)?;

    println!("{} bucket '{}' 添加完成", "✔".green().bold(), name);
    Ok(())
}

/// bucket remove — 移除 Bucket
fn cmd_remove(session: &Session, name: &str) -> anyhow::Result<()> {
    let target = session.buckets_path().join(name);

    if !target.exists() {
        return Err(anyhow::anyhow!("Bucket '{name}' 不存在"));
    }

    println!("{} 正在移除 bucket '{}'...", "移除".cyan().bold(), name);

    std::fs::remove_dir_all(&target).map_err(|e| {
        anyhow::anyhow!("删除 bucket 目录失败：{e}")
    })?;

    // 从 db.json 移除记录
    let mut db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;
    db.remove_bucket(name);
    db.save()?;

    println!("{} bucket '{}' 已移除", "✔".green().bold(), name);
    Ok(())
}

/// bucket list — 列出所有 Bucket
fn cmd_list(session: &Session) -> anyhow::Result<()> {
    let buckets = hit_core::bucket::list_buckets(session)?;

    if buckets.is_empty() {
        println!("没有已添加的 Bucket");
        return Ok(());
    }

    println!(
        "{}  {}  {}",
        pad("名称", 20).bold(),
        pad("Manifest", 10).bold(),
        "描述".bold()
    );

    for bucket in &buckets {
        let count = bucket.manifest_count().unwrap_or(0);
        let desc = bucket
            .metadata
            .as_ref()
            .map(|m| m.description.as_str())
            .unwrap_or("");
        println!(
            "{}  {}  {}",
            pad(&bucket.name, 20),
            pad(&count.to_string(), 10),
            desc
        );
    }

    println!("\n共 {} 个 Bucket", buckets.len());
    Ok(())
}

/// 按显示宽度右补空格对齐（CJK 全角字符占 2 列）
fn pad(s: &str, width: usize) -> String {
    let dw = display_width(s);
    if dw >= width {
        s.to_string()
    } else {
        format!("{s}{}", " ".repeat(width - dw))
    }
}

/// 计算字符串的终端显示宽度（CJK 全角字符占 2，其余占 1）
fn display_width(s: &str) -> usize {
    s.chars().map(|c| if (c as u32) > 0x2E80 { 2 } else { 1 }).sum()
}

/// bucket update — 更新 Bucket
fn cmd_update(session: &Session, name: Option<&str>) -> anyhow::Result<()> {
    let should_interrupt = AtomicBool::new(false);

    let buckets = hit_core::bucket::list_buckets(session)?;

    let to_update: Vec<_> = match name {
        Some(n) => buckets.into_iter().filter(|b| b.name == n).collect(),
        None => buckets,
    };

    if to_update.is_empty() {
        println!("没有可更新的 Bucket");
        return Ok(());
    }

    let mut updated = 0;
    for bucket in &to_update {
        match hit_core::bucket::pull_bucket(session, &bucket.name, &should_interrupt) {
            Ok(_path) => {
                updated += 1;
                println!("  {} {}", "✔".green(), bucket.name);
            }
            Err(e) => {
                println!("  {} {} 失败: {e}", "✘".red(), bucket.name);
            }
        }
    }

    println!("\n{} Bucket 更新完成（{updated}/{}）", "✔".green(), to_update.len());
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
    fn bucket_list_empty() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            subcmd: BucketCmd::List,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn bucket_list_with_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        std::fs::create_dir_all(buckets_dir.join("main")).unwrap();
        std::fs::create_dir_all(buckets_dir.join("extras")).unwrap();

        let session = test_session(dir.path());
        let args = Args {
            subcmd: BucketCmd::List,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn bucket_add_already_exists() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        std::fs::create_dir_all(buckets_dir.join("main")).unwrap();

        let session = test_session(dir.path());
        let args = Args {
            subcmd: BucketCmd::Add {
                name: "main".into(),
                url: Some("https://example.com/repo.git".into()),
            },
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("已存在"));
    }

    #[test]
    fn bucket_remove_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            subcmd: BucketCmd::Remove {
                name: "nonexistent".into(),
            },
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("不存在"));
    }
}
