//! Parser 业务逻辑集成测试。
//!
//! 覆盖 `FlatManifest::resolve_architecture` / `supported_architectures` /
//! `parse_and_resolve` / `collect_all_env_sets` 在真实 manifest 上的行为。

use hit_core::manifest::{
    parse_and_resolve, parse_str, supported_architectures, Arch, FlatManifest, OneOrMany,
};

#[test]
fn git_resolve_x86_64_overrides_url() {
    let body = include_str!("fixtures/manifest/git.json");
    let m = parse_str(body).unwrap();
    let flat = FlatManifest::resolve_architecture(m, Arch::X86_64);
    // 顶层 url 为 None（git.json 无顶层 url）；架构分支 64bit 提供 url
    let url = flat
        .0
        .url
        .as_ref()
        .expect("x86_64 应合并到 url")
        .as_slice();
    assert!(url[0].contains("64-bit.7z"), "URL 应包含 64-bit：{}", url[0]);
    assert!(flat.0.architecture.is_none());
}

#[test]
fn git_resolve_arm64_selects_arm_branch() {
    let body = include_str!("fixtures/manifest/git.json");
    let m = parse_str(body).unwrap();
    let flat = FlatManifest::resolve_architecture(m, Arch::Arm64);
    let url = flat
        .0
        .url
        .as_ref()
        .expect("arm64 应合并到 url")
        .as_slice();
    assert!(
        url[0].contains("arm64"),
        "URL 应包含 arm64：{}",
        url[0]
    );
}

#[test]
fn git_resolve_x86_falls_back() {
    // git.json 没有 32bit 分支；请求 x86 应保留顶层（None）
    let body = include_str!("fixtures/manifest/git.json");
    let m = parse_str(body).unwrap();
    let flat = FlatManifest::resolve_architecture(m, Arch::X86);
    assert!(
        flat.0.url.is_none(),
        "x86 回退到顶层，顶层 url 为 None"
    );
}

#[test]
fn git_supported_architectures_64bit_and_arm64() {
    let body = include_str!("fixtures/manifest/git.json");
    let m = parse_str(body).unwrap();
    let archs = supported_architectures(&m);
    assert_eq!(archs, vec![Arch::X86_64, Arch::Arm64]);
}

#[test]
fn seven_zip_resolve_x86_has_url() {
    let body = include_str!("fixtures/manifest/7zip.json");
    let flat = parse_and_resolve(body, Arch::X86).unwrap();
    let url = flat.0.url.as_ref().expect("32bit 应有 url").as_slice();
    assert!(url[0].contains(".msi"));
}

#[test]
fn nodejs_bin_preserved_after_resolve() {
    let body = include_str!("fixtures/manifest/nodejs.json");
    let m = parse_str(body).unwrap();
    let bin_count_before = m
        .bin
        .as_ref()
        .map(|b| b.len())
        .unwrap_or(0);

    let flat = FlatManifest::resolve_architecture(m, Arch::X86_64);
    let bin_count_after = flat
        .0
        .bin
        .as_ref()
        .map(|b| b.len())
        .unwrap_or(0);
    // nodejs.json 的 bin 在顶层，架构分支无 bin；合并后保留顶层
    assert_eq!(bin_count_before, bin_count_after);
}

#[test]
fn ack_no_arch_block_resolve_keeps_top() {
    let body = include_str!("fixtures/manifest/ack.json");
    let m = parse_str(body).unwrap();
    let top_url_before = m.url.clone();
    let flat = FlatManifest::resolve_architecture(m, Arch::X86_64);
    // ack.json 无 architecture 字段，合并后 url 不变
    match (top_url_before, flat.0.url) {
        (Some(OneOrMany::One(a)), Some(OneOrMany::One(b))) => assert_eq!(a, b),
        other => panic!("期望顶层 url 保留：{other:?}"),
    }
}

#[test]
fn env_set_branch_overrides_top_completely() {
    // 构造 manifest 顶层 env_set + 架构分支 env_set；合并应整体替换
    let body = r#"{
        "version": "1.0",
        "description": "d",
        "homepage": "https://x",
        "license": "MIT",
        "env_set": { "TOP_A": "1", "TOP_B": "2" },
        "architecture": {
            "64bit": { "env_set": { "X64_ONLY": "3" } }
        }
    }"#;
    let flat = parse_and_resolve(body, Arch::X86_64).unwrap();
    let env = flat.0.env_set.as_ref().unwrap();
    assert!(env.get("TOP_A").is_none());
    assert!(env.get("TOP_B").is_none());
    assert_eq!(env.get("X64_ONLY").map(String::as_str), Some("3"));
}

#[test]
fn shortcuts_branch_overrides_top() {
    let body = r#"{
        "version": "1.0",
        "description": "d",
        "homepage": "https://x",
        "license": "MIT",
        "shortcuts": [["top.exe", "Top"]],
        "architecture": {
            "64bit": { "shortcuts": [["x64.exe", "X64"]] }
        }
    }"#;
    let flat = parse_and_resolve(body, Arch::X86_64).unwrap();
    let sc = flat.0.shortcuts.as_ref().unwrap();
    assert_eq!(sc.len(), 1);
    assert_eq!(sc[0].target, "x64.exe");
}
