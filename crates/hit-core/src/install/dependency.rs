//! 依赖解析器
//!
//! 给定根 manifest 的 `depends` 字段，递归构建依赖图并做拓扑排序。
//! 使用三色标记（白 / 灰 / 黑）检测循环依赖。
//!
//! - 依赖格式：`"name"` 或 `"bucket/name"`
//! - 已安装的依赖（`apps/<name>/current` 存在）会被跳过
//! - Manifest 通过 bucket 目录查找：`<buckets>/<bucket>/<name>.json` 或 `<buckets>/<bucket>/bucket/<name>.json`

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use hit_common::{HitError, Result, Session};

use crate::bucket::list_buckets;
use crate::manifest::{parse_str, Manifest};

/// 已解析的依赖项
#[derive(Debug, Clone)]
pub struct ResolvedDep {
    /// 依赖名称（不含 bucket 前缀）
    pub name: String,
    /// 来源 bucket（裸名时为空串）
    pub bucket: String,
    /// 已加载的 manifest
    pub manifest: Manifest,
}

/// 解析依赖树（拓扑排序）
///
/// 返回顺序：最底层依赖在前，`root_app` 自身不出现在结果中。
/// 已安装的依赖被跳过（不加入结果）。
pub fn resolve_dependencies(
    session: &Session,
    root_app: &str,
    root_manifest: &Manifest,
) -> Result<Vec<ResolvedDep>> {
    let mut visiting = HashSet::new(); // 灰色：正在 DFS 栈中
    let mut visited = HashSet::new(); // 黑色：已完整处理
    let mut out = Vec::new();

    // 根节点预入 visiting 栈
    visiting.insert(root_app.to_string());

    // 展开 root 的依赖（root_app 自身不会出现在结果中）
    let apps_path = session.apps_path();
    for dep_spec in root_manifest.depends_list() {
        let (bucket_opt, name) = parse_dep_spec(dep_spec);
        // 自依赖：直接跳过
        if name == root_app {
            continue;
        }
        // 已安装跳过
        let installed_path = apps_path.join(name).join("current");
        if installed_path.exists() {
            visited.insert(name.to_string());
            continue;
        }
        if visited.contains(name) {
            continue;
        }
        let dep_manifest = load_dep_manifest(session, bucket_opt, name)?;
        dfs_visit(session, name, &dep_manifest, &mut visiting, &mut visited, &mut out)?;
    }

    // root 出栈（不加入 visited 也不加入 out）
    visiting.remove(root_app);

    Ok(out)
}

/// 解析 `"bucket/name"` 或 `"name"` 格式
///
/// 返回 `(Some(bucket), name)` 或 `(None, name)`
pub fn parse_dep_spec(spec: &str) -> (Option<&str>, &str) {
    if let Some((bucket, name)) = spec.split_once('/') {
        let bucket = bucket.trim();
        let name = name.trim();
        if bucket.is_empty() || name.is_empty() {
            (None, spec)
        } else {
            (Some(bucket), name)
        }
    } else {
        (None, spec)
    }
}

/// DFS 访问单个节点（递归处理其依赖）
fn dfs_visit(
    session: &Session,
    app: &str,
    manifest: &Manifest,
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
    out: &mut Vec<ResolvedDep>,
) -> Result<()> {
    if visited.contains(app) {
        return Ok(());
    }
    if !visiting.insert(app.to_string()) {
        return Err(HitError::Install {
            app: app.to_string(),
            message: format!("循环依赖：{} 被重复访问", app),
        });
    }

    let apps_path = session.apps_path();
    for dep_spec in manifest.depends_list() {
        let (bucket_opt, name) = parse_dep_spec(dep_spec);
        // 已安装跳过：使用 session 的 apps_path 以支持 with_config 测试
        let installed_path = apps_path.join(name).join("current");
        if installed_path.exists() {
            visited.insert(name.to_string());
            continue;
        }
        // 已在黑色集合中跳过
        if visited.contains(name) {
            continue;
        }
        // 加载 manifest
        let dep_manifest = load_dep_manifest(session, bucket_opt, name)?;
        dfs_visit(session, name, &dep_manifest, visiting, visited, out)?;
    }

    visiting.remove(app);
    visited.insert(app.to_string());

    out.push(ResolvedDep {
        name: app.to_string(),
        bucket: String::new(),
        manifest: manifest.clone(),
    });

    Ok(())
}

/// 加载依赖的 manifest
///
/// - 若提供 bucket 名：直接查 `<buckets>/<bucket>/<name>.json`（含 bucket/ 子目录）
/// - 否则遍历所有 bucket 查找第一个匹配项
fn load_dep_manifest(
    session: &Session,
    bucket_opt: Option<&str>,
    name: &str,
) -> Result<Manifest> {
    let buckets_root = session.buckets_path();
    if let Some(bucket_name) = bucket_opt {
        let bucket_dir = buckets_root.join(bucket_name);
        let path = locate_manifest_in_bucket(&bucket_dir, name)?;
        let content = std::fs::read_to_string(&path).map_err(|e| {
            HitError::io(format!("读取 manifest '{}'", path.display()), e)
        })?;
        return parse_str(&content).map_err(|e| HitError::Manifest {
            app: name.to_string(),
            message: e.to_string(),
        });
    }

    for bucket in list_buckets(session)? {
        if let Ok(path) = locate_manifest_in_bucket(&bucket.path, name) {
            let content = std::fs::read_to_string(&path).map_err(|e| {
                HitError::io(format!("读取 manifest '{}'", path.display()), e)
            })?;
            return parse_str(&content).map_err(|e| HitError::Manifest {
                app: name.to_string(),
                message: e.to_string(),
            });
        }
    }

    Err(HitError::NotFound {
        kind: "app".to_string(),
        name: name.to_string(),
        extra: String::new(),
    })
}

/// 在指定 bucket 目录中查找 manifest
///
/// Scoop 兼容两种布局：
/// 1. `<bucket>/<name>.json`（旧布局）
/// 2. `<bucket>/bucket/<name>.json`（Scoop v0.3.0+ 子目录布局）
fn locate_manifest_in_bucket(bucket_dir: &Path, name: &str) -> Result<PathBuf> {
    let direct = bucket_dir.join(format!("{name}.json"));
    if direct.is_file() {
        return Ok(direct);
    }
    let sub = bucket_dir.join("bucket").join(format!("{name}.json"));
    if sub.is_file() {
        return Ok(sub);
    }
    Err(HitError::NotFound {
        kind: "app".to_string(),
        name: name.to_string(),
        extra: format!(" (in bucket '{}')", bucket_dir.display()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;
    use std::fs;

    fn test_session(dir: &Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    fn write_manifest(dir: &Path, name: &str, deps: &[&str]) {
        let deps_json = if deps.is_empty() {
            String::new()
        } else {
            let quoted: Vec<String> = deps.iter().map(|d| format!("\"{d}\"")).collect();
            format!(r#","depends":[{}]"#, quoted.join(","))
        };
        let json = format!(
            r#"{{"version":"1.0","description":"test","homepage":"https://example.com","license":"MIT"{deps_json}}}"#
        );
        fs::write(dir.join(format!("{name}.json")), json).unwrap();
    }

    #[test]
    fn parse_dep_spec_bucket_qualified() {
        assert_eq!(parse_dep_spec("main/git"), (Some("main"), "git"));
        assert_eq!(parse_dep_spec("extras/python"), (Some("extras"), "python"));
    }

    #[test]
    fn parse_dep_spec_bare() {
        assert_eq!(parse_dep_spec("git"), (None, "git"));
    }

    #[test]
    fn parse_dep_spec_malformed_slash() {
        // 空 bucket 或空 name 时退化为裸名
        assert_eq!(parse_dep_spec("/name"), (None, "/name"));
        assert_eq!(parse_dep_spec("bucket/"), (None, "bucket/"));
    }

    #[test]
    fn resolve_linear_chain() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        fs::create_dir_all(&bucket_dir).unwrap();
        // A -> B -> C
        write_manifest(&bucket_dir, "A", &["B"]);
        write_manifest(&bucket_dir, "B", &["C"]);
        write_manifest(&bucket_dir, "C", &[]);

        let session = test_session(dir.path());
        let root = crate::manifest::parse_str(&fs::read_to_string(bucket_dir.join("A.json")).unwrap()).unwrap();
        let result = resolve_dependencies(&session, "A", &root).unwrap();

        // 拓扑排序：C 先，B 后；A 自身不出现
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "C");
        assert_eq!(result[1].name, "B");
    }

    #[test]
    fn resolve_diamond_dependency() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        fs::create_dir_all(&bucket_dir).unwrap();
        // A -> B, A -> C; B -> D; C -> D
        write_manifest(&bucket_dir, "A", &["B", "C"]);
        write_manifest(&bucket_dir, "B", &["D"]);
        write_manifest(&bucket_dir, "C", &["D"]);
        write_manifest(&bucket_dir, "D", &[]);

        let session = test_session(dir.path());
        let root = crate::manifest::parse_str(&fs::read_to_string(bucket_dir.join("A.json")).unwrap()).unwrap();
        let result = resolve_dependencies(&session, "A", &root).unwrap();

        // D 出现且仅出现一次
        let d_count = result.iter().filter(|d| d.name == "D").count();
        assert_eq!(d_count, 1);
        // 总依赖数 = 3 (B, C, D)
        assert_eq!(result.len(), 3);
        // D 必须在 B 和 C 之前
        let d_pos = result.iter().position(|d| d.name == "D").unwrap();
        let b_pos = result.iter().position(|d| d.name == "B").unwrap();
        let c_pos = result.iter().position(|d| d.name == "C").unwrap();
        assert!(d_pos < b_pos);
        assert!(d_pos < c_pos);
    }

    #[test]
    fn resolve_circular_dependency_errors() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        fs::create_dir_all(&bucket_dir).unwrap();
        // A -> B -> A
        write_manifest(&bucket_dir, "A", &["B"]);
        write_manifest(&bucket_dir, "B", &["A"]);

        let session = test_session(dir.path());
        let root = crate::manifest::parse_str(&fs::read_to_string(bucket_dir.join("A.json")).unwrap()).unwrap();
        let err = resolve_dependencies(&session, "A", &root).unwrap_err();
        match err {
            HitError::Install { message, .. } => {
                assert!(message.contains("循环"), "应报告循环依赖：{message}");
            }
            other => panic!("期望 Install 错误，实际：{other:?}"),
        }
    }

    #[test]
    fn resolve_skips_already_installed() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        let apps_dir = dir.path().join("apps").join("C").join("current");
        fs::create_dir_all(&bucket_dir).unwrap();
        fs::create_dir_all(&apps_dir).unwrap(); // C 已安装

        write_manifest(&bucket_dir, "A", &["B", "C"]);
        write_manifest(&bucket_dir, "B", &[]);

        let session = test_session(dir.path());
        let root = crate::manifest::parse_str(&fs::read_to_string(bucket_dir.join("A.json")).unwrap()).unwrap();
        let result = resolve_dependencies(&session, "A", &root).unwrap();

        // C 被跳过，只有 B
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "B");
    }

    #[test]
    fn resolve_missing_dependency_errors() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir, "A", &["missing"]);

        let session = test_session(dir.path());
        let root = crate::manifest::parse_str(&fs::read_to_string(bucket_dir.join("A.json")).unwrap()).unwrap();
        let err = resolve_dependencies(&session, "A", &root).unwrap_err();
        match err {
            HitError::NotFound { name, .. } => assert_eq!(name, "missing"),
            other => panic!("期望 NotFound 错误，实际：{other:?}"),
        }
    }
}
