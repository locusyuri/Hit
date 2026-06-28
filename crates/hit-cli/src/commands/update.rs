//! `hit update` — 更新已安装软件
//!
//! 流程：更新所有 Bucket → 检查新版本 → 执行升级。

use std::sync::atomic::AtomicBool;

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;
use hit_core::bucket::index::build_index;
use hit_core::manifest::parse_str;
use hit_core::manifest::variables::Arch;

/// 更新参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要更新的软件名（留空配合 --all 使用）
    pub apps: Vec<String>,

    /// 更新所有已安装软件
    #[arg(short, long)]
    pub all: bool,

    /// 强制更新（忽略版本比较）
    #[arg(short, long)]
    pub force: bool,
}

/// 执行更新
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let should_interrupt = AtomicBool::new(false);

    // Step 1: 更新所有 Bucket
    println!("{} 正在更新 Bucket...", "刷新".cyan().bold());
    let buckets = hit_core::bucket::list_buckets(session)?;
    let mut updated = 0;
    for bucket in &buckets {
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
    println!("{} Bucket 更新完成（{updated}/{}）\n", "✔".green(), buckets.len());

    // Step 2: 检查新版本
    let db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;
    let index = build_index(session)?;

    // 确定要检查的 app 列表
    let apps_to_check: Vec<String> = if args.all || args.apps.is_empty() {
        db.list_packages().keys().cloned().collect()
    } else {
        args.apps.clone()
    };

    if apps_to_check.is_empty() {
        println!("没有已安装的软件");
        return Ok(());
    }

    let mut upgradable = Vec::new();

    for app_name in &apps_to_check {
        let pkg = match db.get_package(app_name) {
            Some(p) => p,
            None => {
                println!("  {} 未安装，跳过", app_name.dimmed());
                continue;
            }
        };

        // 在 bucket index 中查找
        let candidates = index.find(app_name);
        if candidates.is_empty() {
            continue;
        }

        // 使用已安装的 bucket（或第一个匹配）
        let summary = candidates
            .iter()
            .find(|p| p.bucket == pkg.bucket)
            .or(candidates.first())
            .unwrap();

        // 读取 manifest 获取完整信息（兼容两种 bucket 布局）
        let manifest_path = hit_core::bucket::manifest_path(session.buckets_path(), &summary.bucket, app_name);

        let content = match std::fs::read_to_string(&manifest_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let manifest = match parse_str(&content) {
            Ok(m) => m,
            Err(_) => continue,
        };

        // 版本比较
        if manifest.version == pkg.version && !args.force {
            continue;
        }

        upgradable.push((app_name.clone(), summary.bucket.clone(), manifest));
    }

    if upgradable.is_empty() {
        println!("所有软件已是最新版本");
        return Ok(());
    }

    // Step 3: 执行升级
    println!("{} 可升级 {} 个软件", "⬆".cyan().bold(), upgradable.len());

    let arch = Arch::current().unwrap_or(Arch::X86_64);
    let mut upgraded = 0;

    for (app_name, bucket, manifest) in &upgradable {
        println!("{} {} → {}", "升级".cyan().bold(), app_name, manifest.version.green());

        let options = hit_core::install::InstallOptions {
            force: true,
            arch: Some(arch),
            no_deps: false,
            global: false,
            should_interrupt: AtomicBool::new(false),
        };

        match hit_core::install::install(session, app_name, manifest, bucket, &options) {
            Ok(result) => {
                upgraded += 1;
                println!("  {} {} 升级完成", "✔".green(), result.version);
            }
            Err(e) => {
                println!("  {} 升级失败: {e}", "✘".red());
            }
        }
    }

    println!("\n{} 升级完成（{}/{}）", "✔".green(), upgraded, upgradable.len());

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
    fn update_no_args_empty_db() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: Vec::new(),
            all: false,
            force: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn update_all_flag_empty() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: Vec::new(),
            all: true,
            force: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
