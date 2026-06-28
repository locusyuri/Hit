//! `hit install` — 安装软件包
//!
//! 支持输入格式：`git`、`main/git`（指定 bucket）、`git@2.45.1`（版本约束，暂不支持）。

use std::sync::atomic::AtomicBool;

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;
use hit_core::bucket::index::{build_index, SoftwareIndex};
use hit_core::manifest::{parse_str, Arch, Manifest};

/// 安装参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要安装的软件名（支持多个，格式：name / bucket/name / name@version）
    pub apps: Vec<String>,

    /// 强制重装（即使已安装）
    #[arg(short, long)]
    pub force: bool,

    /// 指定架构（64bit / 32bit / arm64）
    #[arg(short, long)]
    pub arch: Option<String>,
}

/// 用户输入解析结果
struct AppSpec {
    bucket: Option<String>,
    name: String,
    version: Option<String>,
}

/// 解析用户输入：`name` / `bucket/name` / `name@version`
fn parse_app_spec(input: &str) -> AppSpec {
    // 处理 @version 约束（先提取）
    let (input, version) = match input.rsplit_once('@') {
        Some((rest, v)) if !v.is_empty() && rest.contains('/') => (rest, Some(v.to_string())),
        Some((rest, v)) if !v.is_empty() => (rest, Some(v.to_string())),
        _ => (input, None),
    };

    // 处理 bucket/name 格式
    if let Some((bucket, name)) = input.split_once('/') {
        AppSpec {
            bucket: Some(bucket.to_string()),
            name: name.to_string(),
            version,
        }
    } else {
        AppSpec {
            bucket: None,
            name: input.to_string(),
            version,
        }
    }
}

/// 从 bucket 索引中查找软件并加载 manifest
///
/// 流程：build_index → find → 读取文件 → parse_str
fn find_manifest(
    session: &Session,
    spec: &AppSpec,
) -> anyhow::Result<(String, String, Manifest)> {
    let index: SoftwareIndex = build_index(session)?;

    let mut candidates = index.find(&spec.name);

    // 如果指定了 bucket，过滤匹配的
    if let Some(ref bucket_name) = spec.bucket {
        candidates.retain(|p| p.bucket == *bucket_name);
    }

    if candidates.is_empty() {
        return Err(anyhow::anyhow!(
            "未找到软件 '{}'",
            spec.name
        ));
    }

    // 选择最佳匹配
    let summary = if candidates.len() > 1 && spec.bucket.is_none() {
        // 多个 bucket 有同名软件，按优先级自动选择
        index.best_match(&spec.name).unwrap()
    } else {
        candidates[0]
    };
    let bucket = summary.bucket.clone();
    let app = summary.name.clone();

    // 读取 manifest 文件（兼容两种 bucket 布局）
    let manifest_path = hit_core::bucket::manifest_path(session.buckets_path(), &bucket, &app);

    let content = std::fs::read_to_string(&manifest_path).map_err(|e| anyhow::anyhow!(
        "读取 manifest 失败 ({}): {e}",
        manifest_path.display()
    ))?;

    let manifest = parse_str(&content).map_err(|e| anyhow::anyhow!(
        "解析 manifest '{}' 失败: {e}",
        manifest_path.display()
    ))?;

    Ok((bucket, app, manifest))
}

/// 执行安装
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    if args.apps.is_empty() {
        return Err(anyhow::anyhow!("至少指定一个要安装的软件名"));
    }

    let arch = args
        .arch
        .as_deref()
        .map(|s| {
            Arch::from_scoop_key(s)
                .ok_or_else(|| anyhow::anyhow!("无效架构 '{}'（应为 64bit / 32bit / arm64）", s))
        })
        .transpose()?
        .or_else(Arch::current);

    for input in &args.apps {
        let spec = parse_app_spec(input);

        // 版本约束暂不支持
        if spec.version.is_some() {
            return Err(anyhow::anyhow!(
                "版本约束暂不支持（'{}' 中的 '@{}' 部分）",
                input,
                spec.version.as_deref().unwrap_or("")
            ));
        }

        println!("{} {} ...", "安装".cyan().bold(), spec.name);

        let (bucket, app, manifest) = find_manifest(session, &spec)?;

        let options = hit_core::install::InstallOptions {
            force: args.force,
            arch,
            no_deps: false,
            global: false,
            should_interrupt: AtomicBool::new(false),
        };

        let result = hit_core::install::install(session, &app, &manifest, &bucket, &options)?;

        println!(
            "{} {} {} 安装完成（{}）",
            "✔".green().bold(),
            app.bold(),
            result.version.green(),
            result.shims_created.len().to_string().green()
        );
    }

    Ok(())
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;

    /// 创建临时 session（用于测试）
    fn test_session(dir: &std::path::Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    /// 创建最小 manifest JSON 字符串
    fn minimal_manifest_json(version: &str) -> String {
        format!(
            r#"{{
            "version": "{version}",
            "description": "test app",
            "homepage": "https://example.com",
            "license": "MIT"
        }}"#
        )
    }

    // ── AppSpec 解析测试 ──

    #[test]
    fn parse_app_spec_simple_name() {
        let spec = parse_app_spec("git");
        assert_eq!(spec.name, "git");
        assert!(spec.bucket.is_none());
        assert!(spec.version.is_none());
    }

    #[test]
    fn parse_app_spec_bucket_name() {
        let spec = parse_app_spec("main/git");
        assert_eq!(spec.name, "git");
        assert_eq!(spec.bucket.as_deref(), Some("main"));
        assert!(spec.version.is_none());
    }

    #[test]
    fn parse_app_spec_with_version() {
        let spec = parse_app_spec("main/git@2.45.1");
        assert_eq!(spec.name, "git");
        assert_eq!(spec.bucket.as_deref(), Some("main"));
        assert_eq!(spec.version.as_deref(), Some("2.45.1"));
    }

    #[test]
    fn parse_app_spec_name_with_version() {
        let spec = parse_app_spec("git@2.45.1");
        assert_eq!(spec.name, "git");
        assert!(spec.bucket.is_none());
        assert_eq!(spec.version.as_deref(), Some("2.45.1"));
    }

    // ── find_manifest 测试 ──

    #[test]
    fn find_manifest_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        std::fs::create_dir_all(buckets_dir.join("main")).unwrap();

        let session = test_session(dir.path());
        let spec = AppSpec {
            bucket: None,
            name: "nonexistent".into(),
            version: None,
        };

        let result = find_manifest(&session, &spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到软件"));
    }

    #[test]
    fn find_manifest_found_in_bucket() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        let main_dir = buckets_dir.join("main");
        std::fs::create_dir_all(&main_dir).unwrap();

        std::fs::write(
            main_dir.join("git.json"),
            minimal_manifest_json("2.45.1"),
        )
        .unwrap();

        let session = test_session(dir.path());
        let spec = AppSpec {
            bucket: None,
            name: "git".into(),
            version: None,
        };

        let (bucket, app, manifest) = find_manifest(&session, &spec).unwrap();
        assert_eq!(bucket, "main");
        assert_eq!(app, "git");
        assert_eq!(manifest.version, "2.45.1");
    }

    #[test]
    fn find_manifest_ambiguous_selects_best() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");

        // 在两个 bucket 中创建同名 manifest（不同版本）
        for (bucket_name, version) in [("main", "2.45.1"), ("extras", "2.44.0")] {
            let bucket_dir = buckets_dir.join(bucket_name);
            std::fs::create_dir_all(&bucket_dir).unwrap();
            std::fs::write(
                bucket_dir.join("git.json"),
                minimal_manifest_json(version),
            )
            .unwrap();
        }

        let session = test_session(dir.path());
        let spec = AppSpec {
            bucket: None,
            name: "git".into(),
            version: None,
        };

        let result = find_manifest(&session, &spec);
        assert!(result.is_ok());
        let (bucket, _app, manifest) = result.unwrap();
        // 应自动选择 main bucket
        assert_eq!(bucket, "main");
        assert_eq!(manifest.version, "2.45.1");
    }

    #[test]
    fn find_manifest_specified_bucket() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");

        for (bucket_name, version) in [("main", "2.45.1"), ("extras", "3.0.0")] {
            let bucket_dir = buckets_dir.join(bucket_name);
            std::fs::create_dir_all(&bucket_dir).unwrap();
            std::fs::write(
                bucket_dir.join("git.json"),
                minimal_manifest_json(version),
            )
            .unwrap();
        }

        let session = test_session(dir.path());
        let spec = AppSpec {
            bucket: Some("extras".into()),
            name: "git".into(),
            version: None,
        };

        let (bucket, _app, manifest) = find_manifest(&session, &spec).unwrap();
        assert_eq!(bucket, "extras");
        assert_eq!(manifest.version, "3.0.0");
    }

    // ── execute 空输入测试 ──

    #[test]
    fn install_empty_apps_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: Vec::new(),
            force: false,
            arch: None,
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("至少指定一个"));
    }
}
