//! Manifest 解析器。
//!
//! 提供：
//! - `parse_str`：JSON 反序列化
//! - `FlatManifest`：合并架构分支后的扁平视图
//! - `parse_and_resolve`：解析 + 架构合并一步完成
//!
//! 架构合并规则参考 Scoop `arch_specific`（`ref/Scoop/lib/manifest.ps1:148-155`）：
//! 对每个"可架构化"字段，取 `architecture.<arch>.<field>`；若为 `None` 则回退到顶层。
//! 数组/Map 类字段为整体替换（不合并）。

use crate::manifest::schema::{ArchSpec, Architecture, HookType, Manifest, ScriptField};
use crate::manifest::variables::Arch;
use hit_common::error::Result;
use std::collections::BTreeMap;

/// 从字符串解析 Manifest。
///
/// 失败时使用 `HitError::Other(anyhow)` 兜底，安装流水线调用点会再用
/// `HitError::Manifest { app, message }` 包裹以提供应用名上下文。
pub fn parse_str(input: &str) -> Result<Manifest> {
    sonic_rs::from_str::<Manifest>(input)
        .map_err(|e| anyhow::anyhow!("manifest JSON 解析失败：{e}").into())
}

/// 架构合并后的扁平 Manifest 视图。
///
/// 合并规则：对每个"可架构化"字段（`url` / `hash` / `extract_dir` / `extract_to` /
/// `bin` / `shortcuts` / `env_add_path` / `env_set` / `pre_install` / `post_install` /
/// `installer` / `uninstaller`），取 `architecture.<arch>.<field>`；若为 `None` 则保留顶层值。
/// 合并后 `architecture` 字段被清空（`None`）。
///
/// `description` / `homepage` / `license` / `notes` / `checkver` / `autoupdate` /
/// `version` / `depends` / `suggest` / `persist` 不受架构影响。
#[derive(Debug, Clone)]
pub struct FlatManifest(pub Manifest);

impl FlatManifest {
    /// 合并指定架构分支到顶层，返回扁平视图。
    pub fn resolve_architecture(mut m: Manifest, arch: Arch) -> Self {
        let spec = m
            .architecture
            .as_ref()
            .and_then(|a| pick_arch_spec(a, arch));
        if let Some(spec) = spec.cloned() {
            merge_arch_spec_into_top(&mut m, spec);
        }
        m.architecture = None;
        FlatManifest(m)
    }

    /// 解包为内部 Manifest。
    pub fn into_inner(self) -> Manifest {
        self.0
    }

    /// 借用内部 Manifest。
    pub fn inner(&self) -> &Manifest {
        &self.0
    }

    /// 获取架构合并后指定钩子类型的有效脚本。
    ///
    /// - `PreInstall` / `PostInstall` / `PreUninstall` / `PostUninstall`：
    ///   返回对应的 `ScriptField` 引用。
    /// - `Installer` / `Uninstaller`：
    ///   返回 `InstallerSpec.script` 字段。
    ///
    /// 对应 Scoop PS `Invoke-HookScript` 先调 `arch_specific` 获取脚本再执行
    /// （`ref/Scoop/lib/install.ps1:147-172`）。
    pub fn resolve_script(&self, hook: HookType) -> Option<&ScriptField> {
        let m = &self.0;
        match hook {
            HookType::PreInstall => m.pre_install.as_ref(),
            HookType::PostInstall => m.post_install.as_ref(),
            HookType::PreUninstall => m.pre_uninstall.as_ref(),
            HookType::PostUninstall => m.post_uninstall.as_ref(),
            HookType::Installer => m.installer.as_ref()?.script.as_ref(),
            HookType::Uninstaller => m.uninstaller.as_ref()?.script.as_ref(),
        }
    }
}

/// 便捷：解析 + 架构合并一步完成。
pub fn parse_and_resolve(input: &str, arch: Arch) -> Result<FlatManifest> {
    let m = parse_str(input)?;
    Ok(FlatManifest::resolve_architecture(m, arch))
}

// ============================================================================
// 架构合并实现
// ============================================================================

/// 从 Architecture 选取指定分支的 ArchSpec（若存在）。
fn pick_arch_spec(a: &Architecture, arch: Arch) -> Option<&ArchSpec> {
    match arch {
        Arch::X86_64 => a.x86_64.as_ref(),
        Arch::X86 => a.x86.as_ref(),
        Arch::Arm64 => a.arm64.as_ref(),
    }
}

/// 合并规则：若 src 为 Some，则覆盖 dst；否则保留 dst。
fn override_opt<T>(dst: &mut Option<T>, src: Option<T>) {
    if src.is_some() {
        *dst = src;
    }
}

/// 将架构分支字段合并到顶层 Manifest。
fn merge_arch_spec_into_top(m: &mut Manifest, spec: ArchSpec) {
    override_opt(&mut m.url, spec.url);
    override_opt(&mut m.hash, spec.hash);
    override_opt(&mut m.extract_dir, spec.extract_dir);
    override_opt(&mut m.extract_to, spec.extract_to);
    override_opt(&mut m.bin, spec.bin);
    override_opt(&mut m.shortcuts, spec.shortcuts);
    override_opt(&mut m.env_add_path, spec.env_add_path);
    override_opt(&mut m.pre_install, spec.pre_install);
    override_opt(&mut m.post_install, spec.post_install);
    override_opt(&mut m.installer, spec.installer);
    override_opt(&mut m.uninstaller, spec.uninstaller);

    // env_set：整体替换（与 Scoop 一致）
    if spec.env_set.is_some() {
        m.env_set = spec.env_set;
    }
}

/// 列出 Manifest 支持的架构分支（用于 install 前校验目标平台）。
pub fn supported_architectures(m: &Manifest) -> Vec<Arch> {
    match &m.architecture {
        None => {
            // 无 architecture 字段：视为通用（默认 x86_64 + x86 + arm64）
            vec![Arch::X86_64, Arch::X86, Arch::Arm64]
        }
        Some(a) => {
            let mut v = Vec::new();
            if a.x86_64.is_some() {
                v.push(Arch::X86_64);
            }
            if a.x86.is_some() {
                v.push(Arch::X86);
            }
            if a.arm64.is_some() {
                v.push(Arch::Arm64);
            }
            v
        }
    }
}

/// 检查 Manifest 是否支持指定架构。
pub fn supports_arch(m: &Manifest, arch: Arch) -> bool {
    supported_architectures(m).contains(&arch)
}

/// 提取 Manifest 中所有 `env_set` 键值对（顶层 + 所有架构分支）。
///
/// 用于 install 流水线注册环境变量前的预扫描。
pub fn collect_all_env_sets(m: &Manifest) -> Vec<BTreeMap<String, String>> {
    let mut out = Vec::new();
    if let Some(map) = &m.env_set {
        out.push(map.clone());
    }
    if let Some(a) = &m.architecture {
        if let Some(s) = &a.x86_64
            && let Some(map) = &s.env_set
        {
            out.push(map.clone());
        }
        if let Some(s) = &a.x86
            && let Some(map) = &s.env_set
        {
            out.push(map.clone());
        }
        if let Some(s) = &a.arm64
            && let Some(map) = &s.env_set
        {
            out.push(map.clone());
        }
    }
    out
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::schema::OneOrMany;

    #[test]
    fn parse_str_valid_json() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://example.com",
            "license": "MIT"
        }"#;
        let m = parse_str(body).unwrap();
        assert_eq!(m.version, "1.0");
    }

    #[test]
    fn parse_str_invalid_json() {
        assert!(parse_str("not json").is_err());
    }

    #[test]
    fn resolve_architecture_falls_back_to_top_level() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "url": "https://generic/file.zip",
            "hash": "abc123"
        }"#;
        let m = parse_str(body).unwrap();
        let flat = FlatManifest::resolve_architecture(m, Arch::X86_64);

        // 无 architecture 字段 → 顶层 url/hash 保留
        assert!(flat.0.architecture.is_none());
        match flat.0.url.as_ref().unwrap() {
            OneOrMany::One(s) => assert_eq!(s, "https://generic/file.zip"),
            _ => panic!("期望 OneOrMany::One"),
        }
    }

    #[test]
    fn resolve_architecture_overrides_with_branch() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "url": "https://generic/file.zip",
            "architecture": {
                "64bit": {
                    "url": "https://x64/file.zip",
                    "hash": "x64hash"
                }
            }
        }"#;
        let m = parse_str(body).unwrap();
        let flat = FlatManifest::resolve_architecture(m, Arch::X86_64);

        // 64bit 分支应覆盖顶层 url
        match flat.0.url.as_ref().unwrap() {
            OneOrMany::One(s) => assert_eq!(s, "https://x64/file.zip"),
            _ => panic!("期望 OneOrMany::One"),
        }
        match flat.0.hash.as_ref().unwrap() {
            crate::manifest::schema::HashField::Single(s) => assert_eq!(s, "x64hash"),
            _ => panic!("期望 HashField::Single"),
        }
    }

    #[test]
    fn resolve_architecture_fallback_when_branch_missing() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "url": "https://generic/file.zip",
            "architecture": {
                "64bit": { "url": "https://x64/file.zip" }
            }
        }"#;
        let m = parse_str(body).unwrap();
        // 请求 arm64 分支，但 manifest 只有 64bit → 回退到顶层 url
        let flat = FlatManifest::resolve_architecture(m, Arch::Arm64);
        match flat.0.url.as_ref().unwrap() {
            OneOrMany::One(s) => assert_eq!(s, "https://generic/file.zip"),
            _ => panic!("期望 OneOrMany::One"),
        }
    }

    #[test]
    fn supported_architectures_all_when_no_arch_block() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT"
        }"#;
        let m = parse_str(body).unwrap();
        let archs = supported_architectures(&m);
        assert_eq!(archs.len(), 3);
    }

    #[test]
    fn supported_architectures_lists_declared_only() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "architecture": {
                "64bit": { "url": "https://x64/a.zip" },
                "arm64": { "url": "https://arm/a.zip" }
            }
        }"#;
        let m = parse_str(body).unwrap();
        let archs = supported_architectures(&m);
        assert_eq!(archs, vec![Arch::X86_64, Arch::Arm64]);
        assert!(supports_arch(&m, Arch::X86_64));
        assert!(!supports_arch(&m, Arch::X86));
    }

    #[test]
    fn parse_and_resolve_one_step() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "url": "https://generic/file.zip",
            "architecture": {
                "64bit": { "url": "https://x64/file.zip" }
            }
        }"#;
        let flat = parse_and_resolve(body, Arch::X86_64).unwrap();
        match flat.0.url.as_ref().unwrap() {
            OneOrMany::One(s) => assert_eq!(s, "https://x64/file.zip"),
            _ => panic!("期望 OneOrMany::One"),
        }
        assert!(flat.0.architecture.is_none());
    }

    #[test]
    fn env_set_overrides_whole_map() {
        // 架构分支的 env_set 应整体替换（不合并）顶层
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "env_set": { "A": "1", "B": "2" },
            "architecture": {
                "64bit": { "env_set": { "C": "3" } }
            }
        }"#;
        let m = parse_str(body).unwrap();
        let flat = FlatManifest::resolve_architecture(m, Arch::X86_64);
        let env = flat.0.env_set.as_ref().unwrap();
        // 顶层 A/B 应被 64bit 的 C 完全覆盖
        assert!(env.get("A").is_none());
        assert_eq!(env.get("C").map(String::as_str), Some("3"));
    }

    #[test]
    fn collect_all_env_sets_includes_all_branches() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "env_set": { "TOP": "1" },
            "architecture": {
                "64bit": { "env_set": { "X64": "1" } },
                "arm64": { "env_set": { "ARM": "1" } }
            }
        }"#;
        let m = parse_str(body).unwrap();
        let sets = collect_all_env_sets(&m);
        assert_eq!(sets.len(), 3);
    }

    #[test]
    fn resolve_script_pre_install() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "pre_install": "Write-Host 'hi'"
        }"#;
        let flat = parse_and_resolve(body, Arch::X86_64).unwrap();
        use crate::manifest::schema::ScriptField;
        assert!(matches!(
            flat.resolve_script(HookType::PreInstall),
            Some(ScriptField::Single(_))
        ));
    }

    #[test]
    fn resolve_script_installer() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT",
            "installer": { "script": "Expand-7zipArchive" }
        }"#;
        let flat = parse_and_resolve(body, Arch::X86_64).unwrap();
        assert!(flat.resolve_script(HookType::Installer).is_some());
    }

    #[test]
    fn resolve_script_missing_returns_none() {
        let body = r#"{
            "version": "1.0",
            "description": "test",
            "homepage": "https://x",
            "license": "MIT"
        }"#;
        let flat = parse_and_resolve(body, Arch::X86_64).unwrap();
        assert!(flat.resolve_script(HookType::PreInstall).is_none());
        assert!(flat.resolve_script(HookType::PostInstall).is_none());
        assert!(flat.resolve_script(HookType::Installer).is_none());
        assert!(flat.resolve_script(HookType::Uninstaller).is_none());
    }
}
