//! Session/Context 模式
//!
//! 参考 `ref/Hok/crates/libscoop/src/session.rs`，Session 作为 Hit 所有核心操作的
//! 统一入口，持有配置、事件总线与路径缓存。
//!
//! - 所有核心函数签名以 `session: &Session` 为首参数
//! - `Session::new()` 自动搜索配置文件路径，加载失败则使用默认配置
//! - 路径缓存避免重复计算 `~/.hit/` 路径

use std::cell::{OnceCell, RefCell};
use std::path::PathBuf;

use crate::config::HitConfig;
use crate::error::Result;
use crate::event::{Event, EventBus, Receiver, Sender};
use crate::paths;

/// 路径缓存：一次性计算，复用
#[derive(Debug, Clone)]
struct PathCache {
    root: PathBuf,
    apps: PathBuf,
    shims: PathBuf,
    cache: PathBuf,
    persist: PathBuf,
    buckets: PathBuf,
}

impl PathCache {
    /// 构建路径缓存：优先使用 config.root_path（用户显式指定），否则回退到 paths::root_path()
    /// （基于 HIT_ROOT / SCOOP / USERPROFILE 回退链）。
    fn new(config: &HitConfig) -> Self {
        let root = config
            .root_path
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(paths::root_path);
        Self {
            root: root.clone(),
            apps: root.join("apps"),
            shims: root.join("shims"),
            cache: root.join("cache"),
            persist: root.join("persist"),
            buckets: root.join("buckets"),
        }
    }
}

/// Hit 运行时会话
///
/// 典型用法：
/// ```no_run
/// use hit_common::Session;
/// let session = Session::new().expect("初始化 Session 失败");
/// let cfg = session.config();
/// let emitter = session.emitter();
/// ```
pub struct Session {
    /// 用户配置（RefCell 允许局部修改后 save 回写）
    config: RefCell<HitConfig>,
    /// 配置源路径（`None` 表示使用默认配置、未从文件加载）
    config_path: Option<PathBuf>,
    /// 事件总线（OnceCell 允许延迟初始化或测试时跳过）
    event_bus: OnceCell<EventBus>,
    /// 路径缓存
    paths: PathCache,
}

impl Session {
    /// 创建 Session：自动从 `~/.hit/config.json` 加载配置（不存在则用默认值）
    ///
    /// 注意：配置文件自身的位置由 `paths::root_path()` 决定（基于环境变量回退链），
    /// 而 `config.root_path` 字段仅影响后续 `apps/shims/cache/persist/buckets` 的路径计算。
    pub fn new() -> Result<Self> {
        let path = HitConfig::default_path();
        let config = HitConfig::load(&path)?;
        let paths = PathCache::new(&config);
        Ok(Self {
            config: RefCell::new(config),
            config_path: Some(path),
            event_bus: OnceCell::new(),
            paths,
        })
    }

    /// 创建使用默认配置的 Session（不读文件、不绑定路径）
    pub fn with_defaults() -> Self {
        let config = HitConfig::default();
        Self {
            paths: PathCache::new(&config),
            config: RefCell::new(config),
            config_path: None,
            event_bus: OnceCell::new(),
        }
    }

    /// 创建带指定配置的 Session（用于测试或注入）
    pub fn with_config(config: HitConfig) -> Self {
        Self {
            paths: PathCache::new(&config),
            config: RefCell::new(config),
            config_path: None,
            event_bus: OnceCell::new(),
        }
    }

    // ── 配置访问 ──────────────────────────────────────────────────

    /// 借用当前配置（注意：借用期间不可变）
    pub fn config(&self) -> std::cell::Ref<'_, HitConfig> {
        self.config.borrow()
    }

    /// 可变借用配置（修改后应调用 `save_config` 持久化）
    pub fn config_mut(&self) -> std::cell::RefMut<'_, HitConfig> {
        self.config.borrow_mut()
    }

    /// 将当前配置保存到原加载路径；若无加载路径则返回 Ok(())
    pub fn save_config(&self) -> Result<()> {
        if let Some(path) = &self.config_path {
            self.config.borrow().save(path)?;
        }
        Ok(())
    }

    // ── 事件总线 ──────────────────────────────────────────────────

    /// 获取 EventBus（不存在则自动创建默认容量）
    pub fn event_bus(&self) -> &EventBus {
        self.event_bus.get_or_init(EventBus::new)
    }

    /// 便捷获取发送端
    pub fn emitter(&self) -> Sender {
        self.event_bus().emitter()
    }

    /// 便捷获取接收端
    pub fn receiver(&self) -> &Receiver {
        self.event_bus().receiver()
    }

    /// 发送事件（EventBus 未初始化时延迟初始化后发送）
    pub fn emit(&self, event: Event) {
        self.event_bus().emit(event);
    }

    // ── 路径缓存 ──────────────────────────────────────────────────

    /// Hit 根目录（`~/.hit/`）
    pub fn root_path(&self) -> &PathBuf {
        &self.paths.root
    }

    /// 软件安装目录（`~/.hit/apps/`）
    pub fn apps_path(&self) -> &PathBuf {
        &self.paths.apps
    }

    /// Shim 目录（`~/.hit/shims/`）
    pub fn shims_path(&self) -> &PathBuf {
        &self.paths.shims
    }

    /// 下载缓存目录（`~/.hit/cache/`）
    pub fn cache_path(&self) -> &PathBuf {
        &self.paths.cache
    }

    /// Persist 目录（`~/.hit/persist/`）
    pub fn persist_path(&self) -> &PathBuf {
        &self.paths.persist
    }

    /// Bucket 仓库目录（`~/.hit/buckets/`）
    pub fn buckets_path(&self) -> &PathBuf {
        &self.paths.buckets
    }
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("config_path", &self.config_path)
            .field("event_bus", &self.event_bus.get())
            .field("root", &self.paths.root)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::acquire_env_lock;

    #[test]
    fn with_defaults_uses_default_config() {
        let _guard = acquire_env_lock();
        let s = Session::with_defaults();
        assert!(!s.config().no_junction);
        assert_eq!(s.config().auto_cleanup_days, 30);
    }

    #[test]
    fn path_cache_is_consistent() {
        let _guard = acquire_env_lock();
        let s = Session::with_defaults();
        assert!(s.apps_path().ends_with("apps"));
        assert!(s.shims_path().ends_with("shims"));
    }

    #[test]
    fn emit_works_after_lazy_init() {
        let s = Session::with_config(HitConfig {
            root_path: Some("P:\\emit_test".into()),
            ..HitConfig::default()
        });
        s.emit(Event::LogInfo {
            message: "test".into(),
        });
        assert_eq!(s.receiver().len(), 1);
    }

    #[test]
    fn config_mut_can_modify() {
        let _guard = acquire_env_lock();
        let s = Session::with_defaults();
        s.config_mut().no_junction = true;
        assert!(s.config().no_junction);
    }

    #[test]
    fn config_root_path_overrides_env_root() {
        let cfg = HitConfig {
            root_path: Some("Q:\\custom_hit_root".into()),
            ..HitConfig::default()
        };
        let s = Session::with_config(cfg);
        assert_eq!(s.root_path().to_string_lossy(), "Q:\\custom_hit_root");
        assert_eq!(
            s.apps_path().to_string_lossy(),
            "Q:\\custom_hit_root\\apps"
        );
    }

    #[test]
    fn no_root_path_uses_env_fallback() {
        let _guard = acquire_env_lock();
        let cfg = HitConfig::default(); // root_path = None
        let s = Session::with_config(cfg);
        // 没有指定 root_path，应使用 paths::root_path() 的回退链
        // 至少验证 root 路径以 `.hit` 结尾（默认行为）
        let root = s.root_path().to_string_lossy().to_string();
        assert!(
            root.ends_with(".hit") || root.contains("hit"),
            "root 路径应包含 .hit 或 hit：{root}"
        );
    }
}
