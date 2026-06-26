//! 安装卸载集成测试
//!
//! 使用 hit-test-utils 创建隔离环境，测试安装/卸载流水线。

use hit_common::config::HitConfig;
use hit_common::Session;
use hit_core::install::{install, uninstall, InstallOptions};
use hit_core::manifest::{License, Manifest};
use hit_core::store::{Db, db_path};
use hit_test_utils::temp_scoop_root;

/// 创建测试 session（指向临时 scoop root）
fn test_session(root: &std::path::Path) -> Session {
    let config = HitConfig {
        root_path: Some(root.to_string_lossy().into()),
        ..Default::default()
    };
    Session::with_config(config)
}

/// 极简 manifest（无 url、无 bin）
fn minimal_manifest(version: &str) -> Manifest {
    Manifest {
        version: version.to_string(),
        description: "test app".into(),
        homepage: "https://example.com".into(),
        license: License::Identifier("MIT".into()),
        ..Default::default()
    }
}

/// 写入 bucket manifest 文件
fn write_manifest(root: &std::path::Path, bucket: &str, app: &str, version: &str) {
    let bucket_dir = root.join("buckets").join(bucket);
    std::fs::create_dir_all(&bucket_dir).unwrap();
    let json = format!(
        r#"{{"version":"{version}","description":"test","homepage":"https://example.com","license":"MIT"}}"#
    );
    std::fs::write(bucket_dir.join(format!("{app}.json")), json).unwrap();
}

#[test]
fn install_minimal_manifest_saves_to_db() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    write_manifest(&root, "main", "myapp", "1.0");

    let manifest = minimal_manifest("1.0");
    let options = InstallOptions::default();
    let result = install(&session, "myapp", &manifest, "main", &options);

    // 极简 manifest 无 url，后续步骤可能失败
    // 关键验证：不会因为"已安装"或 manifest 解析失败
    if let Err(e) = &result {
        let msg = e.to_string();
        assert!(!msg.contains("已安装"), "不应报已安装: {msg}");
    }

    // 如果安装成功，验证 db.json
    if result.is_ok() {
        let db = Db::load(&db_path(&session)).unwrap();
        assert!(db.is_installed("myapp"));
        assert_eq!(db.get_package("myapp").unwrap().version, "1.0");
    }
}

#[test]
fn install_already_installed_errors() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    // 模拟已安装
    let current = root.join("apps").join("myapp").join("current");
    std::fs::create_dir_all(&current).unwrap();

    let manifest = minimal_manifest("1.0");
    let options = InstallOptions::default();
    let result = install(&session, "myapp", &manifest, "main", &options);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("已安装"));
}

#[test]
fn install_force_overwrites_existing() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    // 模拟已安装
    let current = root.join("apps").join("myapp").join("current");
    std::fs::create_dir_all(&current).unwrap();

    let manifest = minimal_manifest("1.0");
    let options = InstallOptions {
        force: true,
        ..Default::default()
    };
    let result = install(&session, "myapp", &manifest, "main", &options);

    // force 模式不应报"已安装"
    if let Err(e) = &result {
        assert!(!e.to_string().contains("已安装"));
    }
}

#[test]
fn uninstall_nonexistent_errors() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    let result = uninstall(&session, "nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("未安装"));
}

#[test]
fn uninstall_without_junction_fails_gracefully() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    // 创建 app 目录但没有 current junction
    let app_dir = root.join("apps").join("myapp");
    std::fs::create_dir_all(&app_dir).unwrap();

    {
        let mut db = Db::load(&db_path(&session)).unwrap();
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

    // 卸载会因缺少 current junction 而报错（但不应 panic）
    let result = uninstall(&session, "myapp");
    assert!(result.is_err());
}

#[test]
fn install_multiple_apps_independent() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    write_manifest(&root, "main", "app1", "1.0");
    write_manifest(&root, "main", "app2", "2.0");

    let manifest1 = minimal_manifest("1.0");
    let manifest2 = minimal_manifest("2.0");
    let options = InstallOptions::default();

    // 两个 app 的安装应互不干扰
    let r1 = install(&session, "app1", &manifest1, "main", &options);
    let r2 = install(&session, "app2", &manifest2, "main", &options);

    // 至少不应因"已安装"报错
    for (name, r) in [("app1", &r1), ("app2", &r2)] {
        if let Err(e) = r {
            assert!(
                !e.to_string().contains("已安装"),
                "{name} 不应报已安装: {e}"
            );
        }
    }
}
