//! `hit hold` — 版本锁定

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// hold 参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要锁定的软件名称
    pub package: String,
}

/// 执行版本锁定
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let mut db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;

    let pkg = db
        .get_package_mut(&args.package)
        .ok_or_else(|| anyhow::anyhow!("'{}' 未安装", args.package))?;

    if pkg.held {
        println!("{} '{}' 已经是锁定状态", "⏭".dimmed(), args.package);
        return Ok(());
    }

    pkg.held = true;
    db.save()?;

    println!(
        "{} '{}' 已锁定（update 时将跳过升级）",
        "🔒".green(),
        args.package
    );

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
    fn hold_nonexistent_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            package: "nonexistent".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未安装"));
    }

    #[test]
    fn hold_sets_held_flag() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        // 插入已安装记录
        {
            let mut db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
            db.insert_package(
                "myapp".into(),
                hit_core::store::InstalledPackage {
                    version: "1.0".into(),
                    bucket: "main".into(),
                    ..Default::default()
                },
            );
            db.save().unwrap();
        }

        let args = Args {
            package: "myapp".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());

        // 验证 held 标志
        let db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
        assert!(db.get_package("myapp").unwrap().held);
    }
}
