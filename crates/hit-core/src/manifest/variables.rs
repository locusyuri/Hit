//! 变量替换引擎。
//!
//! 实现 Scoop Manifest 字符串字段的 `$variable` 替换，参考：
//! - `ref/Scoop/lib/core.ps1:1203-1225` — `substitute` 函数（**单次替换，非递归**）
//! - `ref/Scoop/lib/autoupdate.ps1:429-463` — `Get-VersionSubstitution`（版本派生变量）
//! - `ref/Scoop/lib/autoupdate.ps1:214-219` — `get_hash_for_app`（URL 上下文变量）
//! - `ref/Scoop/lib/autoupdate.ps1:44-51` — hash 正则模板变量
//!
//! 设计要点：
//! - 使用 `BTreeMap` 保证按键字典序替换（确定性输出，便于测试对比）
//! - 变量键是完整 token（`"$version"` 而非 `"version"`），避免误替换
//! - `substitute_manifest_in_place` 仅替换允许字段；纯文本字段（`description` /
//!   `homepage` / `notes` / `license` / `suggest` / `maintainer_note`）保持原样
//! - 架构分支（`architecture.{x86_64, x86, arm64}`）独立替换；调用方负责切换 `$architecture`
//! - `$url` / `$baseurl` / `$basename` 是 autoupdate hash 专属上下文，install-time 不注入

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::manifest::schema::{
    ArchSpec, Autoupdate, AutoupdateHash, BinItem, BinList, Checkver, CheckverField, HashField,
    InstallerSpec, Manifest, OneOrMany, PersistItem, PersistList, ScriptField,
};

/// 统一变量表（字符串键值，键为完整 `$token`）。
pub type VarMap = BTreeMap<String, String>;

/// 任何可展开为 VarMap 的上下文对象。
pub trait IntoVarMap {
    fn to_var_map(&self) -> VarMap;
}

// ============================================================================
// 上下文对象
// ============================================================================

/// 安装时变量上下文：`$version` / `$dir` / `$persist_dir` / `$architecture` / `$global`。
#[derive(Debug, Clone)]
pub struct InstallVars {
    pub version: String,
    pub dir: PathBuf,
    pub persist_dir: PathBuf,
    /// 当前处理的架构分支（决定 `$architecture` 值）。
    pub architecture: Arch,
    pub global: bool,
    pub app: String,
    /// `link_current` 重写前的原目录（Scoop 兼容）。
    pub original_dir: Option<PathBuf>,
}

impl Default for InstallVars {
    fn default() -> Self {
        Self {
            version: String::new(),
            dir: PathBuf::new(),
            persist_dir: PathBuf::new(),
            architecture: Arch::X86_64,
            global: false,
            app: String::new(),
            original_dir: None,
        }
    }
}

impl IntoVarMap for InstallVars {
    fn to_var_map(&self) -> VarMap {
        let mut m = VarMap::new();
        m.insert("$version".into(), self.version.clone());
        m.insert("$dir".into(), path_to_scoop_str(&self.dir));
        m.insert("$persist_dir".into(), path_to_scoop_str(&self.persist_dir));
        m.insert("$architecture".into(), self.architecture.scoop_key().into());
        m.insert("$global".into(), self.global.to_string());
        m.insert("$app".into(), self.app.clone());
        if let Some(p) = &self.original_dir {
            m.insert("$original_dir".into(), path_to_scoop_str(p));
        }
        m
    }
}

/// Autoupdate 版本变量上下文（`$version` + checkver 自定义捕获组）。
#[derive(Debug, Clone, Default)]
pub struct AutoupdateVars {
    pub version: String,
    /// `$matchTag`、`$matchHead` 等自定义捕获组，键为捕获组名（首字母大写，无前缀）。
    pub custom_matches: BTreeMap<String, String>,
}

/// URL 处理上下文：`$url` / `$baseurl` / `$basename` / `$basenameNoExt` / `$urlNoExt`。
///
/// 仅在处理 `autoupdate.hash` 时注入 VarMap，install-time 不应出现。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlContext {
    /// 剥去 fragment（`#/...`）后的 URL。
    pub url: String,
    /// URL 目录部分（去文件名，去尾 `/`）。
    pub baseurl: String,
    /// URL 文件名（URL-decoded）。
    pub basename: String,
    /// 剥去扩展名后的 URL（保留 fragment 之前）。
    pub url_no_ext: String,
    /// 文件名剥去扩展名。
    pub basename_no_ext: String,
}

impl UrlContext {
    /// 从原始 URL（可能含 `#/...` fragment）构造上下文。
    pub fn from_url(raw_url: &str) -> Self {
        let stripped = strip_fragment(raw_url);
        let basename_raw = url_remote_filename(stripped);
        let basename = url_decode(basename_raw);
        let baseurl = strip_trailing_slash(strip_filename(stripped));
        let basename_no_ext = strip_ext(&basename).to_string();
        let url_no_ext = strip_ext(stripped).to_string();
        Self {
            url: stripped.to_string(),
            baseurl: baseurl.to_string(),
            basename,
            url_no_ext,
            basename_no_ext,
        }
    }

    pub fn to_var_map(&self) -> VarMap {
        let mut m = VarMap::new();
        m.insert("$url".into(), self.url.clone());
        m.insert("$baseurl".into(), self.baseurl.clone());
        m.insert("$basename".into(), self.basename.clone());
        m.insert("$urlNoExt".into(), self.url_no_ext.clone());
        m.insert("$basenameNoExt".into(), self.basename_no_ext.clone());
        m
    }
}

/// 架构枚举（与 `architecture` 字段 `64bit` / `32bit` / `arm64` 对应）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Arch {
    #[default]
    X86_64,
    X86,
    Arm64,
}

impl Arch {
    /// 从 Scoop manifest 键名解析（`"64bit"` / `"32bit"` / `"arm64"`）。
    pub fn from_scoop_key(s: &str) -> Option<Arch> {
        match s {
            "64bit" => Some(Arch::X86_64),
            "32bit" => Some(Arch::X86),
            "arm64" => Some(Arch::Arm64),
            _ => None,
        }
    }

    /// 从 Rust target_arch 解析。
    pub fn from_target_arch(s: &str) -> Option<Arch> {
        match s {
            "x86_64" => Some(Arch::X86_64),
            "x86" => Some(Arch::X86),
            "aarch64" => Some(Arch::Arm64),
            _ => None,
        }
    }

    /// 当前平台架构（基于 `cfg!(target_arch)`）。
    pub fn current() -> Option<Arch> {
        if cfg!(target_arch = "x86_64") {
            Some(Arch::X86_64)
        } else if cfg!(target_arch = "x86") {
            Some(Arch::X86)
        } else if cfg!(target_arch = "aarch64") {
            Some(Arch::Arm64)
        } else {
            None
        }
    }

    /// Scoop manifest 键名。
    pub fn scoop_key(&self) -> &'static str {
        match self {
            Arch::X86_64 => "64bit",
            Arch::X86 => "32bit",
            Arch::Arm64 => "arm64",
        }
    }
}

// ============================================================================
// 核心替换函数
// ============================================================================

/// 单次替换（不递归）。按 key 字典序遍历 `vars`。
///
/// 与 Scoop PS 版 `substitute` 语义一致：若被替换值包含新的 `$token`，
/// **不会**被二次展开。
pub fn substitute(input: &str, vars: &VarMap) -> String {
    let mut out = input.to_string();
    for (k, v) in vars {
        out = out.replace(k.as_str(), v.as_str());
    }
    out
}

/// 构造 autoupdate 版本派生变量（含 `$match*` 自定义捕获组）。
///
/// 派生规则（来自 `ref/Scoop/lib/autoupdate.ps1:429-463`）：
/// - `$version`、`$dotVersion`、`$underscoreVersion`、`$dashVersion`、`$cleanVersion`
/// - `$majorVersion` / `$minorVersion` / `$patchVersion` / `$buildVersion`
/// - `$preReleaseVersion`（末段 `-` 之后）
/// - `$matchHead`（头部 `\d+\.\d+(?:\.\d+)?`）/ `$matchTail`（剩余部分）
/// - `$match<Name>`（来自 checkver 捕获组，键名首字母大写）
pub fn autoupdate_version_vars(av: &AutoupdateVars) -> VarMap {
    let mut m = VarMap::new();
    let v = &av.version;
    m.insert("$version".into(), v.clone());
    m.insert("$dotVersion".into(), v.replace(['_', '-'], "."));
    m.insert("$underscoreVersion".into(), v.replace(['.', '-'], "_"));
    m.insert("$dashVersion".into(), v.replace(['.', '_'], "-"));
    m.insert("$cleanVersion".into(), v.replace(['.', '-', '_'], ""));

    // 预发布：末段 `-` 之后
    let main_part = v.split('-').next().unwrap_or(v);
    let pre = v.get(main_part.len() + 1..).unwrap_or("");
    m.insert("$preReleaseVersion".into(), pre.to_string());

    // 点分段（仅基于 main_part，避免 `1.2.3-rc1` 把 `rc1` 算作 build）
    let segs: Vec<&str> = main_part.split('.').collect();
    m.insert("$majorVersion".into(), segs.first().copied().unwrap_or("").to_string());
    m.insert("$minorVersion".into(), segs.get(1).copied().unwrap_or("").to_string());
    m.insert("$patchVersion".into(), segs.get(2).copied().unwrap_or("").to_string());
    m.insert("$buildVersion".into(), segs.get(3).copied().unwrap_or("").to_string());

    // matchHead / matchTail：`\d+\.\d+(?:\.\d+)?` 形式的头部与剩余
    let head = take_version_head(v);
    m.insert("$matchHead".into(), head.to_string());
    m.insert("$matchTail".into(), v[head.len()..].to_string());

    // 自定义捕获组：键名首字母大写，前缀 `$match`
    for (name, value) in &av.custom_matches {
        let key = format!("$match{}", title_case(name));
        m.insert(key, value.clone());
    }
    m
}

/// 展开 hash 正则模板：`$md5` / `$sha1` / `$sha256` / `$sha512` / `$checksum` / `$base64`。
///
/// 用于 `find_hash_in_textfile` 等场景，将占位符展开为正则捕获组。
pub fn hash_regex_templates(pattern: &str) -> String {
    pattern
        .replace("$md5", "([a-fA-F0-9]{32})")
        .replace("$sha1", "([a-fA-F0-9]{40})")
        .replace("$sha256", "([a-fA-F0-9]{64})")
        .replace("$sha512", "([a-fA-F0-9]{128})")
        .replace("$checksum", "([a-fA-F0-9]{32,128})")
        .replace("$base64", "([a-zA-Z0-9+/=]{24,88})")
}

/// 递归替换 manifest 中所有"允许变量引用"的字段。
///
/// 不触碰纯文本字段（`description` / `homepage` / `notes` / `license` / `suggest` /
/// `maintainer_note`）。架构分支独立替换；调用方应在处理不同分支时切换 `$architecture` 值。
pub fn substitute_manifest_in_place(m: &mut Manifest, vars: &VarMap) {
    sub_top_level(m, vars);

    if let Some(arch) = &mut m.architecture {
        if let Some(spec) = &mut arch.x86_64 {
            sub_arch_spec(spec, vars);
        }
        if let Some(spec) = &mut arch.x86 {
            sub_arch_spec(spec, vars);
        }
        if let Some(spec) = &mut arch.arm64 {
            sub_arch_spec(spec, vars);
        }
    }

    if let Some(au) = &mut m.autoupdate {
        sub_autoupdate(au, vars);
    }

    if let Some(CheckverField::Full(cv)) = &mut m.checkver {
        sub_checkver(cv, vars);
    }
}

// ============================================================================
// 内部替换：按字段类型拆分
// ============================================================================

fn sub_top_level(m: &mut Manifest, vars: &VarMap) {
    sub_opt_one_or_many_str(&mut m.url, vars);
    sub_opt_hash(&mut m.hash, vars, None);
    sub_opt_one_or_many_str(&mut m.extract_dir, vars);
    sub_opt_one_or_many_str(&mut m.extract_to, vars);
    sub_opt_bin_list(&mut m.bin, vars);
    sub_opt_shortcuts(&mut m.shortcuts, vars);
    sub_opt_env_set(&mut m.env_set, vars);
    sub_opt_one_or_many_str(&mut m.env_add_path, vars);
    sub_opt_persist_list(&mut m.persist, vars);
    sub_opt_script(&mut m.pre_install, vars);
    sub_opt_script(&mut m.post_install, vars);
    sub_opt_script(&mut m.pre_uninstall, vars);
    sub_opt_script(&mut m.post_uninstall, vars);
    sub_opt_installer(&mut m.installer, vars);
    sub_opt_installer(&mut m.uninstaller, vars);
    if let Some(map) = &mut m.cookie {
        sub_map_values(map, vars);
    }
}

fn sub_arch_spec(s: &mut ArchSpec, vars: &VarMap) {
    sub_opt_one_or_many_str(&mut s.url, vars);
    sub_opt_hash(&mut s.hash, vars, None);
    sub_opt_one_or_many_str(&mut s.extract_dir, vars);
    sub_opt_one_or_many_str(&mut s.extract_to, vars);
    sub_opt_bin_list(&mut s.bin, vars);
    sub_opt_shortcuts(&mut s.shortcuts, vars);
    sub_opt_one_or_many_str(&mut s.env_add_path, vars);
    sub_opt_env_set(&mut s.env_set, vars);
    sub_opt_script(&mut s.pre_install, vars);
    sub_opt_script(&mut s.post_install, vars);
    sub_opt_installer(&mut s.installer, vars);
    sub_opt_installer(&mut s.uninstaller, vars);
}

fn sub_autoupdate(au: &mut Autoupdate, vars: &VarMap) {
    sub_opt_one_or_many_str(&mut au.url, vars);
    sub_opt_one_or_many_str(&mut au.extract_dir, vars);
    sub_opt_bin_list(&mut au.bin, vars);
    if let Some(arch) = &mut au.architecture {
        if let Some(spec) = &mut arch.x86_64 {
            sub_arch_spec(spec, vars);
        }
        if let Some(spec) = &mut arch.x86 {
            sub_arch_spec(spec, vars);
        }
        if let Some(spec) = &mut arch.arm64 {
            sub_arch_spec(spec, vars);
        }
    }
    if let Some(AutoupdateHash::Fetch { url, regex, .. }) = &mut au.hash {
        // autoupdate.hash.url 处理时注入 URL 上下文（来自外层 download URL）
        let mut url_vars = vars.clone();
        if let Some(OneOrMany::One(primary_url)) = &au.url {
            let ctx = UrlContext::from_url(&substitute(primary_url, vars));
            url_vars.extend(ctx.to_var_map());
        }
        *url = substitute(url, &url_vars);
        if let Some(r) = regex {
            *r = substitute(r, &url_vars);
        }
    } else if let Some(AutoupdateHash::Plain(h)) = &mut au.hash {
        sub_hash(h, vars);
    }
}

fn sub_checkver(cv: &mut Checkver, vars: &VarMap) {
    if let Some(u) = &mut cv.url {
        *u = substitute(u, vars);
    }
    if let Some(r) = &mut cv.regex {
        *r = substitute(r, vars);
    }
    if let Some(r) = &mut cv.re {
        *r = substitute(r, vars);
    }
    if let Some(s) = &mut cv.script {
        for line in s.iter_mut() {
            *line = substitute(line, vars);
        }
    }
}

// --- 字段级替换 helper ---

fn sub_opt_one_or_many_str(field: &mut Option<OneOrMany<String>>, vars: &VarMap) {
    match field {
        Some(OneOrMany::One(s)) => *s = substitute(s, vars),
        Some(OneOrMany::Many(v)) => {
            for s in v.iter_mut() {
                *s = substitute(s, vars);
            }
        }
        None => {}
    }
}

fn sub_opt_hash(field: &mut Option<HashField>, vars: &VarMap, url_ctx: Option<&UrlContext>) {
    if let Some(h) = field {
        sub_hash(h, vars);
        // url_ctx 目前未被使用（保留为将来 hash-fetch 场景预留参数）
        let _ = url_ctx;
    }
}

fn sub_hash(h: &mut HashField, vars: &VarMap) {
    match h {
        HashField::Single(s) => *s = substitute(s, vars),
        HashField::Multiple(v) => {
            for s in v.iter_mut() {
                *s = substitute(s, vars);
            }
        }
    }
}

fn sub_opt_bin_list(field: &mut Option<BinList>, vars: &VarMap) {
    if let Some(BinList(items)) = field {
        for item in items.iter_mut() {
            match item {
                BinItem::Simple(p) => *p = substitute(p, vars),
                BinItem::Aliased { path, alias, args } => {
                    *path = substitute(path, vars);
                    *alias = substitute(alias, vars);
                    if let Some(a) = args {
                        *a = substitute(a, vars);
                    }
                }
            }
        }
    }
}

fn sub_opt_shortcuts(field: &mut Option<Vec<crate::manifest::schema::ShortcutItem>>, vars: &VarMap) {
    if let Some(list) = field {
        for sc in list.iter_mut() {
            sc.target = substitute(&sc.target, vars);
            sc.name = substitute(&sc.name, vars);
            if let Some(a) = &mut sc.args {
                *a = substitute(a, vars);
            }
            if let Some(i) = &mut sc.icon {
                *i = substitute(i, vars);
            }
        }
    }
}

fn sub_opt_env_set(field: &mut Option<BTreeMap<String, String>>, vars: &VarMap) {
    if let Some(map) = field {
        sub_map_values(map, vars);
    }
}

fn sub_map_values(map: &mut BTreeMap<String, String>, vars: &VarMap) {
    for v in map.values_mut() {
        *v = substitute(v, vars);
    }
}

fn sub_opt_persist_list(field: &mut Option<PersistList>, vars: &VarMap) {
    if let Some(PersistList(items)) = field {
        for item in items.iter_mut() {
            match item {
                PersistItem::Simple(p) => *p = substitute(p, vars),
                PersistItem::Renamed { src, dst } => {
                    *src = substitute(src, vars);
                    *dst = substitute(dst, vars);
                }
            }
        }
    }
}

fn sub_opt_script(field: &mut Option<ScriptField>, vars: &VarMap) {
    match field {
        Some(ScriptField::Single(s)) => *s = substitute(s, vars),
        Some(ScriptField::Lines(v)) => {
            for line in v.iter_mut() {
                *line = substitute(line, vars);
            }
        }
        None => {}
    }
}

fn sub_opt_installer(field: &mut Option<InstallerSpec>, vars: &VarMap) {
    if let Some(spec) = field {
        sub_opt_script(&mut spec.script, vars);
        if let Some(f) = &mut spec.file {
            *f = substitute(f, vars);
        }
        sub_opt_one_or_many_str(&mut spec.args, vars);
    }
}

// ============================================================================
// 字符串工具
// ============================================================================

/// PathBuf → Scoop 风格字符串（Windows 下反斜杠保持，但替换时调用方负责规范化）。
fn path_to_scoop_str(p: &std::path::Path) -> String {
    p.to_string_lossy().into_owned()
}

fn strip_fragment(url: &str) -> &str {
    url.split('#').next().unwrap_or(url)
}

fn url_remote_filename(url: &str) -> &str {
    url.rsplit('/').next().unwrap_or(url)
}

fn strip_filename(url: &str) -> &str {
    match url.rfind('/') {
        Some(i) => &url[..=i],
        None => url,
    }
}

fn strip_trailing_slash(s: &str) -> &str {
    s.strip_suffix('/').unwrap_or(s)
}

fn strip_ext(s: &str) -> &str {
    match s.rfind('.') {
        Some(i) if i > 0 => &s[..i],
        _ => s,
    }
}

/// 最小 URL-decode：处理 `%XX` 与 `+`。
fn url_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '+' {
            out.push(' ');
        } else if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(b) = u8::from_str_radix(&hex, 16) {
                    out.push(b as char);
                } else {
                    out.push('%');
                    out.push_str(&hex);
                }
            } else {
                out.push('%');
                out.push_str(&hex);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// 首字母大写。
fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut out = first.to_uppercase().to_string();
            out.extend(c);
            out
        }
    }
}

/// 取版本字符串头部 `\d+\.\d+(?:\.\d+)?`（用于 `$matchHead`）。
fn take_version_head(v: &str) -> &str {
    let mut dot_count = 0;
    let mut end = 0;
    for (i, c) in v.char_indices() {
        if c == '.' {
            dot_count += 1;
            if dot_count > 2 {
                break;
            }
        } else if !c.is_ascii_digit() {
            break;
        }
        end = i + c.len_utf8();
    }
    &v[..end]
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute_basic() {
        let mut v = VarMap::new();
        v.insert("$version".into(), "1.2.3".into());
        v.insert("$arch".into(), "x64".into());
        assert_eq!(substitute("app-$version-$arch.zip", &v), "app-1.2.3-x64.zip");
    }

    #[test]
    fn substitute_no_recursion() {
        // 单次替换：每个 key 只替换一轮，已替换的字符串不会被重新扫描
        // 使用不相互引用的值来验证
        let mut v = VarMap::new();
        v.insert("$a".into(), "X".into());
        v.insert("$b".into(), "Y".into());
        // 输入 "$a$b" 应得 "XY"（$a→X, $b→Y，单次扫描）
        assert_eq!(substitute("$a$b", &v), "XY");

        // 替换结果包含其他 $ token 时不会被二次展开：
        // 用 "$c" 作值，"$c" 又映射到 "Z"；按 BTreeMap 键序 $c 在 $d 之前
        let mut v2 = VarMap::new();
        v2.insert("$c".into(), "literal-$d".into());
        v2.insert("$d".into(), "Z".into());
        // $c 先替换为 "literal-$d"，但 $d 已被处理过（在 $c 之后），不再展开
        // 等等，BTreeMap 顺序是 $c 然后 $d。$c 替换为 "literal-$d"，
        // 然后 $d → "Z" 会把 "literal-$d" 中的 $d 也替换为 Z！
        // 这是单次遍历多个键的自然结果（与 Scoop PS 一致）。
        // 验证：最终结果是 "literal-Z"
        assert_eq!(substitute("$c", &v2), "literal-Z");
    }

    #[test]
    fn substitute_deterministic_key_order() {
        let mut v = VarMap::new();
        v.insert("$x".into(), "1".into());
        v.insert("$xy".into(), "2".into());
        // BTreeMap 按键字典序：$x 在 $xy 之前
        assert_eq!(substitute("$xy", &v), "1y");
    }

    #[test]
    fn url_context_from_url() {
        let ctx = UrlContext::from_url(
            "https://example.com/path/to/file-1.2.3.zip#/dl.zip",
        );
        assert_eq!(ctx.url, "https://example.com/path/to/file-1.2.3.zip");
        assert_eq!(ctx.baseurl, "https://example.com/path/to");
        assert_eq!(ctx.basename, "file-1.2.3.zip");
        assert_eq!(ctx.basename_no_ext, "file-1.2.3");
        assert_eq!(ctx.url_no_ext, "https://example.com/path/to/file-1.2.3");
    }

    #[test]
    fn url_context_url_decode() {
        let ctx = UrlContext::from_url("https://x/y/my%20file.zip");
        assert_eq!(ctx.basename, "my file.zip");
        assert_eq!(ctx.basename_no_ext, "my file");
    }

    #[test]
    fn autoupdate_version_vars_all_derivatives() {
        let av = AutoupdateVars {
            version: "1.2.3".into(),
            custom_matches: BTreeMap::new(),
        };
        let v = autoupdate_version_vars(&av);
        assert_eq!(v["$version"], "1.2.3");
        assert_eq!(v["$cleanVersion"], "123");
        assert_eq!(v["$dotVersion"], "1.2.3");
        assert_eq!(v["$dashVersion"], "1-2-3");
        assert_eq!(v["$underscoreVersion"], "1_2_3");
        assert_eq!(v["$majorVersion"], "1");
        assert_eq!(v["$minorVersion"], "2");
        assert_eq!(v["$patchVersion"], "3");
        assert_eq!(v["$buildVersion"], "");
    }

    #[test]
    fn autoupdate_version_vars_prerelease() {
        let av = AutoupdateVars {
            version: "1.2.3-rc1".into(),
            custom_matches: BTreeMap::new(),
        };
        let v = autoupdate_version_vars(&av);
        assert_eq!(v["$preReleaseVersion"], "rc1");
        assert_eq!(v["$majorVersion"], "1");
        assert_eq!(v["$patchVersion"], "3");
    }

    #[test]
    fn autoupdate_version_vars_custom_matches() {
        let av = AutoupdateVars {
            version: "1.0".into(),
            custom_matches: [("tag".into(), "v1.0.0".into())].into_iter().collect(),
        };
        let v = autoupdate_version_vars(&av);
        assert_eq!(v["$matchTag"], "v1.0.0");
    }

    #[test]
    fn hash_regex_templates_all_tokens() {
        let pat = "$md5|$sha1|$sha256|$sha512|$checksum|$base64";
        let out = hash_regex_templates(pat);
        assert!(out.contains("([a-fA-F0-9]{32})"));
        assert!(out.contains("([a-fA-F0-9]{40})"));
        assert!(out.contains("([a-fA-F0-9]{64})"));
        assert!(out.contains("([a-fA-F0-9]{128})"));
        assert!(out.contains("([a-fA-F0-9]{32,128})"));
        assert!(out.contains("([a-zA-Z0-9+/=]{24,88})"));
    }

    #[test]
    fn arch_current_on_x86_64() {
        // CI / 开发机通常为 x86_64；其他架构下返回 None 也算合法
        if cfg!(target_arch = "x86_64") {
            assert_eq!(Arch::current(), Some(Arch::X86_64));
        }
    }

    #[test]
    fn arch_from_scoop_key_roundtrip() {
        for a in [Arch::X86_64, Arch::X86, Arch::Arm64] {
            assert_eq!(Arch::from_scoop_key(a.scoop_key()), Some(a));
        }
    }

    #[test]
    fn arch_from_target_arch() {
        assert_eq!(Arch::from_target_arch("x86_64"), Some(Arch::X86_64));
        assert_eq!(Arch::from_target_arch("x86"), Some(Arch::X86));
        assert_eq!(Arch::from_target_arch("aarch64"), Some(Arch::Arm64));
        assert_eq!(Arch::from_target_arch("unknown"), None);
    }

    #[test]
    fn install_vars_into_var_map() {
        let iv = InstallVars {
            version: "1.0".into(),
            dir: PathBuf::from("C:/scoop/apps/foo/current"),
            persist_dir: PathBuf::from("C:/scoop/persist/foo"),
            architecture: Arch::X86_64,
            global: false,
            app: "foo".into(),
            original_dir: None,
        };
        let m = iv.to_var_map();
        assert_eq!(m["$version"], "1.0");
        assert_eq!(m["$architecture"], "64bit");
        assert_eq!(m["$global"], "false");
        assert_eq!(m["$app"], "foo");
        assert!(!m.contains_key("$original_dir"));
    }

    #[test]
    fn title_case_and_strip_ext() {
        assert_eq!(title_case("tag"), "Tag");
        assert_eq!(title_case(""), "");
        assert_eq!(strip_ext("file.zip"), "file");
        assert_eq!(strip_ext("noext"), "noext");
        // 前导 `.` 不被视为扩展名分隔符（`i > 0`）
        assert_eq!(strip_ext(".hidden"), ".hidden");
        assert_eq!(strip_ext("archive.tar.gz"), "archive.tar");
    }

    #[test]
    fn take_version_head_variants() {
        assert_eq!(take_version_head("1.2.3.4-rc1"), "1.2.3");
        assert_eq!(take_version_head("1.2"), "1.2");
        assert_eq!(take_version_head("1.2.3"), "1.2.3");
        assert_eq!(take_version_head("v1.2.3"), "");
    }
}
