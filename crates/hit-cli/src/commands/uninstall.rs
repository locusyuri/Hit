//! `hit uninstall` — 卸载软件

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// 卸载参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 要卸载的软件名（支持多个）
    pub apps: Vec<String>,

    /// 同时删除 persist 数据
    #[arg(short, long)]
    pub purge: bool,
}

/// 执行卸载
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    if args.apps.is_empty() {
        return Err(anyhow::anyhow!("至少指定一个要卸载的软件名"));
    }

    for app in &args.apps {
        // 检查安装状态
        let db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;
        if !db.is_installed(app) {
            return Err(anyhow::anyhow!("'{app}' 未安装"));
        }

        println!("{} {} ...", "卸载".cyan().bold(), app);

        hit_core::install::uninstall(session, app)?;

        // --purge：删除 persist 数据
        if args.purge {
            let persist_dir = session.persist_path().join(app);
            if persist_dir.exists() {
                std::fs::remove_dir_all(&persist_dir).ok();
            }
        }

        println!("{} {} 已卸载", "✔".green().bold(), app.bold());
    }

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
    fn uninstall_empty_apps_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: Vec::new(),
            purge: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("至少指定一个"));
    }

    #[test]
    fn uninstall_nonexistent_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args {
            apps: vec!["nonexistent".into()],
            purge: false,
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未安装"));
    }

    #[test]
    fn uninstall_multiple_apps() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        let main_dir = buckets_dir.join("main");
        std::fs::create_dir_all(&main_dir).unwrap();

        let session = test_session(dir.path());

        // 模拟已安装记录（写入 db.json）
        {
            let mut db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
            db.insert_package(
                "app1".into(),
                hit_core::store::InstalledPackage {
                    version: "1.0".into(),
                    bucket: "main".into(),
                    ..Default::default()
                },
            );
            db.insert_package(
                "app2".into(),
                hit_core::store::InstalledPackage {
                    version: "2.0".into(),
                    bucket: "main".into(),
                    ..Default::default()
                },
            );
            db.save().unwrap();
        }

        // 创建最小 app 目录（模拟已安装）
        for app_name in &["app1", "app2"] {
            let app_dir = dir.path().join("apps").join(app_name);
            std::fs::create_dir_all(app_dir.join("current")).unwrap();
            // 写一个标记文件确保目录非空
            std::fs::write(app_dir.join("current").join("test.txt"), "data").unwrap();
        }

        let args = Args {
            apps: vec!["app1".into(), "app2".into()],
            purge: false,
        };

        // 执行卸载
        let result = execute(&args, &session);
        assert!(result.is_ok(), "卸载应成功: {:?}", result);

        // 验证 db 中记录已删除
        let db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
        assert!(!db.is_installed("app1"));
        assert!(!db.is_installed("app2"));
    }

    #[test]
    fn purge_removes_persist_dir() {
        let dir = tempfile::tempdir().unwrap();
        let buckets_dir = dir.path().join("buckets");
        std::fs::create_dir_all(buckets_dir.join("main")).unwrap();

        let session = test_session(dir.path());

        // 创建 persist 目录
        let persist_dir = session.persist_path().join("myapp");
        std::fs::create_dir_all(&persist_dir).unwrap();
        std::fs::write(persist_dir.join("data.txt"), "config").unwrap();

        // 创建安装记录和最小 app 目录
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
        let app_dir = dir.path().join("apps").join("myapp");
        std::fs::create_dir_all(app_dir.join("current")).unwrap();

        let args = Args {
            apps: vec!["myapp".into()],
            purge: true,
        };

        let result = execute(&args, &session);
        assert!(result.is_ok(), "卸载应成功: {:?}", result);

        // 验证 persist 目录已删除
        assert!(!persist_dir.exists(), "purge 后 persist 目录应已删除");
    }
}
