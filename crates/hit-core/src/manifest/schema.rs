//! Scoop Manifest 数据结构定义。
//!
//! 完整兼容 Scoop 官方 Manifest JSON 格式（参见 `ref/Main/bucket/*.json`），
//! 并为 Hit 扩展字段预留占位（按 TODO 1.2.2 要求本阶段仅声明不解析）。
//!
//! 多态字段建模方式：
//! - `OneOrMany<T>` — JSON 单值或数组（`url` / `env_add_path` / `depends` 等）
//! - `BinList` / `BinItem` — `bin`：`string | string[] | (tuple[2..=5])[]`
//! - `PersistList` / `PersistItem` — `persist`：`string | (string | tuple[2])[]`
//! - `ShortcutItem` — `shortcuts`：`tuple[2..=4][]`
//! - `ScriptField` — 脚本字段：`string | string[]`
//! - `License` — `string | { identifier, url? }`
//! - `HashField` — `string | string[]`（算法由长度推断或 `algo:` 前缀声明）
//! - `CheckverField` — `string | { github | url | script | ... }`
//! - `AutoupdateHash` — 普通 hash 或 `{ url, regex }` 抓取对象

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

// ============================================================================
// 顶层 Manifest
// ============================================================================

/// Scoop Manifest 顶层结构。
///
/// 必填字段（按 Scoop 规范全部 1571 个真实 manifest 均存在）：
/// `version` / `description` / `homepage` / `license`。
///
/// 其他字段均为可选，使用 `Option<T>` + `#[serde(default)]` 容忍缺省。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Manifest {
    // -------- 必填 --------
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: License,

    // -------- 下载源 --------
    pub architecture: Option<Architecture>,
    pub url: Option<OneOrMany<String>>,
    pub hash: Option<HashField>,
    pub extract_dir: Option<OneOrMany<String>>,
    pub extract_to: Option<OneOrMany<String>>,
    pub cookie: Option<BTreeMap<String, String>>,

    // -------- 安装后集成 --------
    pub bin: Option<BinList>,
    pub shortcuts: Option<Vec<ShortcutItem>>,
    pub env_set: Option<BTreeMap<String, String>>,
    pub env_add_path: Option<OneOrMany<String>>,
    pub persist: Option<PersistList>,

    // -------- 生命周期脚本 --------
    pub pre_install: Option<ScriptField>,
    pub post_install: Option<ScriptField>,
    pub pre_uninstall: Option<ScriptField>,
    pub post_uninstall: Option<ScriptField>,
    pub installer: Option<InstallerSpec>,
    pub uninstaller: Option<InstallerSpec>,

    // -------- 依赖 --------
    pub depends: Option<OneOrMany<String>>,
    /// `suggest` 字段:键为建议安装的软件类别名,值为该类别下的软件列表
    /// (支持单字符串或字符串数组,参考 digital.json)
    pub suggest: Option<BTreeMap<String, OneOrMany<String>>>,

    // -------- 元信息 --------
    pub notes: Option<ScriptField>,
    pub checkver: Option<CheckverField>,
    pub autoupdate: Option<Autoupdate>,
    pub innosetup: Option<bool>,
    pub psmodule: Option<PowerShellModule>,

    /// Maintainer 注释，键名 `"##"`（Scoop 约定）。
    #[serde(rename = "##")]
    pub maintainer_note: Option<String>,

    // -------- Hit 扩展字段占位（Phase 1 不解析） --------
    // `alias` / `bucket_priority` / `bucket_maintainer` / `bucket_last_update`
    // / `dependencies` / `health_check` / `bundle` / `shadow` / `mirrors`
    // / `lifecycle` / `monitor` / `dev` —— 在对应功能阶段按需添加。
    // 反序列化时通过 serde_json 默认 `deny_unknown_fields` 关闭（默认行为）
    // 忽略未知字段，从而兼容未来的 Hit 扩展。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
}

impl Manifest {
    /// 返回 depends 列表的统一视图（`Vec<&str>`）。
    ///
    /// - `None` → 空 Vec
    /// - `One("perl")` → `["perl"]`
    /// - `Many(["a", "b"])` → `["a", "b"]`
    ///
    /// 对应 Scoop PS `@($manifest.depends)`（`ref/Scoop/lib/depends.ps1:45`）。
    pub fn depends_list(&self) -> Vec<&str> {
        match &self.depends {
            None => Vec::new(),
            Some(om) => om.as_slice().iter().map(String::as_str).collect(),
        }
    }
}

// ============================================================================
// 通用多态：OneOrMany<T>
// ============================================================================

/// 单值或数组（`url` / `env_add_path` / `depends` / `extract_dir` / `extract_to`）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    /// 将单值/数组统一展开为 `Vec<T>`。
    pub fn into_vec(self) -> Vec<T> {
        match self {
            OneOrMany::One(v) => vec![v],
            OneOrMany::Many(v) => v,
        }
    }

    /// 借用视图：返回元素切片（不消耗所有权）。
    pub fn as_slice(&self) -> &[T] {
        match self {
            OneOrMany::One(v) => std::slice::from_ref(v),
            OneOrMany::Many(v) => v.as_slice(),
        }
    }

    /// 元素数量。
    pub fn len(&self) -> usize {
        match self {
            OneOrMany::One(_) => 1,
            OneOrMany::Many(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ============================================================================
// ScriptField：string | string[]
// ============================================================================

/// 脚本字段（`pre_install` / `post_install` / `pre_uninstall` / `post_uninstall` / `notes`）。
///
/// JSON 中既可以是单行字符串，也可以是多行字符串数组。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptField {
    Single(String),
    Lines(Vec<String>),
}

impl ScriptField {
    /// 统一返回所有行。
    pub fn lines(&self) -> Vec<&str> {
        match self {
            ScriptField::Single(s) => vec![s.as_str()],
            ScriptField::Lines(v) => v.iter().map(String::as_str).collect(),
        }
    }

    /// 把所有行拼成一个脚本（用 `\n` 分隔）。
    pub fn joined(&self) -> String {
        match self {
            ScriptField::Single(s) => s.clone(),
            ScriptField::Lines(v) => v.join("\n"),
        }
    }
}

// ============================================================================
// License：string | { identifier, url? }
// ============================================================================

/// 许可证字段。
///
/// 1452 例为字符串（`"MIT"`、`"GPL-2.0-only"`），119 例为对象 `{ identifier, url }`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum License {
    Identifier(String),
    Detailed {
        identifier: String,
        url: Option<String>,
    },
}

impl Default for License {
    fn default() -> Self {
        License::Identifier(String::new())
    }
}

impl License {
    /// 返回 SPDX 标识符。
    pub fn identifier(&self) -> &str {
        match self {
            License::Identifier(s) => s,
            License::Detailed { identifier, .. } => identifier,
        }
    }
}

// ============================================================================
// HashField：string | string[]
// ============================================================================

/// 哈希字段（顶层 `hash` 或 `architecture.<arch>.hash`）。
///
/// 支持三种形式：
/// - `Single(String)`：单个 hash 字符串
/// - `Multiple(Vec<String>)`：hash 字符串数组（与 url 数组逐位对应）
/// - `Fetch`：从远程 URL 抓取 hash 的对象 `{url, regex?, jsonpath?, xpath?}`
///
/// 算法由字符串长度识别：40 → sha1，64 → sha256，128 → sha512；也可能带 `algo:` 前缀。
/// `Fetch` 变体参考 `AutoupdateHash::Fetch`，用于 autoupdate 中从 release notes 等远程源取哈希。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HashField {
    /// 抓取对象必须在 Plain 之前以优先匹配 object。
    /// 支持 `{url, regex?}`、`{url, jsonpath?}`、`{url, xpath?}`、`{jp, url}` 等所有 Scoop 取哈希形式。
    Fetch {
        url: String,
        regex: Option<String>,
        jsonpath: Option<String>,
        xpath: Option<String>,
    },
    Single(String),
    Multiple(Vec<String>),
}

impl HashField {
    /// 统一返回所有 hash 字符串。
    ///
    /// `Fetch` 变体无法静态给出 hash 值（需运行时抓取），返回空 Vec。
    pub fn values(&self) -> Vec<&str> {
        match self {
            HashField::Fetch { .. } => Vec::new(),
            HashField::Single(s) => vec![s.as_str()],
            HashField::Multiple(v) => v.iter().map(String::as_str).collect(),
        }
    }
}

// ============================================================================
// BinList / BinItem
// ============================================================================

/// `bin` 字段整体：接受单字符串（单 bin）或数组。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BinList(pub Vec<BinItem>);

impl BinList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// `bin` 数组中的单个条目。
///
/// - `Simple("bin/git.exe")`
/// - `Aliased { path: "python.exe", alias: "python3", args: None }`
/// - `Aliased { path: "idle.bat", alias: "idle3", args: Some("--flag x") }`（tuple[3+]）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinItem {
    Simple(String),
    Aliased {
        path: String,
        alias: String,
        /// 当 JSON 数组长度 ≥ 4 时，`tuple[3..]` 用空格拼接。
        args: Option<String>,
    },
}

impl BinItem {
    /// 返回目标可执行文件路径。
    pub fn path(&self) -> &str {
        match self {
            BinItem::Simple(p) => p,
            BinItem::Aliased { path, .. } => path,
        }
    }

    /// 返回对外暴露的命令名（无别名时与 `path` 相同）。
    pub fn alias_or_default(&self) -> String {
        match self {
            BinItem::Simple(p) => stem(p),
            BinItem::Aliased { alias, .. } => alias.clone(),
        }
    }
}

/// 从路径提取不含扩展名的文件名（用作默认 shim 名）。
fn stem(path: &str) -> String {
    let p = path.replace('\\', "/");
    let name = p.rsplit('/').next().unwrap_or(&p);
    name.split('.').next().unwrap_or(name).to_string()
}

impl Serialize for BinList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        if self.0.len() == 1 {
            match &self.0[0] {
                BinItem::Simple(p) => return serializer.serialize_str(p),
                BinItem::Aliased { path, alias, args } => {
                    let mut seq = serializer.serialize_seq(None)?;
                    seq.serialize_element(path)?;
                    seq.serialize_element(alias)?;
                    if let Some(a) = args {
                        seq.serialize_element(a)?;
                    }
                    return seq.end();
                }
            }
        }
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for item in &self.0 {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BinList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{Error, SeqAccess, Visitor};

        struct BinListVisitor;

        impl<'de> Visitor<'de> for BinListVisitor {
            type Value = BinList;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or an array of bin items")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<BinList, E> {
                Ok(BinList(vec![BinItem::Simple(v.to_string())]))
            }

            fn visit_string<E: Error>(self, v: String) -> Result<BinList, E> {
                Ok(BinList(vec![BinItem::Simple(v)]))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<BinList, A::Error> {
                let mut items = Vec::new();
                while let Some(item) = seq.next_element::<BinItem>()? {
                    items.push(item);
                }
                Ok(BinList(items))
            }
        }

        deserializer.deserialize_any(BinListVisitor)
    }
}

impl Serialize for BinItem {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        match self {
            BinItem::Simple(p) => serializer.serialize_str(p),
            BinItem::Aliased { path, alias, args } => {
                let len = if args.is_some() { 3 } else { 2 };
                let mut seq = serializer.serialize_seq(Some(len))?;
                seq.serialize_element(path)?;
                seq.serialize_element(alias)?;
                if let Some(a) = args {
                    seq.serialize_element(a)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for BinItem {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{Error, SeqAccess, Visitor};

        struct BinItemVisitor;

        impl<'de> Visitor<'de> for BinItemVisitor {
            type Value = BinItem;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or an array of 2-5 strings")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<BinItem, E> {
                Ok(BinItem::Simple(v.to_string()))
            }

            fn visit_string<E: Error>(self, v: String) -> Result<BinItem, E> {
                Ok(BinItem::Simple(v))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<BinItem, A::Error> {
                let mut parts: Vec<String> = Vec::new();
                while let Some(s) = seq.next_element::<String>()? {
                    parts.push(s);
                }
                match parts.len() {
                    0 => Err(A::Error::custom("bin tuple is empty")),
                    1 => Ok(BinItem::Simple(parts.pop().unwrap())),
                    2 => {
                        let mut it = parts.into_iter();
                        Ok(BinItem::Aliased {
                            path: it.next().unwrap(),
                            alias: it.next().unwrap(),
                            args: None,
                        })
                    }
                    _ => {
                        let path = parts.remove(0);
                        let alias = parts.remove(0);
                        let args = Some(parts.join(" "));
                        Ok(BinItem::Aliased { path, alias, args })
                    }
                }
            }
        }

        deserializer.deserialize_any(BinItemVisitor)
    }
}

// ============================================================================
// PersistList / PersistItem
// ============================================================================

/// `persist` 字段整体：接受单字符串或数组。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersistList(pub Vec<PersistItem>);

impl PersistList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// `persist` 数组中的单个条目。
///
/// - `Simple("etc")` → 持久化到 `$persist_dir/etc`，路径同名映射。
/// - `Renamed { src: "data", dst: "data" }` → 显式指定 src/dst。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistItem {
    Simple(String),
    Renamed { src: String, dst: String },
}

impl PersistItem {
    /// 规范化为 (source, target) 路径对。
    ///
    /// `Simple("etc")` → `("etc", "etc")`（同名映射）
    /// `Renamed { src: "data", dst: "backup" }` → `("data", "backup")`
    ///
    /// 对应 Scoop PS `persist_def`（`ref/Scoop/lib/install.ps1:429-443`）。
    pub fn source_and_target(&self) -> (&str, &str) {
        match self {
            PersistItem::Simple(p) => (p.as_str(), p.as_str()),
            PersistItem::Renamed { src, dst } => (src.as_str(), dst.as_str()),
        }
    }
}

impl Serialize for PersistList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        if self.0.len() == 1
            && let PersistItem::Simple(p) = &self.0[0]
        {
            return serializer.serialize_str(p);
        }
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for item in &self.0 {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for PersistList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{Error, SeqAccess, Visitor};

        struct PersistListVisitor;

        impl<'de> Visitor<'de> for PersistListVisitor {
            type Value = PersistList;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or an array of persist items")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<PersistList, E> {
                Ok(PersistList(vec![PersistItem::Simple(v.to_string())]))
            }

            fn visit_string<E: Error>(self, v: String) -> Result<PersistList, E> {
                Ok(PersistList(vec![PersistItem::Simple(v)]))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<PersistList, A::Error> {
                let mut items = Vec::new();
                while let Some(item) = seq.next_element::<PersistItem>()? {
                    items.push(item);
                }
                Ok(PersistList(items))
            }
        }

        deserializer.deserialize_any(PersistListVisitor)
    }
}

impl Serialize for PersistItem {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        match self {
            PersistItem::Simple(p) => serializer.serialize_str(p),
            PersistItem::Renamed { src, dst } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(src)?;
                seq.serialize_element(dst)?;
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for PersistItem {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{Error, SeqAccess, Visitor};

        struct PersistItemVisitor;

        impl<'de> Visitor<'de> for PersistItemVisitor {
            type Value = PersistItem;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or a [src, dst] tuple")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<PersistItem, E> {
                Ok(PersistItem::Simple(v.to_string()))
            }

            fn visit_string<E: Error>(self, v: String) -> Result<PersistItem, E> {
                Ok(PersistItem::Simple(v))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<PersistItem, A::Error> {
                let src: String = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("persist tuple missing src"))?;
                let dst: String = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("persist tuple missing dst"))?;
                Ok(PersistItem::Renamed { src, dst })
            }
        }

        deserializer.deserialize_any(PersistItemVisitor)
    }
}

// ============================================================================
// ShortcutItem：tuple[2..=4]
// ============================================================================

/// `shortcuts` 数组中的单个条目：`[target, name, args?, icon?]`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutItem {
    pub target: String,
    pub name: String,
    pub args: Option<String>,
    pub icon: Option<String>,
}

impl Serialize for ShortcutItem {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let len = 2 + usize::from(self.args.is_some()) + usize::from(self.icon.is_some());
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element(&self.target)?;
        seq.serialize_element(&self.name)?;
        if let Some(a) = &self.args {
            seq.serialize_element(a)?;
        }
        if let Some(i) = &self.icon {
            seq.serialize_element(i)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ShortcutItem {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{Error, SeqAccess, Visitor};

        struct ShortcutVisitor;

        impl<'de> Visitor<'de> for ShortcutVisitor {
            type Value = ShortcutItem;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a shortcut tuple of 2-4 strings [target, name, args?, icon?]")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<ShortcutItem, A::Error> {
                let target: String = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("shortcut missing target"))?;
                let name: String = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("shortcut missing name"))?;
                let args: Option<String> = seq.next_element()?;
                let icon: Option<String> = seq.next_element()?;
                Ok(ShortcutItem {
                    target,
                    name,
                    args,
                    icon,
                })
            }
        }

        deserializer.deserialize_seq(ShortcutVisitor)
    }
}

// ============================================================================
// Architecture / ArchSpec
// ============================================================================

/// 顶层 `architecture` 字段：按 CPU 架构拆分下载/安装参数。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Architecture {
    #[serde(rename = "64bit")]
    pub x86_64: Option<ArchSpec>,
    #[serde(rename = "32bit")]
    pub x86: Option<ArchSpec>,
    pub arm64: Option<ArchSpec>,
}

/// 单一架构分支的字段集合（与顶层部分重叠）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArchSpec {
    pub url: Option<OneOrMany<String>>,
    pub hash: Option<HashField>,
    pub extract_dir: Option<OneOrMany<String>>,
    pub extract_to: Option<OneOrMany<String>>,
    pub bin: Option<BinList>,
    pub shortcuts: Option<Vec<ShortcutItem>>,
    pub env_add_path: Option<OneOrMany<String>>,
    pub env_set: Option<BTreeMap<String, String>>,
    pub pre_install: Option<ScriptField>,
    pub post_install: Option<ScriptField>,
    pub installer: Option<InstallerSpec>,
    pub uninstaller: Option<InstallerSpec>,
}

// ============================================================================
// InstallerSpec
// ============================================================================

/// `installer` / `uninstaller` 字段。
///
/// - `script`：PowerShell 脚本。
/// - `file` + `args`：调用外部安装程序。
/// - `keep`：安装后保留安装包。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InstallerSpec {
    pub script: Option<ScriptField>,
    pub file: Option<String>,
    pub args: Option<OneOrMany<String>>,
    pub keep: Option<bool>,
}

/// 生命周期钩子类型（对应 Scoop PS `Invoke-HookScript` ValidateSet）。
///
/// 安装顺序：PreInstall → Installer → PostInstall
/// 卸载顺序：PreUninstall → Uninstaller → PostUninstall
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    PreInstall,
    Installer,
    PostInstall,
    PreUninstall,
    Uninstaller,
    PostUninstall,
}

// ============================================================================
// Checkver / Autoupdate
// ============================================================================

/// `checkver` 字段：可以是 github shorthand 字符串或完整对象。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckverField {
    /// 罕见 shorthand（16 例），通常当作 github 仓库或脚本标记。
    Short(String),
    Full(Box<Checkver>),
}

/// `checkver` 完整对象。字段组互斥（`github` / `url` / `script` / `sourceforge`），
/// 但使用扁平 `Option` 以兼容脏数据。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Checkver {
    pub github: Option<String>,
    pub url: Option<String>,
    pub regex: Option<String>,
    pub jsonpath: Option<String>,
    pub xpath: Option<String>,
    pub replace: Option<String>,
    /// `script` 可以是单字符串或字符串数组（参考 feem.json）
    pub script: Option<ScriptField>,
    pub reverse: Option<bool>,
    pub useragent: Option<String>,
    pub sourceforge: Option<String>,
    /// `re` — `regex` 的别名（仅 1 例 manifest 使用）。
    pub re: Option<String>,
}

/// `autoupdate` 字段。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Autoupdate {
    pub architecture: Option<Architecture>,
    pub url: Option<OneOrMany<String>>,
    pub hash: Option<AutoupdateHash>,
    pub extract_dir: Option<OneOrMany<String>>,
    pub bin: Option<BinList>,
}

/// `autoupdate.hash`：普通 hash 或抓取对象 `{ url, regex }`。
///
/// 抓取对象用于从指定 URL 用正则提取新版本对应的 hash（如 release notes 中的 sha256）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AutoupdateHash {
    /// 抓取对象（`{ url, regex? }`），必须在 Plain 之前以优先匹配 object。
    Fetch {
        url: String,
        regex: Option<String>,
        jsonpath: Option<String>,
        xpath: Option<String>,
    },
    Plain(HashField),
}

// ============================================================================
// PowerShellModule
// ============================================================================

/// `psmodule` 字段：PowerShell 模块声明。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PowerShellModule {
    pub name: String,
    pub manifest: Option<Vec<String>>,
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_or_many_into_vec() {
        let one: OneOrMany<String> = OneOrMany::One("a".into());
        assert_eq!(one.into_vec(), vec!["a".to_string()]);

        let many: OneOrMany<String> = OneOrMany::Many(vec!["a".into(), "b".into()]);
        assert_eq!(many.into_vec(), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn script_field_lines() {
        let s = ScriptField::Single("echo hi".into());
        assert_eq!(s.lines(), vec!["echo hi"]);

        let l = ScriptField::Lines(vec!["a".into(), "b".into()]);
        assert_eq!(l.joined(), "a\nb");
    }

    #[test]
    fn license_identifier() {
        let l = License::Identifier("MIT".into());
        assert_eq!(l.identifier(), "MIT");

        let l2 = License::Detailed {
            identifier: "GPL-2.0".into(),
            url: None,
        };
        assert_eq!(l2.identifier(), "GPL-2.0");
    }

    #[test]
    fn bin_item_stem() {
        assert_eq!(stem("bin\\git.exe"), "git");
        assert_eq!(stem("python.exe"), "python");
        assert_eq!(stem("foo"), "foo");
    }

    #[test]
    fn persist_item_source_and_target_simple() {
        let p = PersistItem::Simple("etc".into());
        assert_eq!(p.source_and_target(), ("etc", "etc"));
    }

    #[test]
    fn persist_item_source_and_target_renamed() {
        let p = PersistItem::Renamed {
            src: "data".into(),
            dst: "backup".into(),
        };
        assert_eq!(p.source_and_target(), ("data", "backup"));
    }

    #[test]
    fn manifest_depends_list_none() {
        let m = Manifest::default();
        assert!(m.depends_list().is_empty());
    }

    #[test]
    fn manifest_depends_list_single() {
        let m = Manifest {
            depends: Some(OneOrMany::One("perl".into())),
            ..Default::default()
        };
        assert_eq!(m.depends_list(), vec!["perl"]);
    }

    #[test]
    fn manifest_depends_list_many() {
        let m = Manifest {
            depends: Some(OneOrMany::Many(vec!["a".into(), "b/c".into()])),
            ..Default::default()
        };
        assert_eq!(m.depends_list(), vec!["a", "b/c"]);
    }

    #[test]
    fn hook_type_variants() {
        let hooks = [
            HookType::PreInstall,
            HookType::Installer,
            HookType::PostInstall,
            HookType::PreUninstall,
            HookType::Uninstaller,
            HookType::PostUninstall,
        ];
        assert_eq!(hooks.len(), 6);
        assert_ne!(hooks[0], hooks[1]);
    }
}
