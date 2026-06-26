//! 软件索引构建
//!
//! 遍历所有本地 bucket 目录下的 `.json` manifest 文件，
//! 并行解析为摘要信息（`PackageSummary`），构建内存索引供 search/info 命令使用。

use std::collections::HashMap;
use std::path::PathBuf;

use rayon::prelude::*;

use hit_common::error::Result;
use hit_common::Session;

use crate::bucket::types::list_buckets;
use crate::manifest::parse_str;

/// Bucket 优先级（数值越小优先级越高）
///
/// 当多个 bucket 包含同名软件时，按此顺序选择最佳匹配。
const BUCKET_PRIORITY: &[&str] = &["main", "extras", "versions"];

/// 软件包摘要（从 manifest 文件提取的最小信息集）
#[derive(Debug, Clone)]
pub struct PackageSummary {
    /// 软件名称（来自文件名 stem，如 "git"、"python"）
    pub name: String,
    /// 所属 bucket 名称
    pub bucket: String,
    /// 版本号
    pub version: String,
    /// 描述
    pub description: String,
}

/// 软件索引：bucket 名 → 该 bucket 下所有软件摘要列表
pub struct SoftwareIndex {
    pub packages: HashMap<String, Vec<PackageSummary>>,
}

/// 构建全量软件索引
///
/// 遍历所有本地 bucket，并行解析每个 manifest 文件的摘要信息。
/// 解析失败的文件静默跳过（tracing warn），不影响整体构建。
pub fn build_index(session: &Session) -> Result<SoftwareIndex> {
    let buckets = list_buckets(session)?;

    // 收集所有待解析的 (bucket_name, file_path) 对
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for bucket in &buckets {
        collect_manifest_files(&bucket.name, &bucket.path, &mut files);
    }

    // 并行解析
    let summaries: Vec<PackageSummary> = files
        .par_iter()
        .filter_map(|(bucket_name, path)| {
            let content = std::fs::read_to_string(path).ok()?;
            let manifest = match parse_str(&content) {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!(
                        "跳过无效 manifest '{}': {e}",
                        path.display()
                    );
                    return None;
                }
            };
            let name = path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            Some(PackageSummary {
                name,
                bucket: bucket_name.clone(),
                version: manifest.version,
                description: manifest.description,
            })
        })
        .collect();

    // 按 bucket 分组
    let mut packages: HashMap<String, Vec<PackageSummary>> = HashMap::new();
    for s in summaries {
        packages.entry(s.bucket.clone()).or_default().push(s);
    }

    // 每个 bucket 内按名称排序，保证输出稳定
    for list in packages.values_mut() {
        list.sort_by(|a, b| a.name.cmp(&b.name));
    }

    Ok(SoftwareIndex { packages })
}

/// 收集指定 bucket 目录下所有 manifest .json 文件路径
///
/// 支持 Scoop v0.3.0+ 子目录布局（`bucket/` 子目录下的 .json 也被收集）。
/// 排除 `bucket.json`（Hit 元数据文件）。
fn collect_manifest_files(
    bucket_name: &str,
    bucket_path: &std::path::Path,
    out: &mut Vec<(String, PathBuf)>,
) {
    // 根目录下的 .json 文件
    if let Ok(entries) = std::fs::read_dir(bucket_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_manifest_file(&path) {
                out.push((bucket_name.to_string(), path));
            }
        }
    }

    // bucket/ 子目录下的 .json 文件（Scoop v0.3.0+ 布局）
    let sub_dir = bucket_path.join("bucket");
    if sub_dir.is_dir()
        && let Ok(entries) = std::fs::read_dir(&sub_dir)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_manifest_file(&path) {
                out.push((bucket_name.to_string(), path));
            }
        }
    }
}

/// 判断路径是否为 manifest 文件（.json 且非 bucket.json）
fn is_manifest_file(path: &std::path::Path) -> bool {
    path.is_file()
        && path.extension().is_some_and(|ext| ext == "json")
        && path.file_name().is_some_and(|f| f != "bucket.json")
}

impl SoftwareIndex {
    /// 搜索匹配关键词的软件包（名称或描述包含，不区分大小写）
    pub fn search(&self, query: &str) -> Vec<&PackageSummary> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<&PackageSummary> = self
            .packages
            .values()
            .flatten()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect();
        results.sort_by(|a, b| a.name.cmp(&b.name).then(a.bucket.cmp(&b.bucket)));
        results
    }

    /// 精确查找指定软件名（返回所有 bucket 中的匹配项）
    pub fn find(&self, name: &str) -> Vec<&PackageSummary> {
        let mut results: Vec<&PackageSummary> = self
            .packages
            .values()
            .flatten()
            .filter(|pkg| pkg.name == name)
            .collect();
        results.sort_by(|a, b| a.bucket.cmp(&b.bucket));
        results
    }

    /// 索引中软件包总数
    pub fn total_packages(&self) -> usize {
        self.packages.values().map(Vec::len).sum()
    }

    /// 从多个候选中选择最佳匹配（按 bucket 优先级排序）
    ///
    /// 优先级顺序：main > extras > versions > 其他（按字母序）
    pub fn best_match(&self, name: &str) -> Option<&PackageSummary> {
        let candidates = self.find(name);
        candidates.into_iter().min_by_key(|p| {
            BUCKET_PRIORITY
                .iter()
                .position(|&b| b == p.bucket.as_str())
                .unwrap_or(BUCKET_PRIORITY.len())
        })
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;
    use std::path::Path;

    fn test_session(dir: &Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    fn write_manifest(path: &Path, version: &str, description: &str) {
        let json = format!(
            r#"{{"version":"{version}","description":"{description}","homepage":"https://example.com","license":"MIT"}}"#
        );
        std::fs::write(path, json).unwrap();
    }

    #[test]
    fn build_index_empty_buckets() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets")).unwrap();
        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();
        assert_eq!(index.total_packages(), 0);
    }

    #[test]
    fn build_index_collects_manifests() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir.join("git.json"), "2.40.0", "Git for Windows");
        write_manifest(&bucket_dir.join("python.json"), "3.12.0", "Python interpreter");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        assert_eq!(index.total_packages(), 2);
        let main_pkgs = index.packages.get("main").unwrap();
        assert_eq!(main_pkgs.len(), 2);
        // 按名称排序
        assert_eq!(main_pkgs[0].name, "git");
        assert_eq!(main_pkgs[0].version, "2.40.0");
        assert_eq!(main_pkgs[1].name, "python");
        assert_eq!(main_pkgs[1].version, "3.12.0");
    }

    #[test]
    fn build_index_excludes_bucket_json() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("extras");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir.join("firefox.json"), "120.0", "Web browser");
        // bucket.json 不应被索引
        std::fs::write(
            bucket_dir.join("bucket.json"),
            r#"{"name":"Extras","description":"Community extras"}"#,
        )
        .unwrap();

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        assert_eq!(index.total_packages(), 1);
        let pkgs = index.packages.get("extras").unwrap();
        assert_eq!(pkgs[0].name, "firefox");
    }

    #[test]
    fn build_index_handles_bucket_subdir() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        let sub_dir = bucket_dir.join("bucket");
        std::fs::create_dir_all(&sub_dir).unwrap();
        // 根目录一个
        write_manifest(&bucket_dir.join("root.json"), "1.0", "Root level");
        // bucket/ 子目录两个
        write_manifest(&sub_dir.join("sub1.json"), "2.0", "Subdir one");
        write_manifest(&sub_dir.join("sub2.json"), "3.0", "Subdir two");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        assert_eq!(index.total_packages(), 3);
    }

    #[test]
    fn build_index_skips_malformed_json() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir.join("good.json"), "1.0", "Valid manifest");
        // 格式错误的 .json 文件
        std::fs::write(bucket_dir.join("bad.json"), "{not valid json!!!").unwrap();

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        // bad.json 被跳过，good.json 正常解析
        assert_eq!(index.total_packages(), 1);
        let pkgs = index.packages.get("main").unwrap();
        assert_eq!(pkgs[0].name, "good");
    }

    #[test]
    fn search_finds_by_name() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir.join("git.json"), "2.40.0", "Version control");
        write_manifest(&bucket_dir.join("python.json"), "3.12.0", "Programming language");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        let results = index.search("git");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "git");
    }

    #[test]
    fn search_finds_by_description() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir.join("git.json"), "2.40.0", "Version control system");
        write_manifest(&bucket_dir.join("python.json"), "3.12.0", "Programming language");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        let results = index.search("programming");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "python");
    }

    #[test]
    fn search_case_insensitive() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir.join("Git.json"), "2.40.0", "Version Control");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        let results = index.search("GIT");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Git");
    }

    #[test]
    fn find_exact_match() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("main");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        write_manifest(&bucket_dir.join("git.json"), "2.40.0", "Version control");
        write_manifest(&bucket_dir.join("python.json"), "3.12.0", "Programming language");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        let results = index.find("git");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "git");
        assert_eq!(results[0].bucket, "main");

        let results = index.find("nonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn find_across_buckets() {
        let dir = tempfile::tempdir().unwrap();
        let main_dir = dir.path().join("buckets").join("main");
        let extras_dir = dir.path().join("buckets").join("extras");
        std::fs::create_dir_all(&main_dir).unwrap();
        std::fs::create_dir_all(&extras_dir).unwrap();
        write_manifest(&main_dir.join("git.json"), "2.40.0", "Version control");
        write_manifest(&extras_dir.join("git.json"), "2.39.0", "Version control (old)");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        let results = index.find("git");
        assert_eq!(results.len(), 2);
        // 按 bucket 排序
        assert_eq!(results[0].bucket, "extras");
        assert_eq!(results[1].bucket, "main");
    }

    #[test]
    fn total_packages_count() {
        let dir = tempfile::tempdir().unwrap();
        let main_dir = dir.path().join("buckets").join("main");
        let extras_dir = dir.path().join("buckets").join("extras");
        std::fs::create_dir_all(&main_dir).unwrap();
        std::fs::create_dir_all(&extras_dir).unwrap();
        write_manifest(&main_dir.join("a.json"), "1.0", "A");
        write_manifest(&main_dir.join("b.json"), "1.0", "B");
        write_manifest(&extras_dir.join("c.json"), "1.0", "C");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        assert_eq!(index.total_packages(), 3);
    }

    #[test]
    fn best_match_prefers_main() {
        let dir = tempfile::tempdir().unwrap();
        let main_dir = dir.path().join("buckets").join("main");
        let extras_dir = dir.path().join("buckets").join("extras");
        std::fs::create_dir_all(&main_dir).unwrap();
        std::fs::create_dir_all(&extras_dir).unwrap();
        write_manifest(&main_dir.join("git.json"), "2.45.1", "Git main");
        write_manifest(&extras_dir.join("git.json"), "2.44.0", "Git extras");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        let best = index.best_match("git").unwrap();
        assert_eq!(best.bucket, "main");
        assert_eq!(best.version, "2.45.1");
    }

    #[test]
    fn best_match_fallback_to_extras() {
        let dir = tempfile::tempdir().unwrap();
        let extras_dir = dir.path().join("buckets").join("extras");
        std::fs::create_dir_all(&extras_dir).unwrap();
        write_manifest(&extras_dir.join("git.json"), "2.44.0", "Git extras");

        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        let best = index.best_match("git").unwrap();
        assert_eq!(best.bucket, "extras");
    }

    #[test]
    fn best_match_returns_none_for_unknown() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let index = build_index(&session).unwrap();

        assert!(index.best_match("nonexistent").is_none());
    }
}
