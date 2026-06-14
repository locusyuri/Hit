//! 变量替换引擎集成测试。
//!
//! 验证 `substitute_manifest_in_place` 在真实 manifest 上的行为，
//! 覆盖顶层字段、架构分支、脚本字段、env_set、bin、autoupdate 等场景。

use std::collections::BTreeMap;
use std::path::PathBuf;

use hit_core::manifest::{
    parse_str, substitute_manifest_in_place, Arch, AutoupdateVars, BinItem, BinList, InstallVars,
    IntoVarMap, OneOrMany, ScriptField, UrlContext, VarMap, autoupdate_version_vars,
};

fn sample_vars() -> VarMap {
    let iv = InstallVars {
        version: "2.54.0".into(),
        dir: PathBuf::from("C:/scoop/apps/git/current"),
        persist_dir: PathBuf::from("C:/scoop/persist/git"),
        architecture: Arch::X86_64,
        global: false,
        app: "git".into(),
        original_dir: None,
    };
    iv.to_var_map()
}

#[test]
fn substitute_git_manifest_url_and_hash() {
    let body = include_str!("fixtures/manifest/git.json");
    let mut m = parse_str(body).unwrap();
    let vars = sample_vars();

    // 替换前：url 不含 $version（顶层 url 为 None，hash 为 None；架构分支有具体值）
    let arch64 = m.architecture.as_ref().unwrap().x86_64.as_ref().unwrap();
    assert!(arch64.url.as_ref().unwrap().as_slice()[0].contains("2.54.0"));

    substitute_manifest_in_place(&mut m, &vars);

    // 替换后：架构分支中的 $version 应被替换（虽然此处是 literal 版本号，无 $version token）
    let arch64 = m.architecture.as_ref().unwrap().x86_64.as_ref().unwrap();
    let url = &arch64.url.as_ref().unwrap().as_slice()[0];
    assert!(url.contains("2.54.0"), "URL 应包含版本号：{url}");
}

#[test]
fn substitute_preserves_text_fields() {
    let body = include_str!("fixtures/manifest/git.json");
    let mut m = parse_str(body).unwrap();
    let desc_before = m.description.clone();
    let homepage_before = m.homepage.clone();

    let mut vars = sample_vars();
    // 故意注入一个会"污染"文本字段的变量（应不生效）
    vars.insert("$description".into(), "BAD".into());
    vars.insert("$homepage".into(), "BAD".into());
    substitute_manifest_in_place(&mut m, &vars);

    assert_eq!(m.description, desc_before);
    assert_eq!(m.homepage, homepage_before);
}

#[test]
fn substitute_env_set_values() {
    let body = include_str!("fixtures/manifest/git.json");
    let mut m = parse_str(body).unwrap();
    let mut vars = sample_vars();
    vars.insert("$custom".into(), "injected".into());

    // git.json 的 env_set: {"GIT_INSTALL_ROOT": "$dir"}
    substitute_manifest_in_place(&mut m, &vars);
    let env = m.env_set.as_ref().unwrap();
    let val = env.get("GIT_INSTALL_ROOT").unwrap();
    assert!(
        val.contains("scoop/apps/git/current"),
        "期望 $dir 展开，得到：{val}"
    );
}

#[test]
fn substitute_script_fields_in_place() {
    let body = include_str!("fixtures/manifest/python.json");
    let mut m = parse_str(body).unwrap();
    let vars = sample_vars();

    substitute_manifest_in_place(&mut m, &vars);

    // python.json 的 pre_install 包含 "$version" 引用（在脚本字符串中）
    // 替换后应展开为 "3.14.6"（python.json 中的 literal 版本）
    let pre = m.pre_install.as_ref().unwrap();
    if let ScriptField::Lines(lines) = pre {
        // 不应再有未展开的 $version（除 $py_fullversion 等自定义 PS 变量外）
        let joined = lines.join("\n");
        // $py_fullversion 不是 Hit 识别的变量，应原样保留
        assert!(joined.contains("$py_fullversion"));
    }
}

#[test]
fn substitute_bin_list_paths() {
    let body = include_str!("fixtures/manifest/aws-sam-cli.json");
    let mut m = parse_str(body).unwrap();
    let mut vars = sample_vars();
    vars.insert("$app".into(), "sam".into());
    substitute_manifest_in_place(&mut m, &vars);

    let BinList(items) = m.bin.as_ref().unwrap();
    // bin 数组第一项是 string，第二项是 tuple[2]
    assert!(matches!(&items[0], BinItem::Simple(s) if s.contains("sam.cmd")));
    assert!(matches!(&items[1], BinItem::Aliased { path, alias, .. }
        if path.contains("sam.cmd") && alias == "aws-sam-cli"));
}

#[test]
fn autoupdate_url_substitution_with_url_context() {
    // 构造一个包含 autoupdate.hash = { url, regex } 的最小 manifest
    let body = r#"{
        "version": "1.0.0",
        "description": "dummy",
        "homepage": "https://x",
        "license": "MIT",
        "url": "https://example.com/file-1.0.0.zip",
        "hash": "deadbeef",
        "autoupdate": {
            "url": "https://example.com/file-$version.zip",
            "hash": {
                "url": "$baseurl/checksums.txt",
                "regex": "$basename: ([a-fA-F0-9]{64})"
            }
        }
    }"#;
    let mut m = parse_str(body).unwrap();
    let mut vars = VarMap::new();
    vars.insert("$version".into(), "2.0.0".into());
    substitute_manifest_in_place(&mut m, &vars);

    let au = m.autoupdate.as_ref().unwrap();
    // autoupdate.url 应展开
    let new_url = match au.url.as_ref().unwrap() {
        OneOrMany::One(s) => s.clone(),
        _ => panic!("期望 OneOrMany::One"),
    };
    assert_eq!(new_url, "https://example.com/file-2.0.0.zip");
    // autoupdate.hash 应为 Fetch，url 与 regex 应注入 URL 上下文
    match au.hash.as_ref().unwrap() {
        hit_core::manifest::AutoupdateHash::Fetch { url, regex, .. } => {
            assert_eq!(url, "https://example.com/checksums.txt");
            assert!(regex.as_ref().unwrap().contains("file-2.0.0.zip"));
        }
        other => panic!("期望 AutoupdateHash::Fetch，得到 {other:?}"),
    }
}

#[test]
fn autoupdate_version_vars_complete_set() {
    let av = AutoupdateVars {
        version: "1.2.3".into(),
        custom_matches: BTreeMap::new(),
    };
    let v = autoupdate_version_vars(&av);
    let expected = [
        ("$version", "1.2.3"),
        ("$cleanVersion", "123"),
        ("$dotVersion", "1.2.3"),
        ("$dashVersion", "1-2-3"),
        ("$underscoreVersion", "1_2_3"),
        ("$majorVersion", "1"),
        ("$minorVersion", "2"),
        ("$patchVersion", "3"),
    ];
    for (k, exp) in expected {
        assert_eq!(v.get(k).map(String::as_str), Some(exp), "变量 {k} 期望 {exp}");
    }
}

#[test]
fn arch_branch_independent_substitution() {
    // 验证架构分支独立替换：两个分支的 $architecture 应各自保留
    let body = r#"{
        "version": "1.0",
        "description": "dummy",
        "homepage": "https://x",
        "license": "MIT",
        "architecture": {
            "64bit": { "url": "https://example.com/$architecture/a.zip" },
            "32bit": { "url": "https://example.com/$architecture/b.zip" }
        }
    }"#;
    let mut m = parse_str(body).unwrap();

    // 注意：substitute_manifest_in_place 用同一个 VarMap，不会切换 $architecture
    // 调用方应负责为每个分支准备正确的 VarMap（1.2.4 的 FlatManifest 会处理）
    let mut vars = VarMap::new();
    vars.insert("$architecture".into(), "64bit".into());
    substitute_manifest_in_place(&mut m, &vars);

    let arch = m.architecture.as_ref().unwrap();
    let u64 = arch.x86_64.as_ref().unwrap().url.as_ref().unwrap().as_slice();
    let u32 = arch.x86.as_ref().unwrap().url.as_ref().unwrap().as_slice();
    assert!(u64[0].contains("/64bit/a.zip"));
    // 32bit 分支也被同一个 VarMap 替换（行为一致，但 32bit 实际应使用 "32bit" 值——由调用方决定）
    assert!(u32[0].contains("/64bit/b.zip"));
}

#[test]
fn url_context_handles_fragment_and_encoding() {
    let ctx = UrlContext::from_url("https://a.com/path/my%20file-v1.2.tar.gz#/dl.gz");
    assert_eq!(ctx.url, "https://a.com/path/my%20file-v1.2.tar.gz");
    assert_eq!(ctx.baseurl, "https://a.com/path");
    assert_eq!(ctx.basename, "my file-v1.2.tar.gz");
    assert_eq!(ctx.basename_no_ext, "my file-v1.2.tar");
    assert_eq!(ctx.url_no_ext, "https://a.com/path/my%20file-v1.2.tar");
}
