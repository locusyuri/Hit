//! `hit list` — 列出已安装软件

use clap::Args as ClapArgs;
use hit_common::Session;

use crate::tables::{self, ListRow};

/// 列表参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 按名称过滤（可选）
    pub filter: Option<String>,
}

/// 执行列表
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let db = hit_core::store::Db::load(&hit_core::store::db_path(session))?;

    let packages = db.list_packages();

    // 按 filter 过滤
    let filtered: Vec<_> = packages
        .iter()
        .filter(|(name, _)| match &args.filter {
            Some(f) => name.contains(f.as_str()),
            None => true,
        })
        .collect();

    if filtered.is_empty() {
        if args.filter.is_some() {
            println!(
                "没有匹配 '{}' 的已安装软件",
                args.filter.as_deref().unwrap_or("")
            );
        } else {
            println!("没有已安装的软件");
        }
        return Ok(());
    }

    let rows: Vec<ListRow> = filtered
        .iter()
        .map(|(name, pkg)| ListRow {
            name: (*name).clone(),
            version: pkg.version.clone(),
            architecture: pkg.architecture.clone(),
            bucket: pkg.bucket.clone(),
            install_date: pkg.install_date.clone(),
            held: if pkg.held {
                "[held]".into()
            } else {
                String::new()
            },
        })
        .collect();

    tables::print_list_table(&rows, &format!("共 {} 个软件", filtered.len()));

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
    fn list_empty_shows_message() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());
        let args = Args { filter: None };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn list_with_packages() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        {
            let mut db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
            db.insert_package(
                "git".into(),
                hit_core::store::InstalledPackage {
                    version: "2.45.1".into(),
                    bucket: "main".into(),
                    architecture: "64bit".into(),
                    install_date: "2024-06-15".into(),
                    ..Default::default()
                },
            );
            db.insert_package(
                "python".into(),
                hit_core::store::InstalledPackage {
                    version: "3.12.4".into(),
                    bucket: "main".into(),
                    architecture: "64bit".into(),
                    install_date: "2024-06-10".into(),
                    ..Default::default()
                },
            );
            db.save().unwrap();
        }

        let args = Args { filter: None };
        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn list_shows_held_mark() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        {
            let mut db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
            let mut pkg = hit_core::store::InstalledPackage {
                version: "1.0".into(),
                bucket: "main".into(),
                ..Default::default()
            };
            pkg.held = true;
            db.insert_package("held_app".into(), pkg);
            db.insert_package(
                "normal_app".into(),
                hit_core::store::InstalledPackage {
                    version: "2.0".into(),
                    bucket: "main".into(),
                    ..Default::default()
                },
            );
            db.save().unwrap();
        }

        let args = Args { filter: None };
        let result = execute(&args, &session);
        assert!(result.is_ok());
    }

    #[test]
    fn list_with_filter() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        {
            let mut db = hit_core::store::Db::load(&hit_core::store::db_path(&session)).unwrap();
            db.insert_package(
                "git".into(),
                hit_core::store::InstalledPackage {
                    version: "2.45.1".into(),
                    bucket: "main".into(),
                    ..Default::default()
                },
            );
            db.insert_package(
                "python".into(),
                hit_core::store::InstalledPackage {
                    version: "3.12.4".into(),
                    bucket: "main".into(),
                    ..Default::default()
                },
            );
            db.save().unwrap();
        }

        let args = Args {
            filter: Some("py".into()),
        };
        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
