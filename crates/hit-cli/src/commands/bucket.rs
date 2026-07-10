use std::sync::atomic::AtomicBool;

use clap::{Args as ClapArgs, Subcommand};
use hit_common::Session;
use rusty_rich::{Console, Text};

use crate::tables::{self, BucketRow};

#[derive(ClapArgs, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub subcmd: BucketCmd,
}

#[derive(Subcommand, Debug)]
pub enum BucketCmd {
    Add {
        name: String,
        #[arg(short, long)]
        url: Option<String>,
    },
    #[clap(alias = "rm")]
    Remove {
        name: String,
    },
    #[clap(alias = "ls")]
    List,
    Update {
        name: Option<String>,
    },
}

pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    match &args.subcmd {
        BucketCmd::Add { name, url } => cmd_add(session, name, url.as_deref()),
        BucketCmd::Remove { name } => cmd_remove(session, name),
        BucketCmd::List => cmd_list(session),
        BucketCmd::Update { name } => cmd_update(session, name.as_deref()),
    }
}

fn cmd_add(session: &Session, name: &str, url: Option<&str>) -> anyhow::Result<()> {
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

    let mut console = Console::new();
    console.println(&Text::from_markup(&format!(
        "[bold cyan]添加[/bold cyan] 正在添加 bucket '{}'...",
        name
    )));
    hit_core::bucket::clone_bucket(session, name, &bucket_url, &hit_core::bucket::CloneOptions::default(), &should_interrupt)?;

    console.println(&Text::from_markup(&format!(
        "[bold green]✔[/bold green] bucket '{}' 添加完成",
        name
    )));
    Ok(())
}

fn cmd_remove(session: &Session, name: &str) -> anyhow::Result<()> {
    let target = session.buckets_path().join(name);

    if !target.exists() {
        return Err(anyhow::anyhow!("Bucket '{name}' 不存在"));
    }

    let mut console = Console::new();
    console.println(&Text::from_markup(&format!(
        "[bold cyan]移除[/bold cyan] 正在移除 bucket '{}'...",
        name
    )));

    std::fs::remove_dir_all(&target).map_err(|e| {
        anyhow::anyhow!("删除 bucket 目录失败：{e}")
    })?;

    let mut db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;
    db.remove_bucket(name);
    db.save()?;

    console.println(&Text::from_markup(&format!(
        "[bold green]✔[/bold green] bucket '{}' 已移除",
        name
    )));
    Ok(())
}

fn cmd_list(session: &Session) -> anyhow::Result<()> {
    let buckets = hit_core::bucket::list_buckets(session)?;

    if buckets.is_empty() {
        let mut console = Console::new();
        console.println(&Text::from_markup("[yellow]没有已添加的 Bucket[/yellow]"));
        return Ok(());
    }

    let rows: Vec<BucketRow> = buckets
        .iter()
        .map(|b| {
            let count = b.manifest_count().unwrap_or(0);
            let desc = b
                .metadata
                .as_ref()
                .map(|m| m.description.as_str())
                .unwrap_or("");
            BucketRow {
                name: b.name.clone(),
                manifests: count.to_string(),
                description: desc.to_string(),
            }
        })
        .collect();

    tables::print_bucket_table(&rows, &format!("共 {} 个 Bucket", buckets.len()));
    Ok(())
}

fn cmd_update(session: &Session, name: Option<&str>) -> anyhow::Result<()> {
    let should_interrupt = AtomicBool::new(false);

    let buckets = hit_core::bucket::list_buckets(session)?;

    let to_update: Vec<_> = match name {
        Some(n) => buckets.into_iter().filter(|b| b.name == n).collect(),
        None => buckets,
    };

    if to_update.is_empty() {
        let mut console = Console::new();
        console.println(&Text::from_markup("[yellow]没有可更新的 Bucket[/yellow]"));
        return Ok(());
    }

    let mut console = Console::new();
    let mut updated = 0;
    for bucket in &to_update {
        match hit_core::bucket::pull_bucket(session, &bucket.name, &should_interrupt) {
            Ok(_path) => {
                updated += 1;
                console.println(&Text::from_markup(&format!(
                    "  [green]✔[/green] {}",
                    bucket.name
                )));
            }
            Err(e) => {
                console.println(&Text::from_markup(&format!(
                    "  [red]✘[/red] {} 失败: {}",
                    bucket.name, e
                )));
            }
        }
    }

    console.println(&Text::from_markup(&format!(
        "\n[green]✔[/green] Bucket 更新完成（{updated}/{}）",
        to_update.len()
    )));
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
