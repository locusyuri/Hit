//! Validator 集成测试：覆盖 error / warning / info 各场景。

use hit_core::manifest::{
    parse_str, validate, ArchSpec, Architecture, Checkver, CheckverField, HashField, InstallerSpec,
    License, Manifest, OneOrMany,
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

fn minimal_valid_manifest() -> Manifest {
    Manifest {
        version: "1.0.0".into(),
        description: "A test app".into(),
        homepage: "https://example.com".into(),
        license: License::Identifier("MIT".into()),
        url: Some(OneOrMany::One(
            "https://example.com/app-1.0.0.zip".into(),
        )),
        hash: Some(HashField::Single("a".repeat(64))),
        ..Default::default()
    }
}

// ============================================================================
// 真实 fixture 应通过验证（无 error）
// ============================================================================

#[test]
fn fixtures_have_no_errors() {
    for (name, body) in FIXTURES {
        let m = parse_str(body).unwrap();
        let diag = validate(&m);
        let errors: Vec<_> = diag.errors().collect();
        assert!(
            errors.is_empty(),
            "{name} 不应有 error，但得到：{errors:#?}"
        );
    }
}

#[test]
fn fixtures_may_have_warnings_or_info() {
    // git.json 缺少 ## maintainer_note → 应有 info
    let body = FIXTURES.iter().find(|(n, _)| *n == "git").unwrap().1;
    let m = parse_str(body).unwrap();
    let diag = validate(&m);
    assert!(
        diag.infos().any(|d| d.field == "##"),
        "git.json 应产生 maintainer_note info"
    );
}

// ============================================================================
// 必填字段缺失 → Error
// ============================================================================

#[test]
fn error_missing_version() {
    let mut m = minimal_valid_manifest();
    m.version = String::new();
    let diag = validate(&m);
    assert!(diag.has_errors());
    assert!(diag.errors().any(|d| d.field == "version"));
}

#[test]
fn error_missing_description() {
    let mut m = minimal_valid_manifest();
    m.description = String::new();
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "description"));
}

#[test]
fn error_missing_homepage() {
    let mut m = minimal_valid_manifest();
    m.homepage = String::new();
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "homepage"));
}

#[test]
fn error_missing_license() {
    let mut m = minimal_valid_manifest();
    m.license = License::Identifier(String::new());
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "license"));
}

// ============================================================================
// 字段格式非法 → Error
// ============================================================================

#[test]
fn error_invalid_version_chars() {
    let mut m = minimal_valid_manifest();
    m.version = "1.0.0 beta!".into();
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "version"));
}

#[test]
fn error_homepage_not_uri() {
    let mut m = minimal_valid_manifest();
    m.homepage = "not a url".into();
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "homepage"));
}

#[test]
fn error_invalid_hash_format() {
    let mut m = minimal_valid_manifest();
    m.hash = Some(HashField::Single("not-a-valid-hash".into()));
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "hash"));
}

// ============================================================================
// url / hash 一致性 → Error
// ============================================================================

#[test]
fn error_url_hash_length_mismatch() {
    let mut m = minimal_valid_manifest();
    m.url = Some(OneOrMany::Many(vec![
        "https://a.com/1.zip".into(),
        "https://a.com/2.zip".into(),
        "https://a.com/3.zip".into(),
    ]));
    m.hash = Some(HashField::Multiple(vec![
        HashField::Single("a".repeat(64)),
        HashField::Single("b".repeat(64)),
    ]));
    let diag = validate(&m);
    assert!(
        diag.errors()
            .any(|d| d.field == "hash" && d.message.contains("不一致")),
        "应报 url/hash 数组长度不一致"
    );
}

#[test]
fn error_no_url_anywhere() {
    let mut m = minimal_valid_manifest();
    m.url = None;
    m.architecture = None;
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "url"));
}

#[test]
fn ok_url_in_architecture_only() {
    let mut m = minimal_valid_manifest();
    m.url = None;
    m.hash = None;
    m.architecture = Some(Architecture {
        x86_64: Some(ArchSpec {
            url: Some(OneOrMany::One("https://a.com/x64.zip".into())),
            hash: Some(HashField::Single("a".repeat(64))),
            ..Default::default()
        }),
        ..Default::default()
    });
    let diag = validate(&m);
    assert!(
        !diag.errors().any(|d| d.field == "url"),
        "architecture 有 url 时不应报错"
    );
}

// ============================================================================
// checkver 正则 → Error
// ============================================================================

#[test]
fn error_broken_checkver_regex() {
    let mut m = minimal_valid_manifest();
    m.checkver = Some(CheckverField::Full(Box::new(Checkver {
        regex: Some("[invalid(".into()),
        ..Default::default()
    })));
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "checkver.regex"));
}

#[test]
fn ok_valid_checkver_regex() {
    let mut m = minimal_valid_manifest();
    m.checkver = Some(CheckverField::Full(Box::new(Checkver {
        regex: Some(r"v(\d+\.\d+\.\d+)".into()),
        ..Default::default()
    })));
    let diag = validate(&m);
    assert!(
        !diag.errors().any(|d| d.field == "checkver.regex"),
        "合法正则不应报错"
    );
}

// ============================================================================
// installer / uninstaller → Error
// ============================================================================

#[test]
fn error_installer_empty() {
    let mut m = minimal_valid_manifest();
    m.installer = Some(InstallerSpec {
        file: None,
        script: None,
        args: None,
        keep: None,
    });
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "installer"));
}

#[test]
fn ok_installer_with_script() {
    let mut m = minimal_valid_manifest();
    m.installer = Some(InstallerSpec {
        file: None,
        script: Some(hit_core::manifest::ScriptField::Single("Write-Host 'hi'".into())),
        args: None,
        keep: None,
    });
    let diag = validate(&m);
    assert!(
        !diag.errors().any(|d| d.field == "installer"),
        "有 script 时不应报错"
    );
}

// ============================================================================
// Warning 场景
// ============================================================================

#[test]
fn warning_unknown_license_spdx() {
    let mut m = minimal_valid_manifest();
    m.license = License::Identifier("MyCustomLicense".into());
    let diag = validate(&m);
    assert!(
        diag.warnings().any(|d| d.field == "license"),
        "未知 SPDX 应产生 warning"
    );
    assert!(!diag.has_errors(), "未知 SPDX 不应是 error");
}

#[test]
fn warning_http_url() {
    let mut m = minimal_valid_manifest();
    m.url = Some(OneOrMany::One("http://example.com/app.zip".into()));
    m.hash = Some(HashField::Single("a".repeat(64)));
    let diag = validate(&m);
    assert!(
        diag.warnings().any(|d| d.field == "url" && d.message.contains("http://")),
        "http:// URL 应产生安全警告"
    );
}

#[test]
fn warning_empty_env_set_value() {
    let mut m = minimal_valid_manifest();
    let mut env = std::collections::BTreeMap::new();
    env.insert("MY_VAR".into(), String::new());
    m.env_set = Some(env);
    let diag = validate(&m);
    assert!(
        diag.warnings().any(|d| d.field.contains("env_set")),
        "空 env_set 值应产生 warning"
    );
}

// ============================================================================
// Info 场景
// ============================================================================

#[test]
fn info_missing_maintainer_note() {
    let mut m = minimal_valid_manifest();
    m.maintainer_note = None;
    let diag = validate(&m);
    assert!(
        diag.infos().any(|d| d.field == "##"),
        "缺少 ## 应产生 info"
    );
}

#[test]
fn no_info_when_maintainer_note_present() {
    let mut m = minimal_valid_manifest();
    m.maintainer_note = Some(OneOrMany::One("Maintained by team".into()));
    let diag = validate(&m);
    assert!(
        !diag.infos().any(|d| d.field == "##"),
        "有 ## 时不应产生 info"
    );
}

// ============================================================================
// Diagnostics API
// ============================================================================

#[test]
fn diagnostics_into_result_ok_when_no_errors() {
    let m = minimal_valid_manifest();
    let diag = validate(&m);
    assert!(diag.into_result("test-app").is_ok());
}

#[test]
fn diagnostics_into_result_err_when_has_errors() {
    let mut m = minimal_valid_manifest();
    m.version = String::new();
    m.description = String::new();
    let diag = validate(&m);
    let result = diag.into_result("test-app");
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("test-app"), "错误消息应包含 app 名");
}

// ============================================================================
// 架构分支 url/hash 一致性
// ============================================================================

#[test]
fn error_arch_branch_url_hash_mismatch() {
    let mut m = minimal_valid_manifest();
    m.url = None;
    m.hash = None;
    m.architecture = Some(Architecture {
        x86_64: Some(ArchSpec {
            url: Some(OneOrMany::Many(vec![
                "https://a.com/1.zip".into(),
                "https://a.com/2.zip".into(),
                "https://a.com/3.zip".into(),
            ])),
            hash: Some(HashField::Multiple(vec![HashField::Single("a".repeat(64)), HashField::Single("b".repeat(64))])),
            ..Default::default()
        }),
        ..Default::default()
    });
    let diag = validate(&m);
    assert!(
        diag.errors().any(|d| d.field.contains("64bit") && d.message.contains("不一致")),
        "架构分支 url/hash 长度不一致应报错"
    );
}

// ============================================================================
// checkver URL 校验
// ============================================================================

#[test]
fn error_checkver_url_not_uri() {
    let mut m = minimal_valid_manifest();
    m.checkver = Some(CheckverField::Full(Box::new(Checkver {
        url: Some("not a url".into()),
        ..Default::default()
    })));
    let diag = validate(&m);
    assert!(diag.errors().any(|d| d.field == "checkver.url"));
}

// ============================================================================
// validate_or_err 便捷封装
// ============================================================================

#[test]
fn validate_or_err_convenience() {
    use hit_core::manifest::validate_or_err;

    let m = minimal_valid_manifest();
    assert!(validate_or_err(&m, "my-app").is_ok());

    let mut bad = minimal_valid_manifest();
    bad.version = String::new();
    assert!(validate_or_err(&bad, "my-app").is_err());
}
