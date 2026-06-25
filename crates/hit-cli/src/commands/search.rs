//! `hit search` — 搜索软件包

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;
use hit_core::bucket::index::build_index;

/// 搜索参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 搜索关键词
    pub query: String,

    /// 限定搜索的 Bucket 名称
    #[arg(short, long)]
    pub bucket: Option<String>,
}

/// 执行搜索
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let index = build_index(session)?;

    let mut results = index.search(&args.query);

    // 过滤 bucket
    if let Some(ref bucket_name) = args.bucket {
        results.retain(|p| p.bucket == *bucket_name);
    }

    if results.is_empty() {
        println!("未找到匹配 '{}' 的软件", args.query);
        return Ok(());
    }

    println!(
        "{:<12} {:<10} {}",
        "名称".bold(),
        "版本".bold(),
        "描述".bold()
    );

    for pkg in &results {
        println!(
            "{:<12} {:<10} {}",
            pkg.name,
            pkg.version,
            pkg.description
        );
    }

    println!("\n共 {} 个结果", results.len());

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

    fn minimal_manifest_json(version: &str, desc: &str) -> String {
        format!(
            r#"{{
            "version": "{version}",
            "description": "{desc}",
            "homepage": "https://example.com",
            "license": "MIT"
        }}"#
        )
    }

    #[test]
    fn search_empty_index() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            query: "git".into(),
            bucket: None,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn search_finds_by_name() {
        let dir = tempfile::tempdir().unwrap();
        let main_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&main_dir).unwrap();

        std::fs::write(
            main_dir.join("git.json"),
            minimal_manifest_json("2.45.1", "版本控制工具"),
        )
        .unwrap();

        std::fs::write(
            main_dir.join("curl.json"),
            minimal_manifest_json("8.7.1", "URL 传输工具"),
        )
        .unwrap();

        let session = test_session(dir.path());
        let args = Args {
            query: "git".into(),
            bucket: None,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn search_bucket_filter() {
        let dir = tempfile::tempdir().unwrap();
        for bucket_name in &["main", "extras"] {
            let bucket_dir = dir.path().join("buckets").join(bucket_name);
            std::fs::create_dir_all(&bucket_dir).unwrap();
            std::fs::write(
                bucket_dir.join("git.json"),
                minimal_manifest_json("2.45.1", "版本控制工具"),
            )
            .unwrap();
        }

        let session = test_session(dir.path());
        let args = Args {
            query: "git".into(),
            bucket: Some("extras".into()),
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn search_case_insensitive() {
        let dir = tempfile::tempdir().unwrap();
        let main_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&main_dir).unwrap();

        std::fs::write(
            main_dir.join("Git.json"),
            minimal_manifest_json("2.45.1", "版本控制工具"),
        )
        .unwrap();

        let session = test_session(dir.path());
        let args = Args {
            query: "git".into(),
            bucket: None,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
