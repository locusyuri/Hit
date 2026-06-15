//! Manifest 特殊字段综合测试套件（1.2.7）。
//!
//! 覆盖 architecture / depends / persist / pre/post_install 的端到端行为，
//! 使用真实 Scoop manifest fixture + 内联 JSON 覆盖边缘场景。

use hit_core::manifest::{
    parse_and_resolve, parse_str, Arch, FlatManifest, HookType, InstallVars, IntoVarMap, Manifest,
    PersistItem, ScriptField,
};

const FIXTURES: &[(&str, &str)] = &[
    ("git", include_str!("fixtures/manifest/git.json")),
    ("7zip", include_str!("fixtures/manifest/7zip.json")),
    ("python", include_str!("fixtures/manifest/python.json")),
    ("nodejs", include_str!("fixtures/manifest/nodejs.json")),
    ("ack", include_str!("fixtures/manifest/ack.json")),
    (
        "aws-sam-cli",
        include_str!("fixtures/manifest/aws-sam-cli.json"),
    ),
];

fn fixture(name: &str) -> &str {
    FIXTURES.iter().find(|(n, _)| *n == name).unwrap().1
}

fn resolve(name: &str, arch: Arch) -> FlatManifest {
    parse_and_resolve(fixture(name), arch).unwrap()
}

// ============================================================================
// PersistItem::source_and_target()
// ============================================================================

#[test]
fn persist_simple_source_and_target() {
    let p = PersistItem::Simple("etc".into());
    assert_eq!(p.source_and_target(), ("etc", "etc"));
}

#[test]
fn persist_renamed_source_and_target() {
    let p = PersistItem::Renamed {
        src: "data".into(),
        dst: "backup".into(),
    };
    assert_eq!(p.source_and_target(), ("data", "backup"));
}

#[test]
fn persist_list_from_7zip_fixture() {
    let m: Manifest = parse_str(fixture("7zip")).unwrap();
    let list = m.persist.as_ref().expect("7zip 应有 persist");
    assert_eq!(list.len(), 2);
    // 均为 Simple，同名映射
    let pairs: Vec<_> = list.0.iter().map(|p| p.source_and_target()).collect();
    assert_eq!(pairs[0], ("Codecs", "Codecs"));
    assert_eq!(pairs[1], ("Formats", "Formats"));
}

#[test]
fn persist_renamed_from_inline_json() {
    let body = r#"{
        "version": "1.0",
        "description": "test",
        "homepage": "https://x",
        "license": "MIT",
        "persist": [["config", "settings.json"], "data"]
    }"#;
    let m: Manifest = parse_str(body).unwrap();
    let list = m.persist.as_ref().expect("应有 persist");
    assert_eq!(list.len(), 2);
    assert_eq!(list.0[0].source_and_target(), ("config", "settings.json"));
    assert_eq!(list.0[1].source_and_target(), ("data", "data"));
}

// ============================================================================
// Manifest::depends_list()
// ============================================================================

#[test]
fn depends_list_empty_when_absent() {
    let m: Manifest = parse_str(fixture("git")).unwrap();
    assert!(m.depends_list().is_empty());
}

#[test]
fn depends_list_single_from_ack() {
    let m: Manifest = parse_str(fixture("ack")).unwrap();
    assert_eq!(m.depends_list(), vec!["perl"]);
}

#[test]
fn depends_list_single_from_sam() {
    let m: Manifest = parse_str(fixture("aws-sam-cli")).unwrap();
    assert_eq!(m.depends_list(), vec!["lessmsi"]);
}

#[test]
fn depends_list_many_from_inline_json() {
    let body = r#"{
        "version": "1.0",
        "description": "test",
        "homepage": "https://x",
        "license": "MIT",
        "depends": ["a", "extras/b", "c"]
    }"#;
    let m: Manifest = parse_str(body).unwrap();
    assert_eq!(m.depends_list(), vec!["a", "extras/b", "c"]);
}

// ============================================================================
// FlatManifest::resolve_script()
// ============================================================================

#[test]
fn resolve_script_pre_install_top_level() {
    let flat = resolve("ack", Arch::X86_64);
    let script = flat.resolve_script(HookType::PreInstall);
    assert!(script.is_some(), "ack 应有顶层 pre_install");
    assert!(matches!(script, Some(ScriptField::Single(_))));
}

#[test]
fn resolve_script_pre_install_arch_override() {
    // 7zip arm64 分支有 pre_install，64bit 分支无
    let arm = resolve("7zip", Arch::Arm64);
    assert!(
        arm.resolve_script(HookType::PreInstall).is_some(),
        "7zip arm64 应有 pre_install"
    );

    let x64 = resolve("7zip", Arch::X86_64);
    assert!(
        x64.resolve_script(HookType::PreInstall).is_none(),
        "7zip 64bit 不应有 pre_install"
    );
}

#[test]
fn resolve_script_installer_script() {
    let flat = resolve("python", Arch::X86_64);
    let script = flat.resolve_script(HookType::Installer);
    assert!(script.is_some(), "python 应有 installer.script");
    assert!(matches!(script, Some(ScriptField::Lines(_))));
}

#[test]
fn resolve_script_uninstaller_script() {
    // git uninstaller.script 为 Lines（多行数组）
    let flat = resolve("git", Arch::X86_64);
    let script = flat.resolve_script(HookType::Uninstaller);
    assert!(script.is_some(), "git 应有 uninstaller.script");
    assert!(matches!(script, Some(ScriptField::Lines(_))));

    // 7zip uninstaller.script 为 Single（单行字符串）
    let flat2 = resolve("7zip", Arch::X86_64);
    let script2 = flat2.resolve_script(HookType::Uninstaller);
    assert!(script2.is_some(), "7zip 应有 uninstaller.script");
    assert!(matches!(script2, Some(ScriptField::Single(_))));
}

#[test]
fn resolve_script_missing_returns_none() {
    // ack 无 post_install
    let flat = resolve("ack", Arch::X86_64);
    assert!(flat.resolve_script(HookType::PostInstall).is_none());

    // git 无 installer
    let flat2 = resolve("git", Arch::X86_64);
    assert!(flat2.resolve_script(HookType::Installer).is_none());
}

// ============================================================================
// 架构与特殊字段交互
// ============================================================================

#[test]
fn arch_preserves_persist() {
    // persist 不受架构影响
    let x64 = resolve("7zip", Arch::X86_64);
    let arm = resolve("7zip", Arch::Arm64);
    assert_eq!(
        x64.inner().persist.as_ref().map(|p| p.len()),
        arm.inner().persist.as_ref().map(|p| p.len()),
        "persist 在架构合并后应保持不变"
    );
}

#[test]
fn arch_preserves_depends() {
    // depends 不受架构影响
    let flat = resolve("ack", Arch::X86_64);
    assert_eq!(flat.inner().depends_list(), vec!["perl"]);
}

#[test]
fn arch_pre_install_fallback_to_top() {
    // git 的 pre_install 在顶层，所有架构分支应回退到它
    let x64 = resolve("git", Arch::X86_64);
    let arm = resolve("git", Arch::Arm64);
    assert!(
        x64.resolve_script(HookType::PreInstall).is_some(),
        "git x64 应回退到顶层 pre_install"
    );
    assert!(
        arm.resolve_script(HookType::PreInstall).is_some(),
        "git arm64 应回退到顶层 pre_install"
    );
}

#[test]
fn arch_arm64_pre_install_override() {
    // 7zip arm64 分支覆盖顶层（顶层无 pre_install）
    let arm = resolve("7zip", Arch::Arm64);
    let script = arm.resolve_script(HookType::PreInstall);
    assert!(
        script.is_some(),
        "7zip arm64 分支的 pre_install 应被保留"
    );
    assert!(matches!(script, Some(ScriptField::Lines(_))));
}

// ============================================================================
// 全流程管线测试
// ============================================================================

#[test]
fn full_pipeline_python_all_hooks() {
    let flat = resolve("python", Arch::X86_64);
    let m = flat.inner();

    // persist
    let persist = m.persist.as_ref().expect("python 应有 persist");
    assert_eq!(persist.len(), 2);
    assert_eq!(persist.0[0].source_and_target(), ("Scripts", "Scripts"));
    assert_eq!(
        persist.0[1].source_and_target(),
        ("Lib\\site-packages", "Lib\\site-packages")
    );

    // depends
    assert!(m.depends_list().is_empty());

    // 所有钩子
    assert!(flat.resolve_script(HookType::PreInstall).is_some());
    assert!(flat.resolve_script(HookType::Installer).is_some());
    assert!(flat.resolve_script(HookType::PostInstall).is_some());
    assert!(flat.resolve_script(HookType::Uninstaller).is_some());
    assert!(flat.resolve_script(HookType::PreUninstall).is_none());
    assert!(flat.resolve_script(HookType::PostUninstall).is_none());
}

#[test]
fn full_pipeline_git_lifecycle_order() {
    let flat = resolve("git", Arch::X86_64);

    // 安装阶段：pre_install → (no installer) → post_install
    assert!(flat.resolve_script(HookType::PreInstall).is_some());
    assert!(flat.resolve_script(HookType::Installer).is_none());
    assert!(flat.resolve_script(HookType::PostInstall).is_some());

    // 卸载阶段：pre_uninstall → uninstaller → (no post_uninstall)
    assert!(flat.resolve_script(HookType::PreUninstall).is_some());
    assert!(flat.resolve_script(HookType::Uninstaller).is_some());
    assert!(flat.resolve_script(HookType::PostUninstall).is_none());
}

#[test]
fn full_pipeline_substitute_then_resolve() {
    use hit_core::manifest::substitute_manifest_in_place;
    use std::path::PathBuf;

    let mut m: Manifest = parse_str(fixture("python")).unwrap();
    let vars = InstallVars {
        version: "3.12.0".into(),
        dir: PathBuf::from("C:\\scoop\\apps\\python\\3.12.0"),
        persist_dir: PathBuf::from("C:\\scoop\\persist\\python"),
        architecture: Arch::X86_64,
        global: false,
        app: "python".into(),
        original_dir: None,
    };
    substitute_manifest_in_place(&mut m, &vars.to_var_map());

    // 替换后架构合并仍正确
    let flat = FlatManifest::resolve_architecture(m, Arch::X86_64);
    assert!(flat.resolve_script(HookType::PreInstall).is_some());
    assert!(flat.resolve_script(HookType::Installer).is_some());

    // persist 仍存在且未被替换（persist 条目不含变量）
    let persist = flat.inner().persist.as_ref().expect("应有 persist");
    assert_eq!(persist.len(), 2);
}

// ============================================================================
// HookType 全覆盖
// ============================================================================

#[test]
fn hook_type_all_variants_covered() {
    let body = r#"{
        "version": "1.0",
        "description": "test",
        "homepage": "https://x",
        "license": "MIT",
        "pre_install": "echo pre",
        "post_install": "echo post",
        "pre_uninstall": "echo pre_un",
        "post_uninstall": "echo post_un",
        "installer": { "script": "echo inst" },
        "uninstaller": { "script": "echo uninst" }
    }"#;
    let flat = parse_and_resolve(body, Arch::X86_64).unwrap();

    assert!(flat.resolve_script(HookType::PreInstall).is_some());
    assert!(flat.resolve_script(HookType::Installer).is_some());
    assert!(flat.resolve_script(HookType::PostInstall).is_some());
    assert!(flat.resolve_script(HookType::PreUninstall).is_some());
    assert!(flat.resolve_script(HookType::Uninstaller).is_some());
    assert!(flat.resolve_script(HookType::PostUninstall).is_some());
}
