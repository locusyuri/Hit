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

// ============================================================================
// BUGS.md 回归测试：真实 Scoop manifest 兼容性
// ============================================================================

/// 验证 perl.json 的 autoupdate.hash 为 {url, jsonpath} 对象形式时能正确解析
/// （原 bug：HashField 不支持 Fetch 变体，导致整个 manifest 被跳过）
#[test]
fn regression_perl_hash_fetch_jsonpath() {
    let body = r#"{
        "version": "5.42.2.1",
        "description": "test",
        "homepage": "https://x",
        "license": "GPL-1.0-or-later|Artistic-1.0-Perl",
        "architecture": {
            "64bit": {
                "url": "https://x/strawberry-perl.zip",
                "hash": "32d83be90cf04b807cfb9477482bc36302cdee6f5b04cf57e81adecbd8f07898"
            }
        },
        "autoupdate": {
            "architecture": {
                "64bit": {
                    "url": "https://x/strawberry-perl-$version.zip",
                    "hash": {
                        "url": "https://strawberryperl.com/releases.json",
                        "jsonpath": "$[?(@.version == '$version')].edition.portable.sha256"
                    }
                }
            }
        }
    }"#;
    let flat = parse_and_resolve(body, Arch::X86_64).unwrap();
    // 顶层 hash 仍能取出值
    assert_eq!(
        flat.inner().hash.as_ref().unwrap().values(),
        vec!["32d83be90cf04b807cfb9477482bc36302cdee6f5b04cf57e81adecbd8f07898"]
    );
}

/// 验证顶层 hash 也支持 Fetch 对象形式（与 autoupdate.hash 一致）
#[test]
fn regression_hash_fetch_top_level() {
    let body = r#"{
        "version": "1.0",
        "description": "test",
        "homepage": "https://x",
        "license": "MIT",
        "url": "https://x/app.zip",
        "hash": {
            "url": "https://x/releases.json",
            "jsonpath": "$.sha256"
        }
    }"#;
    let m = parse_str(body).unwrap();
    assert!(matches!(
        m.hash,
        Some(hit_core::manifest::HashField::Fetch { .. })
    ));
}

/// 验证 suggest 字段值可以是字符串数组（原 bug：声明为 String 导致数组形式失败）
/// 参考 digital.json 的 `"suggest": { "JDK": ["java/opendk", "java/oraclejdk"] }`
#[test]
fn regression_suggest_array_value() {
    let body = r#"{
        "version": "0.31",
        "description": "test",
        "homepage": "https://x",
        "license": "GPL-3.0-only",
        "suggest": {
            "JDK": ["java/opendk", "java/oraclejdk"]
        },
        "url": "https://x/Digital.zip",
        "hash": "12f014c8b99140554f8f7464ebc771bbe3de6af39c83c20463492bcb892afc69"
    }"#;
    let m = parse_str(body).unwrap();
    let sug = m.suggest.expect("应有 suggest");
    let jdk = sug.get("JDK").expect("应有 JDK 条目");
    assert_eq!(jdk.len(), 2, "suggest.JDK 应有 2 个条目");
}

/// 验证 suggest 字段值也可以是单字符串（向后兼容）
#[test]
fn regression_suggest_single_string_value() {
    let body = r#"{
        "version": "1.0",
        "description": "test",
        "homepage": "https://x",
        "license": "MIT",
        "suggest": {
            "JDK": "java/openjdk"
        },
        "url": "https://x/app.zip",
        "hash": "12f014c8b99140554f8f7464ebc771bbe3de6af39c83c20463492bcb892afc69"
    }"#;
    let m = parse_str(body).unwrap();
    let sug = m.suggest.expect("应有 suggest");
    let jdk = sug.get("JDK").expect("应有 JDK 条目");
    assert_eq!(jdk.len(), 1, "单字符串 suggest 应归一化为 1 个条目");
}

/// 验证 checkver.script 可以是单字符串（原 bug：声明为 Vec<String> 导致单字符串失败）
/// 参考 feem.json 的 `"script": "(Invoke-WebRequest ...).Headers['x-bz-file-name']"`
#[test]
fn regression_checkver_script_single_string() {
    let body = r#"{
        "version": "4.3.0",
        "description": "test",
        "homepage": "https://x",
        "license": "Proprietary",
        "url": "https://x/feem.zip",
        "hash": "ff79a85a2949447a73103f0d2dfcac6d94cf9bc9ee8e30940d2eb7e49ff7e076",
        "checkver": {
            "script": "(Invoke-WebRequest 'https://feem.link' -Method Head).Headers['x-bz-file-name']",
            "regex": "Feem_v([\\d.]+)_*"
        },
        "autoupdate": {
            "url": "https://x/Feem_v$version.zip"
        }
    }"#;
    let m = parse_str(body).unwrap();
    let cv = m.checkver.expect("应有 checkver");
    match cv {
        hit_core::manifest::CheckverField::Full(c) => {
            assert!(c.script.is_some(), "checkver.script 应被解析");
            assert!(c.regex.is_some(), "checkver.regex 应被解析");
        }
        hit_core::manifest::CheckverField::Short(_) => panic!("应为 Full 变体"),
    }
}
