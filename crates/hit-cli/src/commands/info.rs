//! `hit info` — 查看软件包详情

use clap::Args as ClapArgs;
use hit_common::Session;
use owo_colors::OwoColorize;
use hit_core::bucket::index::build_index;
use hit_core::manifest::{parse_str, supported_architectures};

/// info 参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 软件名称
    pub name: String,

    /// 限定查找的 Bucket 名称
    #[arg(short, long)]
    pub bucket: Option<String>,
}

/// 执行 info 查询
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let index = build_index(session)?;

    let mut candidates = index.find(&args.name);

    if let Some(ref bucket_name) = args.bucket {
        candidates.retain(|p| p.bucket == *bucket_name);
    }

    if candidates.is_empty() {
        return Err(anyhow::anyhow!("未找到软件 '{}'", args.name));
    }

    if candidates.len() > 1 && args.bucket.is_none() {
        let buckets: Vec<&str> = candidates.iter().map(|p| p.bucket.as_str()).collect();
        return Err(anyhow::anyhow!(
            "在多个 bucket 中找到 '{}': {}，请使用 --bucket 指定",
            args.name,
            buckets.join(", ")
        ));
    }

    let summary = candidates[0];

    // 读取 manifest 文件（兼容两种 bucket 布局）
    let manifest_path = hit_core::bucket::manifest_path(session.buckets_path(), &summary.bucket, &summary.name);

    let content = std::fs::read_to_string(&manifest_path).map_err(|e| {
        anyhow::anyhow!(
            "读取 manifest 失败 ({}): {e}",
            manifest_path.display()
        )
    })?;

    let manifest = parse_str(&content).map_err(|e| {
        anyhow::anyhow!(
            "解析 manifest '{}' 失败: {e}",
            manifest_path.display()
        )
    })?;

    // 格式化输出
    println!("{}:        {}", "名称".bold(), summary.name);
    println!("{}:        {}", "版本".bold(), manifest.version);
    println!("{}:        {}", "描述".bold(), manifest.description);
    println!("{}:      {}", "主页".bold(), manifest.homepage);
    println!("{}:      {:?}", "许可证".bold(), manifest.license);

    // 架构
    let archs = supported_architectures(&manifest);
    if archs.is_empty() {
        println!("{}:    无", "架构".bold());
    } else {
        let arch_strs: Vec<&str> = archs.iter().map(|a| a.scoop_key()).collect();
        println!("{}:    {}", "架构".bold(), arch_strs.join(", "));
    }

    // 依赖
    if manifest.depends_list().is_empty() {
        println!("{}:      无", "依赖".bold());
    } else {
        println!("{}:      {}", "依赖".bold(), manifest.depends_list().join(", "));
    }

    // Bucket
    println!("{}:      {}", "Bucket".bold(), summary.bucket);

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
    fn info_not_found() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            name: "nonexistent".into(),
            bucket: None,
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到软件"));
    }

    #[test]
    fn info_found() {
        let dir = tempfile::tempdir().unwrap();
        let main_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&main_dir).unwrap();
        std::fs::write(
            main_dir.join("git.json"),
            minimal_manifest_json("2.45.1", "版本控制工具"),
        )
        .unwrap();

        let session = test_session(dir.path());
        let args = Args {
            name: "git".into(),
            bucket: None,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn info_ambiguous_bucket() {
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
            name: "git".into(),
            bucket: None,
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("多个 bucket"));
    }

    #[test]
    fn info_with_bucket() {
        let dir = tempfile::tempdir().unwrap();
        for (bucket_name, version) in [("main", "2.45.1"), ("extras", "3.0.0")] {
            let bucket_dir = dir.path().join("buckets").join(bucket_name);
            std::fs::create_dir_all(&bucket_dir).unwrap();
            std::fs::write(
                bucket_dir.join("git.json"),
                minimal_manifest_json(version, "版本控制工具"),
            )
            .unwrap();
        }

        let session = test_session(dir.path());
        let args = Args {
            name: "git".into(),
            bucket: Some("extras".into()),
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
