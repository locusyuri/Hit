//! `hit home` — 打开软件主页

use clap::Args as ClapArgs;
use rusty_rich::{Console, Text};
use hit_common::Session;
use hit_core::bucket::index::build_index;
use hit_core::manifest::parse_str;

/// home 参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 软件名称
    pub name: String,
}

/// 执行打开主页
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let index = build_index(session)?;
    let candidates = index.find(&args.name);

    if candidates.is_empty() {
        return Err(anyhow::anyhow!("未找到软件 '{}'", args.name));
    }

    let summary = candidates[0];

    // 读取 manifest
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

    if manifest.homepage.is_empty() {
        return Err(anyhow::anyhow!(
            "'{}' 没有设置主页 URL",
            args.name
        ));
    }

    let mut console = Console::new();
    console.println(&Text::from_markup(&format!("[bold cyan]打开[/bold cyan] {} → {}", args.name, manifest.homepage)));

    // Windows: 使用 cmd /c start 打开浏览器
    std::process::Command::new("cmd")
        .args(["/c", "start", &manifest.homepage])
        .spawn()
        .map_err(|e| anyhow::anyhow!("打开浏览器失败: {e}"))?;

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
    fn home_not_found_errors() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            name: "nonexistent".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到软件"));
    }

    #[test]
    fn home_empty_homepage_errors() {
        let dir = tempfile::tempdir().unwrap();
        let main_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&main_dir).unwrap();
        std::fs::write(
            main_dir.join("myapp.json"),
            r#"{"version":"1.0","description":"test","homepage":"","license":"MIT"}"#,
        )
        .unwrap();

        let session = test_session(dir.path());
        let args = Args {
            name: "myapp".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("没有设置主页"));
    }
}
