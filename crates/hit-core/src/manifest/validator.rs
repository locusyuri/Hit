//! Manifest 验证器。
//!
//! 基于规则的校验器，返回 `Diagnostics`（结构化错误/警告/信息集合）。
//! 规则来源：
//! - 结构规则：`ref/Scoop/schema.json` 的 JSON Schema 定义
//! - 语义规则：Scoop PS 版校验（URL 格式、hash 算法前缀等）
//! - 警告规则：工程最佳实践（SPDX、HTTPS、bucket/name 格式等）
//!
//! `validate()` 永不 panic，所有异常路径降级为 error 诊断。

use std::sync::OnceLock;

use regex::Regex;

use crate::manifest::diagnostic::Diagnostics;
use crate::manifest::schema::{
    ArchSpec, Architecture, Autoupdate, AutoupdateHash, CheckverField, HashField, InstallerSpec,
    License, Manifest, OneOrMany,
};

/// 验证 Manifest，返回诊断集合（无 panic）。
pub fn validate(m: &Manifest) -> Diagnostics {
    let mut d = Diagnostics::new();

    check_version(m, &mut d);
    check_description(m, &mut d);
    check_homepage(m, &mut d);
    check_license(m, &mut d);
    check_url_hash_consistency(m, &mut d);
    check_at_least_one_url(m, &mut d);
    check_env_set(m, &mut d);
    check_installer(m, &mut d);
    check_checkver(m, &mut d);
    check_suggest_depends(m, &mut d);
    check_http_warnings(m, &mut d);
    check_maintainer_note(m, &mut d);

    d
}

/// 便捷封装：仅在有 error 时返回 Err（丢弃 warning/info）。
pub fn validate_or_err(m: &Manifest, app: &str) -> hit_common::error::Result<()> {
    validate(m).into_result(app)
}

// ============================================================================
// 规则实现
// ============================================================================

fn check_version(m: &Manifest, d: &mut Diagnostics) {
    if m.version.is_empty() {
        d.push_error("version", "manifest 缺少 version 字段");
        return;
    }
    if !version_regex().is_match(&m.version) {
        d.push_error(
            "version",
            format!(
                "version '{}' 包含非法字符（仅允许 \\w.-+_）",
                m.version
            ),
        );
    }
}

fn check_description(m: &Manifest, d: &mut Diagnostics) {
    if m.description.is_empty() {
        d.push_error("description", "manifest 缺少 description 字段");
    }
}

fn check_homepage(m: &Manifest, d: &mut Diagnostics) {
    if m.homepage.is_empty() {
        d.push_error("homepage", "manifest 缺少 homepage 字段");
    } else if !looks_like_uri(&m.homepage) {
        d.push_error(
            "homepage",
            format!("homepage '{}' 不是合法 URI（需 http(s):// 或 file:// 前缀）", m.homepage),
        );
    }
}

fn check_license(m: &Manifest, d: &mut Diagnostics) {
    let id = m.license.identifier();
    if id.is_empty() {
        d.push_error("license", "manifest 缺少 license 字段");
        return;
    }
    if !is_known_spdx(id) {
        d.push_warning(
            "license",
            format!("license identifier '{}' 不是已知 SPDX（仅警告）", id),
        );
    }
    if let License::Detailed { url: Some(u), .. } = &m.license
        && !looks_like_uri(u)
    {
        d.push_error("license.url", format!("license.url '{}' 不是合法 URI", u));
    }
}

fn check_at_least_one_url(m: &Manifest, d: &mut Diagnostics) {
    let top_has = m.url.is_some();
    let arch_has = m
        .architecture
        .as_ref()
        .map(any_arch_has_url)
        .unwrap_or(false);
    if !top_has && !arch_has {
        d.push_error(
            "url",
            "manifest 缺少下载源：顶层 url 与 architecture.<arch>.url 均为空",
        );
    }
}

fn check_url_hash_consistency(m: &Manifest, d: &mut Diagnostics) {
    // 顶层 url × hash 数组长度一致性
    if let (Some(url), Some(hash)) = (&m.url, &m.hash) {
        check_url_hash_pair("url", url, "hash", hash, d);
    }
    // 每个架构分支独立检查
    if let Some(a) = &m.architecture {
        check_arch_url_hash(a, "architecture", d);
    }
}

fn check_arch_url_hash(a: &Architecture, prefix: &str, d: &mut Diagnostics) {
    for (key, spec) in [
        ("64bit", a.x86_64.as_ref()),
        ("32bit", a.x86.as_ref()),
        ("arm64", a.arm64.as_ref()),
    ] {
        if let Some(s) = spec
            && let (Some(url), Some(hash)) = (&s.url, &s.hash)
        {
            let p = format!("{prefix}.{key}");
            check_url_hash_pair(
                &format!("{p}.url"),
                url,
                &format!("{p}.hash"),
                hash,
                d,
            );
        }
    }
}

fn check_url_hash_pair(
    url_field: &str,
    url: &OneOrMany<String>,
    hash_field: &str,
    hash: &HashField,
    d: &mut Diagnostics,
) {
    let url_len = url.len();
    let hash_len = hash.values().len();
    if url_len > 1 && hash_len > 1 && url_len != hash_len {
        d.push_error(
            hash_field,
            format!(
                "{hash_field} 数组长度 ({hash_len}) 与 {url_field} 数组长度 ({url_len}) 不一致"
            ),
        );
    }
    for h in hash.values() {
        if !hash_regex().is_match(h) {
            d.push_error(
                hash_field,
                format!("hash '{h}' 格式非法（需 [a-fA-F0-9]{{40|64|128}} 或 algo:hex）"),
            );
        }
    }
}

fn check_env_set(m: &Manifest, d: &mut Diagnostics) {
    if let Some(map) = &m.env_set {
        for (k, v) in map {
            if v.is_empty() {
                d.push_warning(
                    format!("env_set.{k}"),
                    "env_set 值为空字符串（无实际作用）",
                );
            }
        }
    }
}

fn check_installer(m: &Manifest, d: &mut Diagnostics) {
    check_installer_spec("installer", m.installer.as_ref(), d);
    check_installer_spec("uninstaller", m.uninstaller.as_ref(), d);
}

fn check_installer_spec(field: &str, spec: Option<&InstallerSpec>, d: &mut Diagnostics) {
    if let Some(s) = spec
        && s.script.is_none()
        && s.file.is_none()
    {
        d.push_error(
            field,
            format!("{field} 需至少声明 file 或 script（当前均为空）"),
        );
    }
}

fn check_checkver(m: &Manifest, d: &mut Diagnostics) {
    let cv = match &m.checkver {
        Some(CheckverField::Full(c)) => c,
        _ => return,
    };
    if let Some(r) = &cv.regex
        && let Err(e) = Regex::new(r)
    {
        d.push_error("checkver.regex", format!("正则语法错误：{e}"));
    }
    if let Some(r) = &cv.re
        && let Err(e) = Regex::new(r)
    {
        d.push_error("checkver.re", format!("正则语法错误：{e}"));
    }
    if let Some(u) = &cv.url
        && !looks_like_uri(u)
    {
        d.push_error("checkver.url", format!("checkver.url '{u}' 不是合法 URI"));
    }
    if let Some(g) = &cv.github
        && !looks_like_uri(g)
    {
        d.push_error(
            "checkver.github",
            format!("checkver.github '{g}' 不是合法 URI"),
        );
    }
    if let Some(s) = &cv.sourceforge
        && s.is_empty()
    {
        d.push_error("checkver.sourceforge", "checkver.sourceforge 不应为空");
    }
}

fn check_suggest_depends(m: &Manifest, d: &mut Diagnostics) {
    if let Some(sug) = &m.suggest {
        for (display, value) in sug {
            if !looks_like_bucket_ref(value) {
                d.push_warning(
                    format!("suggest.{display}"),
                    format!("suggest 值 '{value}' 非 bucket/name 格式（Scoop 约定）"),
                );
            }
        }
    }
    if let Some(dep) = &m.depends {
        for v in dep.as_slice() {
            if !looks_like_bucket_ref(v) {
                d.push_warning(
                    "depends",
                    format!("depends 值 '{v}' 非 bucket/name 格式（Scoop 约定）"),
                );
            }
        }
    }
}

fn check_http_warnings(m: &Manifest, d: &mut Diagnostics) {
    if let Some(OneOrMany::One(u)) = &m.url
        && u.starts_with("http://")
    {
        d.push_warning("url", "使用 http:// 而非 https://（安全建议）");
    }
    if let Some(au) = &m.autoupdate {
        check_autoupdate_http(au, d);
    }
}

fn check_autoupdate_http(au: &Autoupdate, d: &mut Diagnostics) {
    if let Some(OneOrMany::One(u)) = &au.url
        && u.starts_with("http://")
    {
        d.push_warning("autoupdate.url", "使用 http:// 而非 https://（安全建议）");
    }
    if let Some(AutoupdateHash::Fetch { url, .. }) = &au.hash
        && url.starts_with("http://")
    {
        d.push_warning(
            "autoupdate.hash.url",
            "使用 http:// 而非 https://（安全建议）",
        );
    }
}

fn check_maintainer_note(m: &Manifest, d: &mut Diagnostics) {
    if m.maintainer_note.is_none() {
        d.push_info("##", "缺少 maintainer 注释（Scoop 鼓励但非强制）");
    }
}

// ============================================================================
// 工具函数
// ============================================================================

fn version_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[\w.\-+_]+$").expect("version regex 应能编译"))
}

fn hash_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"^(?:[a-fA-F0-9]{40}|[a-fA-F0-9]{64}|[a-fA-F0-9]{128}|md5:[a-fA-F0-9]{32}|sha1:[a-fA-F0-9]{40}|sha256:[a-fA-F0-9]{64}|sha512:[a-fA-F0-9]{128})$",
        )
        .expect("hash regex 应能编译")
    })
}

fn looks_like_uri(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://") || s.starts_with("file://")
}

fn any_arch_has_url(a: &Architecture) -> bool {
    let has = |s: Option<&ArchSpec>| s.and_then(|x| x.url.as_ref()).is_some();
    has(a.x86_64.as_ref()) || has(a.x86.as_ref()) || has(a.arm64.as_ref())
}

/// 粗略判断是否为 Scoop 风格的 `bucket/name` 引用。
///
/// - `perl`、`git`：单名（main bucket 默认）
/// - `extras/vcredist2022`：bucket/name
/// - `java/openjdk17`：bucket/name
fn looks_like_bucket_ref(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('/').collect();
    match parts.len() {
        1 => !parts[0].is_empty(),
        2 => !parts[0].is_empty() && !parts[1].is_empty(),
        _ => false,
    }
}

/// 已知 SPDX 标识符的最小集合（仅用于 warning 判断；非严格校验）。
///
/// 覆盖常见开源许可证；未命中仅触发 warning，不报错。
const KNOWN_SPDX: &[&str] = &[
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "GPL-2.0-only",
    "GPL-2.0-or-later",
    "GPL-3.0-only",
    "GPL-3.0-or-later",
    "LGPL-2.1-only",
    "LGPL-2.1-or-later",
    "LGPL-3.0-only",
    "LGPL-3.0-or-later",
    "MPL-2.0",
    "ISC",
    "Unlicense",
    "Artistic-2.0",
    "Zlib",
    "BSL-1.0",
    "CC0-1.0",
    "CC-BY-4.0",
    "CC-BY-SA-4.0",
    "Python-2.0",
    "OpenSSL",
    "WTFPL",
    "0BSD",
    "Public Domain",
    "Freeware",
];

fn is_known_spdx(id: &str) -> bool {
    // 支持复合标识（"BSD-2-Clause, BSD-3-Clause, LGPL-2.1-or-later"）：拆分逐个检查
    id.split([',', '/'])
        .map(str::trim)
        .all(|part| part.is_empty() || KNOWN_SPDX.iter().any(|k| k.eq_ignore_ascii_case(part)))
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_regex_accepts_common_forms() {
        let re = version_regex();
        for v in ["1.2.3", "1.2.3-rc1", "2.54.0.windows.1", "1.2.3+build", "v1.2"] {
            assert!(re.is_match(v), "应接受 {v}");
        }
    }

    #[test]
    fn version_regex_rejects_bad() {
        let re = version_regex();
        for v in ["1.2.3!", "foo bar", ""] {
            assert!(!re.is_match(v), "应拒绝 {v:?}");
        }
    }

    #[test]
    fn hash_regex_accepts_all_forms() {
        let re = hash_regex();
        assert!(re.is_match(&"a".repeat(40)));
        assert!(re.is_match(&"b".repeat(64)));
        assert!(re.is_match(&"c".repeat(128)));
        assert!(re.is_match(&format!("sha1:{}", "a".repeat(40))));
        assert!(re.is_match(&format!("sha256:{}", "a".repeat(64))));
        assert!(re.is_match(&format!("sha512:{}", "a".repeat(128))));
        assert!(re.is_match(&format!("md5:{}", "a".repeat(32))));
    }

    #[test]
    fn hash_regex_rejects_bad() {
        let re = hash_regex();
        assert!(!re.is_match("not-a-hash"));
        assert!(!re.is_match(&"g".repeat(64))); // 非 hex
        assert!(!re.is_match("sha256:xyz"));
    }

    #[test]
    fn spdx_recognizes_common() {
        assert!(is_known_spdx("MIT"));
        assert!(is_known_spdx("GPL-2.0-only"));
        // 复合标识
        assert!(is_known_spdx("BSD-2-Clause, BSD-3-Clause, LGPL-2.1-or-later"));
    }

    #[test]
    fn spdx_rejects_unknown() {
        assert!(!is_known_spdx("NotALicense"));
    }

    #[test]
    fn bucket_ref_variants() {
        assert!(looks_like_bucket_ref("perl"));
        assert!(looks_like_bucket_ref("extras/vcredist2022"));
        assert!(!looks_like_bucket_ref(""));
        assert!(!looks_like_bucket_ref("a/b/c"));
    }
}
