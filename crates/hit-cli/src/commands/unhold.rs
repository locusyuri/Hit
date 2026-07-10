//! `hit unhold` — 解除版本锁定

use clap::Args as ClapArgs;
use rusty_rich::{Console, Text};
use hit_common::Session;

/// unhold 参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要解除锁定的软件名称
    pub package: String,
}

/// 执行解除锁定
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let mut db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;

    let pkg = db
        .get_package_mut(&args.package)
        .ok_or_else(|| anyhow::anyhow!("'{}' 未安装", args.package))?;

    let mut console = Console::new();

    if !pkg.held {
        console.println(&Text::from_markup(&format!("[grey50]⏭[/grey50] '{}' 未处于锁定状态", args.package)));
        return Ok(());
    }

    pkg.held = false;
    db.save()?;

    console.println(&Text::from_markup(&format!("[green]🔓[/green] '{}' 已解除锁定", args.package)));

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
    fn unhold_nonexistent_errors() {
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
    fn unhold_clears_held_flag() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        // 插入已安装记录（held = true）
        {
            let mut db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
            let mut pkg = hit_core::store::InstalledPackage {
                version: "1.0".into(),
                bucket: "main".into(),
                ..Default::default()
            };
            pkg.held = true;
            db.insert_package("myapp".into(), pkg);
            db.save().unwrap();
        }

        let args = Args {
            package: "myapp".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());

        // 验证 held 标志已清除
        let db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
        assert!(!db.get_package("myapp").unwrap().held);
    }
}
