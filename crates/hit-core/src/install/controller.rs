//! 安装 / 卸载流水线控制器
//!
//! 编排从 manifest 解析到 shim 创建、persist 链接、current junction 的完整安装流水线。
//! 所有步骤包裹在 RAII `Transaction` 中，任意步骤失败自动回滚。
//!
//! 参考 Scoop `install_app`（`ref/Scoop/lib/install.ps1:1-100`）的 22 步流水线，
//! 合并/简化为 11 步。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use hit_common::event::{Event, InstallPhase};
use hit_common::{HitError, Result, Session};

use crate::compress;
use crate::download::cache::download_to_cache;
use crate::hash::verify_file_hash;
use crate::install::dependency::resolve_dependencies;
use crate::install::persist::link_persist;
use crate::install::shim::{create_shim, remove_app_shims};
use crate::install::transaction::{Transaction, UndoAction};
use crate::manifest::{BinItem, HookType};
use crate::manifest::variables::{Arch, InstallVars, IntoVarMap, substitute};
use crate::manifest::{FlatManifest, Manifest};
use crate::store::{Db, InstalledPackage, db_path};
use crate::win::env::{add_to_path, ensure_shims_in_path, remove_from_path, set_env_var};
use crate::win::fs::link_current;
use crate::win::process::find_running_processes;

/// 安装选项
#[derive(Debug)]
pub struct InstallOptions {
    /// 强制重装（即使已安装）
    pub force: bool,
    /// 指定架构（默认跟随系统）
    pub arch: Option<Arch>,
    /// 跳过依赖检查
    pub no_deps: bool,
    /// 全局安装（当前仅影响 PATH 注册位置）
    pub global: bool,
    /// 中断标志（由调用方设置，如 Ctrl+C）
    pub should_interrupt: AtomicBool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            force: false,
            arch: None,
            no_deps: false,
            global: false,
            should_interrupt: AtomicBool::new(false),
        }
    }
}

/// 安装结果
#[derive(Debug)]
pub struct InstallResult {
    pub app: String,
    pub version: String,
    pub install_dir: PathBuf,
    pub shims_created: Vec<String>,
    pub deps_installed: Vec<String>,
}

/// 安装单个 app（完整流水线）
pub fn install(
    session: &Session,
    app: &str,
    manifest: &Manifest,
    bucket: &str,
    options: &InstallOptions,
) -> Result<InstallResult> {
    let arch = options.arch.unwrap_or_else(|| Arch::current().unwrap_or(Arch::X86_64));

    // 已安装检测
    let current_path = session.apps_path().join(app).join("current");
    if current_path.exists() && !options.force {
        return Err(HitError::Install {
            app: app.to_string(),
            message: format!("'{app}' 已安装，使用 --force 强制重装"),
        });
    }

    // Step 1: 解析 manifest
    emit_phase(session, app, InstallPhase::Resolve, true);
    let flat = FlatManifest::resolve_architecture(manifest.clone(), arch);
    let version = flat.inner().version.clone();
    emit_phase(session, app, InstallPhase::Resolve, false);

    // Step 2: 解析依赖
    let deps = if options.no_deps {
        Vec::new()
    } else {
        resolve_dependencies(session, app, manifest)?
    };

    // 递归安装依赖
    let mut deps_installed = Vec::new();
    for dep in &deps {
        let dep_options = InstallOptions {
            force: false,
            arch: options.arch,
            no_deps: false,
            global: options.global,
            should_interrupt: AtomicBool::new(false),
        };
        install(session, &dep.name, &dep.manifest, &dep.bucket, &dep_options)?;
        deps_installed.push(dep.name.clone());
    }

    // Step 3: 下载
    emit_phase(session, app, InstallPhase::Download, true);
    let urls = flat
        .inner()
        .url
        .as_ref()
        .map(|u| u.as_slice().to_vec())
        .unwrap_or_default();
    let mut cache_files = Vec::new();
    for url in &urls {
        let path = download_to_cache(session, app, &version, url, &options.should_interrupt)?;
        cache_files.push(path);
    }
    emit_phase(session, app, InstallPhase::Download, false);

    // Step 4: 校验哈希
    emit_phase(session, app, InstallPhase::HashVerify, true);
    if let Some(hash_field) = &flat.inner().hash {
        let expected_hashes = hash_field.values();
        for (i, file) in cache_files.iter().enumerate() {
            if let Some(expected) = expected_hashes.get(i) {
                verify_file_hash(file, expected)?;
            }
        }
    }
    emit_phase(session, app, InstallPhase::HashVerify, false);

    // 从事务开始，后续步骤失败自动回滚
    let mut tx = Transaction::begin(app)?;

    // Step 5: 解压
    let version_dir = session.apps_path().join(app).join(&version);
    let extract_dirs: Vec<Option<String>> = flat
        .inner()
        .extract_dir
        .as_ref()
        .map(|d| d.as_slice().iter().map(|s| Some(s.clone())).collect())
        .unwrap_or_else(|| vec![None; cache_files.len()]);

    std::fs::create_dir_all(&version_dir).map_err(|e| {
        HitError::io(
            format!("创建版本目录失败：{}", version_dir.display()),
            e,
        )
    })?;

    for (i, archive) in cache_files.iter().enumerate() {
        let extract_dir = extract_dirs.get(i).and_then(|d| d.as_deref());
        let url = urls.get(i).map(|s| s.as_str());
        let innosetup = flat.inner().innosetup.unwrap_or(false);
        compress::decompress(session, app, archive, &version_dir, extract_dir, url, innosetup)?;
    }

    // 构建变量上下文
    let persist_dir = session.persist_path().join(app);
    let vars = InstallVars {
        version: version.clone(),
        dir: version_dir.clone(),
        persist_dir,
        architecture: arch,
        global: options.global,
        app: app.to_string(),
        original_dir: None,
    };
    let var_map = vars.to_var_map();

    // Step 5.5: 执行 pre_install 脚本
    run_hook_script(session, &flat, HookType::PreInstall, &version_dir, &var_map, bucket)?;

    // Step 6: 创建 shim
    let shims_created = step_create_shims(session, app, &version_dir, &flat, &var_map, &mut tx)?;

    // Step 7: persist 链接
    emit_phase(session, app, InstallPhase::Sync, true);
    if let Some(persist_list) = &flat.inner().persist
        && !persist_list.is_empty()
    {
        link_persist(session, app, &version_dir, persist_list, &mut tx)?;
    }

    // Step 8: 环境变量
    step_setup_env(session, &flat, &version_dir, &var_map, &mut tx)?;

    // Step 9: 创建 current junction
    emit_phase(session, app, InstallPhase::Commit, true);
    let current_dir = link_current(&version_dir, false)?;
    tx.record_undo(UndoAction::RemoveJunction(current_dir.clone()));
    emit_phase(session, app, InstallPhase::Commit, false);

    // Step 10: 执行 post_install 脚本
    run_hook_script(session, &flat, HookType::PostInstall, &version_dir, &var_map, bucket)?;

    // Step 11: 保存安装信息到 db.json
    {
        let mut db = Db::load(&db_path(session))?;

        let env_add_path: Vec<String> = flat
            .inner()
            .env_add_path
            .as_ref()
            .map(|paths| {
                paths
                    .as_slice()
                    .iter()
                    .map(|p| {
                        let substituted = substitute(p, &var_map);
                        if Path::new(&substituted).is_absolute() {
                            substituted
                        } else {
                            version_dir.join(&substituted).display().to_string()
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let env_set: BTreeMap<String, String> = flat
            .inner()
            .env_set
            .as_ref()
            .map(|m| {
                m.iter()
                    .map(|(k, v)| (k.clone(), substitute(v, &var_map)))
                    .collect()
            })
            .unwrap_or_default();

        let persist_files: Vec<String> = flat
            .inner()
            .persist
            .as_ref()
            .map(|pl| pl.0.iter().map(|item| item.source_and_target().0.to_string()).collect())
            .unwrap_or_default();

        let raw_manifest = sonic_rs::to_string(manifest).unwrap_or_default();

        let pkg = InstalledPackage {
            version: version.clone(),
            bucket: bucket.to_string(),
            install_date: crate::store::models::now_iso8601(),
            architecture: arch.scoop_key().to_string(),
            shims: shims_created.clone(),
            persist_files,
            held: false,
            env_add_path,
            env_set,
            raw_manifest,
        };

        db.insert_package(app.to_string(), pkg);
        db.save()?;
    }

    tx.commit()?;
    emit_phase(session, app, InstallPhase::Sync, false);

    Ok(InstallResult {
        app: app.to_string(),
        version,
        install_dir: current_dir,
        shims_created,
        deps_installed,
    })
}

/// 卸载单个 app
pub fn uninstall(session: &Session, app: &str) -> Result<()> {
    let app_dir = session.apps_path().join(app);
    let current_dir = app_dir.join("current");

    if !app_dir.exists() {
        return Err(HitError::Install {
            app: app.to_string(),
            message: format!("'{app}' 未安装"),
        });
    }

    // 检测运行中的进程
    let running = find_running_processes(&app_dir)?;
    if !running.is_empty() {
        let names: Vec<_> = running.iter().map(|p| format!("{}(PID {})", p.name, p.pid)).collect();
        return Err(HitError::Install {
            app: app.to_string(),
            message: format!("以下进程正在运行，请先关闭：{}", names.join(", ")),
        });
    }

    // 从 db.json 读取安装信息（manifest、环境变量等）
    let mut db = Db::load(&db_path(session))?;
    let install_info = db.get_package(app).cloned();

    // 尝试从存储的 raw_manifest 恢复 Manifest（用于执行卸载脚本）
    let stored_flat: Option<FlatManifest> = install_info.as_ref().and_then(|info| {
        if info.raw_manifest.is_empty() {
            return None;
        }
        let m: Manifest = sonic_rs::from_str(&info.raw_manifest).ok()?;
        let arch = Arch::from_scoop_key(&info.architecture).unwrap_or(Arch::X86_64);
        Some(FlatManifest::resolve_architecture(m, arch))
    });

    // 移除 current junction（非致命：失败仅 warn，不中断卸载）
    // 无论何种情况，最终都会尝试暴力删除 apps/<app>/ 整棵树
    let version_dir = if current_dir.exists() {
        let version = find_latest_version_dir(&app_dir).ok();
        if let Err(e) = crate::win::fs::remove_junction(&current_dir) {
            tracing::warn!(app, error = %e, "移除 junction 失败（继续卸载）");
        }
        // 安全兜底：如果 current 仍存在（可能是普通目录残留），清理掉
        if current_dir.exists() {
            std::fs::remove_dir(&current_dir).ok();
        }
        version.unwrap_or_else(|| app_dir.join("unknown"))
    } else {
        find_latest_version_dir(&app_dir).unwrap_or_else(|_| app_dir.join("unknown"))
    };

    // 移除 shim
    remove_app_shims(session, app)?;

    // 移除环境变量（非致命：失败仅 warn，不中断卸载）
    if let Some(ref info) = install_info {
        if !info.env_add_path.is_empty() {
            let patterns: Vec<&str> = info.env_add_path.iter().map(String::as_str).collect();
            if let Err(e) = remove_from_path(&patterns, "PATH") {
                tracing::warn!(app, error = %e, "移除 PATH 条目失败（继续卸载）");
            }
        }
        for key in info.env_set.keys() {
            if let Err(e) = set_env_var(key, None) {
                tracing::warn!(app, key, error = %e, "移除环境变量失败（继续卸载）");
            }
        }
    }

    // 执行 pre_uninstall 脚本
    if let Some(ref flat) = stored_flat
        && let Some(ref info) = install_info
    {
        let arch = Arch::from_scoop_key(&info.architecture).unwrap_or(Arch::X86_64);
        let vars = InstallVars {
            version: info.version.clone(),
            dir: version_dir.clone(),
            persist_dir: session.persist_path().join(app),
            architecture: arch,
            global: false,
            app: app.to_string(),
            original_dir: None,
        };
        let var_map = vars.to_var_map();
        run_hook_script(session, flat, HookType::PreUninstall, &version_dir, &var_map, &info.bucket).ok();
    }

    // 移除版本目录
    if version_dir.exists() {
        std::fs::remove_dir_all(&version_dir).map_err(|e| {
            HitError::io(
                format!("删除版本目录失败：{}", version_dir.display()),
                e,
            )
        })?;
    }

    // 暴力清理 apps/<app>/ 整棵目录树（包括 current 残留、空父目录等）
    if app_dir.exists() {
        std::fs::remove_dir_all(&app_dir).ok();
    }

    // persist 数据保留不删除

    // 从 db.json 移除安装记录
    db.remove_package(app);
    db.save()?;

    Ok(())
}

/// 切换 app 到指定版本（更新 current junction + persist 链接）
pub fn reset_version(session: &Session, app: &str, version: &str) -> Result<()> {
    let app_dir = session.apps_path().join(app);
    let new_version_dir = app_dir.join(version);

    if !new_version_dir.exists() {
        return Err(HitError::Install {
            app: app.to_string(),
            message: format!("版本 '{version}' 未安装"),
        });
    }

    // 移除旧 current junction（直接用 junction::delete，不依赖 read_link）
    let current_dir = app_dir.join("current");
    if current_dir.exists() || std::fs::symlink_metadata(&current_dir).is_ok() {
        junction::delete(&current_dir).ok();
        // fallback：junction::delete 可能失败，用 remove_dir 兜底
        if current_dir.exists() {
            std::fs::remove_dir(&current_dir).ok();
        }
    }

    // 创建新 current junction
    link_current(&new_version_dir, false)?;

    Ok(())
}

// ============================================================================
// 内部辅助函数
// ============================================================================

/// 发送 InstallPhase 事件
fn emit_phase(session: &Session, app: &str, phase: InstallPhase, start: bool) {
    if start {
        session.emit(Event::InstallPhaseStart {
            app: app.to_string(),
            phase,
        });
    } else {
        session.emit(Event::InstallPhaseEnd {
            app: app.to_string(),
            phase,
        });
    }
}

/// 创建所有 shim
fn step_create_shims(
    session: &Session,
    app: &str,
    version_dir: &Path,
    flat: &FlatManifest,
    var_map: &BTreeMap<String, String>,
    tx: &mut Transaction,
) -> Result<Vec<String>> {
    let mut shims_created = Vec::new();

    let bin_list = match &flat.inner().bin {
        Some(bl) if !bl.is_empty() => bl,
        _ => return Ok(shims_created),
    };

    for item in &bin_list.0 {
        let rel_path = substitute(item.path(), var_map);
        let target = version_dir.join(&rel_path);
        let name = substitute(&item.alias_or_default(), var_map);

        let args = match item {
            BinItem::Aliased { args: Some(a), .. } => {
                let substituted = substitute(a, var_map);
                substituted.split_whitespace().map(String::from).collect()
            }
            _ => Vec::new(),
        };

        create_shim(session, &name, &target, &args, None, tx)?;
        shims_created.push(name);
    }

    // 确保 shims 目录在 PATH 中
    ensure_shims_in_path(session.shims_path()).map_err(|e| {
        tracing::warn!(app, error = %e, "确保 shims 在 PATH 中失败（非致命）");
        e
    })?;

    Ok(shims_created)
}

/// 设置环境变量（env_add_path + env_set）
fn step_setup_env(
    _session: &Session,
    flat: &FlatManifest,
    version_dir: &Path,
    var_map: &BTreeMap<String, String>,
    tx: &mut Transaction,
) -> Result<()> {
    let m = flat.inner();

    // env_add_path
    if let Some(paths) = &m.env_add_path {
        let mut path_bufs: Vec<PathBuf> = Vec::new();
        for p in paths.as_slice() {
            let substituted = substitute(p, var_map);
            let abs = if Path::new(&substituted).is_absolute() {
                PathBuf::from(&substituted)
            } else {
                version_dir.join(&substituted)
            };
            path_bufs.push(abs);
        }
        if !path_bufs.is_empty() {
            let path_refs: Vec<&Path> = path_bufs.iter().map(|p| p.as_path()).collect();
            add_to_path(&path_refs, "PATH")?;

            let path_strs: Vec<String> = path_bufs.iter().map(|p| p.display().to_string()).collect();
            tx.record_undo(UndoAction::RemoveFromPath(path_strs));
        }
    }

    // env_set
    if let Some(env_map) = &m.env_set {
        for (key, value) in env_map {
            let substituted = substitute(value, var_map);
            set_env_var(key, Some(&substituted))?;
            tx.record_undo(UndoAction::RemoveEnvVar(key.clone()));
        }
    }

    Ok(())
}

/// 执行钩子脚本（pre_install / post_install / pre_uninstall / post_uninstall）
fn run_hook_script(
    session: &Session,
    flat: &FlatManifest,
    hook: HookType,
    version_dir: &Path,
    var_map: &BTreeMap<String, String>,
    bucket: &str,
) -> Result<()> {
    let script = match flat.resolve_script(hook) {
        Some(s) => s,
        None => return Ok(()),
    };

    let mut body = script.joined();
    for (k, v) in var_map {
        // $global 是 PowerShell 内置变量，不应替换到脚本 body 中
        // （preamble 已正确设置 $global=$false）
        if k != "$global" {
            body = body.replace(k.as_str(), v.as_str());
        }
    }

    // 定义 Scoop 兼容的 PowerShell 变量，使 post_install 脚本能引用 $dir、$version 等
    let app = var_map.get("$app").cloned().unwrap_or_default();
    let version = flat.inner().version.clone();
    let persist_dir = session.persist_path().join(&app);
    let buckets_dir = format!("{}", session.buckets_path().display());
    let scoop_dir = format!("{}", session.root_path().display());
    let dir_str = format!("{}", version_dir.display());

    let preamble = format!(
        "$dir='{}'; $version='{}'; $persist_dir='{}'; $bucketsdir='{}'; $scoopdir='{}'; $app='{}'; $bucket='{}'; $global=$false; ",
        dir_str.replace('\'', "''"),
        version.replace('\'', "''"),
        persist_dir.display().to_string().replace('\'', "''"),
        buckets_dir.replace('\'', "''"),
        scoop_dir.replace('\'', "''"),
        app.replace('\'', "''"),
        bucket.replace('\'', "''"),
    );
    let full_body = preamble + &body;

    let status = std::process::Command::new("pwsh")
        .args(["-NoProfile", "-Command", &full_body])
        .current_dir(version_dir)
        .status()
        .map_err(|e| {
            HitError::Install {
                app: var_map.get("$app").cloned().unwrap_or_default(),
                message: format!("执行 {hook:?} 脚本失败：{e}"),
            }
        })?;

    if !status.success() {
        return Err(HitError::Install {
            app: var_map.get("$app").cloned().unwrap_or_default(),
            message: format!(
                "{hook:?} 脚本退出码：{}",
                status.code().unwrap_or(-1)
            ),
        });
    }

    Ok(())
}

/// 查找最新的版本目录（排除 current junction）
fn find_latest_version_dir(app_dir: &Path) -> Result<PathBuf> {
    let mut dirs: Vec<_> = std::fs::read_dir(app_dir)
        .map_err(|e| HitError::io("读取 app 目录失败", e))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name != "current" && e.path().is_dir()
        })
        .collect();

    dirs.sort_by_key(|b| std::cmp::Reverse(b.file_name()));

    dirs.into_iter()
        .next()
        .map(|e| e.path())
        .ok_or_else(|| HitError::Install {
            app: app_dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            message: "找不到版本目录".into(),
        })
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::License;
    use hit_common::config::HitConfig;

    fn test_session(dir: &Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    fn minimal_manifest(version: &str) -> Manifest {
        Manifest {
            version: version.to_string(),
            description: "test app".into(),
            homepage: "https://example.com".into(),
            license: License::Identifier("MIT".into()),
            ..Default::default()
        }
    }

    #[test]
    fn install_already_installed_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        // 模拟已安装
        let current = dir.path().join("apps").join("myapp").join("current");
        std::fs::create_dir_all(&current).unwrap();

        let manifest = minimal_manifest("1.0");
        let options = InstallOptions::default();
        let result = install(&session, "myapp", &manifest, "main", &options);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("已安装"), "错误信息应包含'已安装'：{err}");
    }

    #[test]
    fn install_already_installed_force_succeeds() {
        // force 模式不因已安装报错（但后续步骤可能因缺少 url 等而失败）
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        let current = dir.path().join("apps").join("myapp").join("current");
        std::fs::create_dir_all(&current).unwrap();

        let manifest = minimal_manifest("1.0");
        let options = InstallOptions {
            force: true,
            ..Default::default()
        };
        // 没有 url，所以 step_download 返回空列表，后续步骤也不应因"已安装"报错
        let result = install(&session, "myapp", &manifest, "main", &options);
        // 可能成功（无 url 无 bin 的极简 manifest），也可能在链接阶段失败
        // 关键是不会因为"已安装"而报错
        if let Err(e) = &result {
            assert!(!e.to_string().contains("已安装"));
        }
    }

    #[test]
    fn uninstall_nonexistent_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        let result = uninstall(&session, "nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未安装"));
    }

    #[test]
    fn reset_version_nonexistent_errors() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        // 创建 app 目录但不含目标版本
        let app_dir = dir.path().join("apps").join("myapp");
        std::fs::create_dir_all(&app_dir).unwrap();

        let result = reset_version(&session, "myapp", "2.0");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未安装"));
    }

    #[test]
    fn reset_version_switches_junction() {
        let dir = tempfile::tempdir().unwrap();
        let session = test_session(dir.path());

        let app_dir = dir.path().join("apps").join("myapp");
        let v1 = app_dir.join("1.0");
        let v2 = app_dir.join("2.0");
        std::fs::create_dir_all(&v1).unwrap();
        std::fs::create_dir_all(&v2).unwrap();
        std::fs::write(v1.join("marker.txt"), "v1").unwrap();
        std::fs::write(v2.join("marker.txt"), "v2").unwrap();

        // 创建 current → 1.0
        let current = app_dir.join("current");
        junction::create(&v1, &current).unwrap();

        // 切换到 2.0
        reset_version(&session, "myapp", "2.0").unwrap();

        // 验证 current 指向 2.0
        assert_eq!(
            std::fs::read_to_string(current.join("marker.txt")).unwrap(),
            "v2"
        );
    }

    #[test]
    fn install_emits_phase_events() {
        let dir = tempfile::tempdir().unwrap();
        let config = HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);

        // event_bus() 通过 OnceCell 自动初始化；提前获取 receiver 引用
        let _bus = session.event_bus();

        let manifest = minimal_manifest("1.0");
        let options = InstallOptions::default();
        let _ = install(&session, "testapp", &manifest, "main", &options);

        // 检查是否发出了 Resolve 事件
        let receiver = session.event_bus().receiver();
        let mut found_resolve = false;
        while let Ok(event) = receiver.try_recv() {
            if let Event::InstallPhaseStart { phase: InstallPhase::Resolve, .. } = &event {
                found_resolve = true;
            }
        }
        assert!(found_resolve, "应发出 Resolve 阶段事件");
    }

    #[test]
    fn find_latest_version_dir_picks_newest() {
        let dir = tempfile::tempdir().unwrap();
        let app_dir = dir.path().join("myapp");
        std::fs::create_dir_all(app_dir.join("1.0")).unwrap();
        std::fs::create_dir_all(app_dir.join("2.0")).unwrap();
        std::fs::create_dir_all(app_dir.join("current")).unwrap(); // 应被排除

        let latest = find_latest_version_dir(&app_dir).unwrap();
        assert!(latest.to_string_lossy().contains("2.0"));
    }
}
