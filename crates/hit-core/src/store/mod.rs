//! Hit 数据存储（`db.json`）
//!
//! 集中式 JSON 数据库，记录所有已安装软件与已注册 bucket。
//! 替代 Scoop 的分散式 `install.json` + `manifest.json` 方案。
//!
//! 原子写入策略：先写 `.json.tmp` 临时文件，再 `std::fs::rename` 覆盖目标，
//! 避免写入中断导致 db.json 损坏。

pub mod migration;
pub mod models;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use hit_common::error::{HitError, Result};
use hit_common::Session;
use sonic_rs::JsonValueMutTrait;
use sonic_rs::JsonValueTrait;

pub use models::{BucketInfo, InstalledPackage};

/// Hit 集中式数据库（对应 `~/.hit/db.json`）
pub struct Db {
    /// 文件路径（save 时写回此路径）
    path: PathBuf,
    /// schema 版本
    version: u32,
    /// 已安装软件（键 = app 名称）
    packages: BTreeMap<String, InstalledPackage>,
    /// 已注册 bucket（键 = bucket 名称）
    buckets: BTreeMap<String, BucketInfo>,
    /// 是否有未保存的变更
    dirty: bool,
}

/// 获取 db.json 路径
pub fn db_path(session: &Session) -> PathBuf {
    session.root_path().join("db.json")
}

impl Db {
    /// 从 db.json 加载数据库
    ///
    /// - 文件不存在 → 返回空数据库
    /// - 文件存在 → 解析 → 迁移 → 类型化反序列化
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::empty(path.to_path_buf()));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| HitError::io(format!("读取 db.json: {}", path.display()), e))?;

        let mut raw: sonic_rs::Value = sonic_rs::from_str(&content)
            .map_err(|e| HitError::Config {
                message: format!("解析 db.json 失败：{e}"),
            })?;

        // 执行 schema 迁移
        let version = migration::migrate(&mut raw)?;

        // 从迁移后的 Value 提取类型化字段
        let packages = extract_map::<InstalledPackage>(&raw, "packages")?;
        let buckets = extract_map::<BucketInfo>(&raw, "buckets")?;

        Ok(Self {
            path: path.to_path_buf(),
            version,
            packages,
            buckets,
            dirty: false,
        })
    }

    /// 原子写入 db.json
    ///
    /// 写入流程：序列化 → 写 `.json.tmp` → `rename` 覆盖目标。
    /// 保证写入中断不会损坏 db.json。
    pub fn save(&mut self) -> Result<()> {
        let json = self.to_json_value()?;
        let content = sonic_rs::to_string_pretty(&json)
            .map_err(|e| HitError::Config {
                message: format!("序列化 db.json 失败：{e}"),
            })?;

        // 确保父目录存在
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                HitError::io(
                    format!("创建目录: {}", parent.display()),
                    e,
                )
            })?;
        }

        // 原子写入：temp file 在同目录（保证同 volume）
        let tmp_path = self.path.with_extension("json.tmp");
        std::fs::write(&tmp_path, content.as_bytes()).map_err(|e| {
            HitError::io(format!("写入临时文件: {}", tmp_path.display()), e)
        })?;

        // rename 覆盖目标文件
        std::fs::rename(&tmp_path, &self.path).map_err(|e| {
            let _ = std::fs::remove_file(&tmp_path);
            HitError::io("原子重命名 db.json", e)
        })?;

        self.dirty = false;
        Ok(())
    }

    /// 创建空数据库
    fn empty(path: PathBuf) -> Self {
        Self {
            path,
            version: migration::CURRENT_VERSION,
            packages: BTreeMap::new(),
            buckets: BTreeMap::new(),
            dirty: false,
        }
    }

    // ── Package CRUD ──

    pub fn get_package(&self, app: &str) -> Option<&InstalledPackage> {
        self.packages.get(app)
    }

    pub fn get_package_mut(&mut self, app: &str) -> Option<&mut InstalledPackage> {
        self.packages.get_mut(app)
    }

    pub fn insert_package(&mut self, app: String, pkg: InstalledPackage) {
        self.packages.insert(app, pkg);
        self.dirty = true;
    }

    pub fn remove_package(&mut self, app: &str) -> Option<InstalledPackage> {
        let removed = self.packages.remove(app);
        if removed.is_some() {
            self.dirty = true;
        }
        removed
    }

    pub fn is_installed(&self, app: &str) -> bool {
        self.packages.contains_key(app)
    }

    pub fn list_packages(&self) -> &BTreeMap<String, InstalledPackage> {
        &self.packages
    }

    pub fn package_count(&self) -> usize {
        self.packages.len()
    }

    // ── Bucket CRUD ──

    pub fn get_bucket(&self, name: &str) -> Option<&BucketInfo> {
        self.buckets.get(name)
    }

    pub fn insert_bucket(&mut self, name: String, info: BucketInfo) {
        self.buckets.insert(name, info);
        self.dirty = true;
    }

    pub fn remove_bucket(&mut self, name: &str) -> Option<BucketInfo> {
        let removed = self.buckets.remove(name);
        if removed.is_some() {
            self.dirty = true;
        }
        removed
    }

    pub fn list_buckets(&self) -> &BTreeMap<String, BucketInfo> {
        &self.buckets
    }

    // ── Metadata ──

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    // ── 内部辅助 ──

    /// 序列化为 sonic_rs::Value（用于 pretty-print）
    fn to_json_value(&self) -> Result<sonic_rs::Value> {
        let packages_val = sonic_rs::to_value(&self.packages)
            .map_err(|e| HitError::Config {
                message: format!("序列化 packages: {e}"),
            })?;
        let buckets_val = sonic_rs::to_value(&self.buckets)
            .map_err(|e| HitError::Config {
                message: format!("序列化 buckets: {e}"),
            })?;

        let mut obj = sonic_rs::json!({});
        if let Some(map) = obj.as_object_mut() {
            map.insert(&"version", self.version);
            map.insert(&"packages", packages_val);
            map.insert(&"buckets", buckets_val);
        }
        Ok(obj)
    }
}

/// 从 Value 中提取 `BTreeMap<String, T>`
fn extract_map<T: serde::de::DeserializeOwned>(
    raw: &sonic_rs::Value,
    key: &str,
) -> Result<BTreeMap<String, T>> {
    match raw.get(key) {
        Some(v) => {
            let json_str = sonic_rs::to_string(v)
                .map_err(|e| HitError::Config {
                    message: format!("提取 {key} 失败：{e}"),
                })?;
            sonic_rs::from_str(&json_str)
                .map_err(|e| HitError::Config {
                    message: format!("反序列化 {key} 失败：{e}"),
                })
        }
        None => Ok(BTreeMap::new()),
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_nonexistent_returns_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");
        let db = Db::load(&path).unwrap();
        assert_eq!(db.package_count(), 0);
        assert_eq!(db.version(), migration::CURRENT_VERSION);
        assert!(!db.is_dirty());
    }

    #[test]
    fn save_creates_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");
        let mut db = Db::load(&path).unwrap();
        db.save().unwrap();
        assert!(path.exists());
    }

    #[test]
    fn save_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");

        // 写入数据
        let mut db = Db::load(&path).unwrap();
        db.insert_package(
            "git".into(),
            InstalledPackage {
                version: "2.45.1".into(),
                bucket: "main".into(),
                install_date: "2024-06-15T10:00:00Z".into(),
                architecture: "64bit".into(),
                shims: vec!["git.exe".into()],
                held: true,
                ..Default::default()
            },
        );
        db.insert_bucket(
            "main".into(),
            BucketInfo {
                name: "main".into(),
                url: "https://github.com/ScoopInstaller/Main".into(),
                last_update: "2024-06-15T12:00:00Z".into(),
            },
        );
        db.save().unwrap();

        // 重新加载
        let db2 = Db::load(&path).unwrap();
        assert_eq!(db2.package_count(), 1);
        let git = db2.get_package("git").unwrap();
        assert_eq!(git.version, "2.45.1");
        assert!(git.held);
        assert_eq!(git.shims, vec!["git.exe"]);

        let main = db2.get_bucket("main").unwrap();
        assert_eq!(main.url, "https://github.com/ScoopInstaller/Main");
    }

    #[test]
    fn atomic_write_no_tmp_remnant() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");
        let tmp_path = path.with_extension("json.tmp");

        let mut db = Db::load(&path).unwrap();
        db.insert_package("test".into(), InstalledPackage::default());
        db.save().unwrap();

        // 临时文件不应残留
        assert!(!tmp_path.exists(), "save 后 .json.tmp 应被 rename 移除");
        assert!(path.exists());
    }

    #[test]
    fn insert_remove_package() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");
        let mut db = Db::load(&path).unwrap();

        assert!(!db.is_installed("git"));

        db.insert_package("git".into(), InstalledPackage {
            version: "2.45".into(),
            ..Default::default()
        });
        assert!(db.is_installed("git"));
        assert!(db.is_dirty());

        let removed = db.remove_package("git");
        assert!(removed.is_some());
        assert!(!db.is_installed("git"));
    }

    #[test]
    fn dirty_flag_tracking() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");
        let mut db = Db::load(&path).unwrap();
        assert!(!db.is_dirty());

        db.insert_package("a".into(), InstalledPackage::default());
        assert!(db.is_dirty());

        db.save().unwrap();
        assert!(!db.is_dirty());
    }

    #[test]
    fn list_packages_btree_sorted() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");
        let mut db = Db::load(&path).unwrap();

        db.insert_package("python".into(), InstalledPackage::default());
        db.insert_package("git".into(), InstalledPackage::default());
        db.insert_package("nodejs".into(), InstalledPackage::default());

        let keys: Vec<_> = db.list_packages().keys().collect();
        assert_eq!(keys, vec!["git", "nodejs", "python"]);
    }

    #[test]
    fn bucket_crud() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");
        let mut db = Db::load(&path).unwrap();

        db.insert_bucket(
            "extras".into(),
            BucketInfo {
                name: "extras".into(),
                url: "https://github.com/ScoopInstaller/Extras".into(),
                last_update: "2024-01-01T00:00:00Z".into(),
            },
        );
        assert!(db.get_bucket("extras").is_some());
        assert_eq!(db.list_buckets().len(), 1);

        let removed = db.remove_bucket("extras");
        assert!(removed.is_some());
        assert!(db.get_bucket("extras").is_none());
    }

    #[test]
    fn load_with_migration() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("db.json");

        // 手动写入无 version 字段的 db.json
        std::fs::write(
            &path,
            r#"{"packages": {"git": {"version": "1.0"}}, "buckets": {}}"#,
        )
        .unwrap();

        let db = Db::load(&path).unwrap();
        assert_eq!(db.version(), migration::CURRENT_VERSION);
        assert!(db.get_package("git").is_some());
    }

    #[test]
    fn db_path_from_session() {
        use hit_common::config::HitConfig;

        let dir = tempdir().unwrap();
        let config = HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let expected = dir.path().join("db.json");
        assert_eq!(db_path(&session), expected);
    }
}
