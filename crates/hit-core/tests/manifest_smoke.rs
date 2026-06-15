//! Manifest 模块冒烟测试。
//!
//! 使用 `ref/Main/bucket/` 真实 manifest 作为 fixture，覆盖主要多态路径：
//! - `bin`（单字符串 / 数组[字符串] / 数组[tuple]）
//! - `persist`（单字符串 / 数组[字符串]）
//! - `license`（字符串 / 对象）
//! - `depends`（单字符串）+ `suggest`（对象）
//! - `checkver`（对象模式）
//! - `autoupdate.hash`（对象 Fetch 模式）

use hit_core::manifest::{
    parse_str, validate, AutoupdateHash, BinItem, BinList, CheckverField, HashField, License,
    Manifest, OneOrMany, PersistList, ScriptField,
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

#[test]
fn parse_all_fixture_manifests() {
    for (name, body) in FIXTURES {
        parse_str(body).unwrap_or_else(|e| panic!("{name} 解析失败：{e}"));
    }
}

#[test]
fn validate_all_fixture_manifests() {
    for (name, body) in FIXTURES {
        let m = parse_str(body).unwrap();
        validate(&m)
            .into_result(name)
            .unwrap_or_else(|e| panic!("{name} 验证失败：{e}"));
    }
}

#[test]
fn git_manifest_polymorphic_paths() {
    let body = FIXTURES.iter().find(|(n, _)| *n == "git").unwrap().1;
    let m: Manifest = parse_str(body).unwrap();

    // license 对象形式：{ identifier, url }
    assert!(matches!(m.license, License::Detailed { .. }));

    // bin：数组，元素均为 string
    let bin = m.bin.as_ref().expect("git 应有 bin");
    assert!(bin.len() > 1);
    assert!(matches!(&bin.0[0], BinItem::Simple(_)));

    // shortcuts：tuple[3]
    let sc = m.shortcuts.as_ref().expect("git 应有 shortcuts");
    assert!(!sc.is_empty());
    assert_eq!(sc[0].target, "git-bash.exe");
    assert_eq!(sc[0].name, "Git\\Git Bash");
    assert_eq!(sc[0].args.as_deref(), Some("--cd-to-home"));

    // pre_install：多行脚本
    let pre = m.pre_install.as_ref().expect("git 应有 pre_install");
    assert!(matches!(pre, ScriptField::Lines(_)));

    // env_set：BTreeMap
    let env = m.env_set.as_ref().expect("git 应有 env_set");
    assert_eq!(env.get("GIT_INSTALL_ROOT").map(String::as_str), Some("$dir"));

    // env_add_path：单字符串 "cmd"
    let path = m.env_add_path.as_ref().expect("git 应有 env_add_path");
    assert!(matches!(path, OneOrMany::One(s) if s == "cmd"));

    // checkver：对象模式
    assert!(matches!(m.checkver, Some(CheckverField::Full(_))));

    // autoupdate.hash：Fetch 对象模式（git.json 的 hash 是 { url, regex }）
    let au = m.autoupdate.as_ref().expect("git 应有 autoupdate");
    assert!(matches!(au.hash, Some(AutoupdateHash::Fetch { .. })));

    // architecture：64bit + arm64
    let arch = m.architecture.as_ref().expect("git 应有 architecture");
    assert!(arch.x86_64.is_some());
    assert!(arch.arm64.is_some());
    assert!(arch.x86.is_none());
}

#[test]
fn python_manifest_bin_alias() {
    let body = FIXTURES.iter().find(|(n, _)| *n == "python").unwrap().1;
    let m: Manifest = parse_str(body).unwrap();

    // bin 数组包含 tuple[2]：["python.exe", "python3"]
    let bin = m.bin.as_ref().expect("python 应有 bin");
    assert!(!bin.is_empty());
    let first = &bin.0[0];
    match first {
        BinItem::Aliased { path, alias, args } => {
            assert_eq!(path, "python.exe");
            assert_eq!(alias, "python3");
            assert!(args.is_none());
        }
        other => panic!("期望 BinItem::Aliased，得到 {other:?}"),
    }

    // license 字符串形式
    assert!(matches!(m.license, License::Identifier(_)));

    // installer.script：多行
    let inst = m.installer.as_ref().expect("python 应有 installer");
    assert!(inst.script.is_some());
}

#[test]
fn ack_manifest_depends_string() {
    let body = FIXTURES.iter().find(|(n, _)| *n == "ack").unwrap().1;
    let m: Manifest = parse_str(body).unwrap();

    // depends 单字符串
    let deps = m.depends.as_ref().expect("ack 应有 depends");
    assert!(matches!(deps, OneOrMany::One(s) if s == "perl"));

    // bin 单字符串 "ack.bat"
    let bin = m.bin.as_ref().expect("ack 应有 bin");
    assert_eq!(bin.len(), 1);
    assert!(matches!(&bin.0[0], BinItem::Simple(s) if s == "ack.bat"));

    // pre_install 单字符串
    let pre = m.pre_install.as_ref().expect("ack 应有 pre_install");
    assert!(matches!(pre, ScriptField::Single(_)));
}

#[test]
fn aws_sam_cli_depends_plus_suggest() {
    let body = FIXTURES
        .iter()
        .find(|(n, _)| *n == "aws-sam-cli")
        .unwrap()
        .1;
    let m: Manifest = parse_str(body).unwrap();

    // depends + suggest 并存
    let deps = m.depends.as_ref().expect("sam 应有 depends");
    assert!(matches!(deps, OneOrMany::One(s) if s == "lessmsi"));

    let sug = m.suggest.as_ref().expect("sam 应有 suggest");
    assert_eq!(sug.get("AWS CLI").map(String::as_str), Some("aws"));

    // bin 数组混 string + tuple[2]
    let BinList(items) = m.bin.as_ref().expect("sam 应有 bin");
    assert_eq!(items.len(), 2);
    assert!(matches!(&items[0], BinItem::Simple(_)));
    assert!(matches!(&items[1], BinItem::Aliased { .. }));
}

#[test]
fn persist_list_simple_strings() {
    // 7zip / nodejs 的 persist 都是字符串数组
    for name in ["7zip", "nodejs"] {
        let body = FIXTURES.iter().find(|(n, _)| *n == name).unwrap().1;
        let m: Manifest = parse_str(body).unwrap();
        let list: &PersistList = m
            .persist
            .as_ref()
            .unwrap_or_else(|| panic!("{name} 应有 persist"));
        assert!(!list.is_empty(), "{name} persist 不应为空");
    }
}

#[test]
fn hash_field_values() {
    // ack.json 单字符串 hash
    let body = FIXTURES.iter().find(|(n, _)| *n == "ack").unwrap().1;
    let m: Manifest = parse_str(body).unwrap();
    let hash = m.hash.as_ref().expect("ack 应有 hash");
    assert!(matches!(hash, HashField::Single(_)));
    assert_eq!(hash.values().len(), 1);
}

#[test]
fn roundtrip_serialize_preserves_structure() {
    // 解析后再序列化回字符串，应仍能再次解析。
    for (name, body) in FIXTURES {
        let m: Manifest = parse_str(body).unwrap();
        let s2 = sonic_rs::to_string_pretty(&m)
            .unwrap_or_else(|e| panic!("{name} 序列化失败：{e}"));
        let m2: Manifest =
            parse_str(&s2).unwrap_or_else(|e| panic!("{name} roundtrip 解析失败：{e}"));
        assert_eq!(m.version, m2.version);
        assert_eq!(m.description, m2.description);
    }
}
