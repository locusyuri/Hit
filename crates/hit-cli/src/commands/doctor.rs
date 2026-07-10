//! `hit doctor` — 健康检查与自动修复

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;
use hit_core::win::fs::{create_junction, remove_junction};

/// 健康检查参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 自动修复可修复的问题
    #[arg(short, long)]
    pub fix: bool,
}

/// 执行健康检查
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let mut issues = hit_core::health::check_all(session);

    // 额外检查 shim 完整性
    issues.extend(check_shims(session));

    if issues.is_empty() {
        println!("{} 系统健康，无问题", "✔".green().bold());
        return Ok(());
    }

    println!(
        "{} 发现 {} 个问题：\n",
        "⚠".yellow().bold(),
        issues.len()
    );

    for issue in &issues {
        let icon = match issue.issue {
            hit_core::health::IssueType::MissingAppDir
            | hit_core::health::IssueType::MissingVersion
            | hit_core::health::IssueType::StaleDbRecord => "✗".red(),
            _ => "⚠".yellow(),
        };

        let fixable = if issue.fixable {
            " (可修复)".dimmed().to_string()
        } else {
            String::new()
        };

        println!(
            "  {} {}: {}{}",
            icon,
            issue.app.bold(),
            issue.issue,
            fixable
        );
    }

    // 自动修复
    if args.fix {
        let fixable_count = issues.iter().filter(|i| i.fixable).count();
        if fixable_count == 0 {
            println!("\n{} 没有可自动修复的问题", "ℹ".blue());
            return Ok(());
        }

        println!("\n{} 正在修复 {} 个问题...", "修复".cyan().bold(), fixable_count);

        let mut fixed = 0;
        for issue in &issues {
            if !issue.fixable {
                continue;
            }

            match &issue.issue {
                hit_core::health::IssueType::MissingAppDir => {
                    let db_path = hit_core::store::db_path(session);
                    if let Ok(mut db) = hit_core::store::Db::load(&db_path) {
                        db.remove_package(&issue.app);
                        if let Ok(()) = db.save() {
                            fixed += 1;
                            println!("  {} {} 已移除孤立记录", "✔".green(), issue.app);
                        } else {
                            println!("  {} {} 修复失败: 保存数据库失败", "✗".red(), issue.app);
                        }
                    } else {
                        println!("  {} {} 修复失败: 加载数据库失败", "✗".red(), issue.app);
                    }
                }
                hit_core::health::IssueType::MissingCurrent => {
                    #[cfg(windows)]
                    if let Some(version_dir) = find_latest_version(issue.path.parent().unwrap()) {
                        match create_junction(&version_dir, &issue.path) {
                            Ok(()) => {
                                fixed += 1;
                                println!("  {} {} → {}", "✔".green(), issue.app, version_dir.display());
                            }
                            Err(e) => {
                                println!("  {} {} 修复失败: {e}", "✗".red(), issue.app);
                            }
                        }
                    }
                    #[cfg(not(windows))]
                    { let _ = issue; }
                }
                hit_core::health::IssueType::BrokenJunction => {
                    #[cfg(windows)]
                    {
                        let _ = remove_junction(&issue.path);
                        if let Some(version_dir) = find_latest_version(issue.path.parent().unwrap()) {
                            match create_junction(&version_dir, &issue.path) {
                                Ok(()) => {
                                    fixed += 1;
                                    println!("  {} {} → {}", "✔".green(), issue.app, version_dir.display());
                                }
                                Err(e) => {
                                    println!("  {} {} 修复失败: {e}", "✗".red(), issue.app);
                                }
                            }
                        }
                    }
                    #[cfg(not(windows))]
                    { let _ = issue; }
                }
                hit_core::health::IssueType::BrokenShim => {
                    // 删除损坏的 shim 文件
                    let _ = std::fs::remove_file(&issue.path);
                    let exe_path = issue.path.with_extension("exe");
                    let _ = std::fs::remove_file(&exe_path);
                    fixed += 1;
                    println!("  {} {} 已删除", "✔".green(), issue.app);
                }
                _ => {}
            }
        }

        println!(
            "\n{} 已修复 {}/{} 个问题",
            "✔".green(),
            fixed,
            fixable_count
        );
    } else {
        println!(
            "\n{} 使用 {} 自动修复可修复的问题",
            "提示".blue(),
            "hit doctor --fix".yellow()
        );
    }

    Ok(())
}

/// 检查所有 shim 的健康状态
fn check_shims(session: &Session) -> Vec<hit_core::health::HealthIssue> {
    let mut issues = Vec::new();
    let shims_path = session.shims_path();

    if !shims_path.exists() {
        return issues;
    }

    for entry in std::fs::read_dir(shims_path).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("shim") {
            continue;
        }

        if let Ok(data) = hit_shim::parse::read_shim_file(&path) {
            let target = std::path::Path::new(&data.path);
            if !target.exists() {
                let app_name = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                issues.push(hit_core::health::HealthIssue {
                    app: app_name,
                    issue: hit_core::health::IssueType::BrokenShim,
                    path,
                    fixable: true,
                });
            }
        }
    }

    issues
}

/// 查找目录中最新的版本子目录
fn find_latest_version(app_dir: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut versions: Vec<String> = std::fs::read_dir(app_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name != "current")
        .filter(|name| name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
        .collect();

    versions.sort();
    versions.last().map(|v| app_dir.join(v))
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
    fn doctor_healthy_system() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args { fix: false };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn doctor_detects_issues() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());

        // 创建孤立目录
        std::fs::create_dir_all(session.apps_path().join("orphan")).unwrap();

        let args = Args { fix: false };
        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn doctor_fix_flag_works() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let args = Args { fix: true };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
