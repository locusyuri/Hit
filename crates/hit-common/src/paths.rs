//! Scoop 兼容路径计算
//!
//! 所有路径默认基于 Hit 根目录（`HIT_ROOT` / `SCOOP` / `~/.hit`），与 Scoop 的 `~/scoop/`
//! 布局保持一致，便于迁移与兼容。
//!
//! 根目录回退链（优先级从高到低，见 [`root_path`]）：
//!   1. `HIT_ROOT` 环境变量
//!   2. `SCOOP` 环境变量（兼容）
//!   3. `USERPROFILE` / `HOME` + `.hit`

use std::path::PathBuf;

/// Hit 根目录。
///
/// 回退链（优先级从高到低）：
///   1. `HIT_ROOT` 环境变量 — 用户显式指定
///   2. `SCOOP` 环境变量 — Scoop 兼容读取
///   3. `USERPROFILE` / `HOME` + `.hit` — 默认路径
///
/// # Panics
/// 若 `HIT_ROOT` / `SCOOP` / `USERPROFILE` / `HOME` 均未设置，panic 并提示用户
/// 设置 `HIT_ROOT` 环境变量。Scoop 的 `root_path()` 在同样情况下会失败。
pub fn root_path() -> PathBuf {
    if let Ok(p) = std::env::var("HIT_ROOT")
        && !p.is_empty()
    {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("SCOOP")
        && !p.is_empty()
    {
        return PathBuf::from(p);
    }
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .expect(
            "无法确定用户目录：请设置 HIT_ROOT 或 USERPROFILE/HOME 环境变量。",
        );
    PathBuf::from(home).join(".hit")
}

/// 下载缓存目录：`<root>/cache/`
pub fn cache_path() -> PathBuf {
    root_path().join("cache")
}

/// 软件安装根目录：`<root>/apps/`
pub fn apps_path() -> PathBuf {
    root_path().join("apps")
}

/// 特定软件的安装目录：`<root>/apps/<app>/`
pub fn app_path(app: &str) -> PathBuf {
    apps_path().join(app)
}

/// 特定软件特定版本的安装目录：`<root>/apps/<app>/<version>/`
pub fn app_version_path(app: &str, version: &str) -> PathBuf {
    app_path(app).join(version)
}

/// `current` junction 路径：`<root>/apps/<app>/current`
pub fn app_current_path(app: &str) -> PathBuf {
    app_path(app).join("current")
}

/// Shim 目录：`<root>/shims/`（需加入 PATH）
pub fn shims_path() -> PathBuf {
    root_path().join("shims")
}

/// 持久化数据目录：`<root>/persist/`
pub fn persist_path() -> PathBuf {
    root_path().join("persist")
}

/// 特定软件的持久化目录：`<root>/persist/<app>/`
pub fn app_persist_path(app: &str) -> PathBuf {
    persist_path().join(app)
}

/// Bucket 仓库目录：`<root>/buckets/`
pub fn buckets_path() -> PathBuf {
    root_path().join("buckets")
}

/// 特定 Bucket 的目录：`<root>/buckets/<name>/`
pub fn bucket_path(name: &str) -> PathBuf {
    buckets_path().join(name)
}

/// 日志目录：`<root>/logs/`
pub fn logs_path() -> PathBuf {
    root_path().join("logs")
}

/// 跨模块 env 锁（仅测试可见）。
///
/// `paths::root_path()` 读取 USERPROFILE/HOME 等进程级环境变量，多个测试并发
/// 修改这些变量会相互干扰。任何会修改 env 或调用 `root_path()` 的测试都应
/// 先持有此锁。`acquire_env_lock()` 会恢复 poisoned 状态（should_panic 测试遗留）。
#[cfg(test)]
pub(crate) static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
pub(crate) fn acquire_env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn save_env() -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        (
            env::var("HIT_ROOT").ok(),
            env::var("SCOOP").ok(),
            env::var("USERPROFILE").ok(),
            env::var("HOME").ok(),
        )
    }

    fn restore_env(saved: (Option<String>, Option<String>, Option<String>, Option<String>)) {
        // SAFETY：测试内通过 ENV_LOCK Mutex 串行化，Windows 下环境变量修改对进程内其它线程
        // 可见但在串行化保护下不会并发读写，符合 Rust 2024 对 env set_var/remove_var 的 unsafe 要求。
        unsafe {
            match saved.0 {
                Some(v) => env::set_var("HIT_ROOT", v),
                None => env::remove_var("HIT_ROOT"),
            }
            match saved.1 {
                Some(v) => env::set_var("SCOOP", v),
                None => env::remove_var("SCOOP"),
            }
            match saved.2 {
                Some(v) => env::set_var("USERPROFILE", v),
                None => env::remove_var("USERPROFILE"),
            }
            match saved.3 {
                Some(v) => env::set_var("HOME", v),
                None => env::remove_var("HOME"),
            }
        }
    }

    #[test]
    fn hit_root_env_wins() {
        let _guard = acquire_env_lock();
        let saved = save_env();
        unsafe {
            env::set_var("HIT_ROOT", "Z:\\hit_root_test");
            env::set_var("SCOOP", "Z:\\should_not_win");
        }
        let p = root_path();
        restore_env(saved);
        assert_eq!(p, PathBuf::from("Z:\\hit_root_test"));
    }

    #[test]
    fn scoop_env_second() {
        let _guard = acquire_env_lock();
        let saved = save_env();
        unsafe {
            env::remove_var("HIT_ROOT");
            env::set_var("SCOOP", "Y:\\scoop_test");
        }
        let p = root_path();
        restore_env(saved);
        assert_eq!(p, PathBuf::from("Y:\\scoop_test"));
    }

    #[test]
    fn userprofile_fallback_appends_hit() {
        let _guard = acquire_env_lock();
        let saved = save_env();
        unsafe {
            env::remove_var("HIT_ROOT");
            env::remove_var("SCOOP");
            env::set_var("USERPROFILE", "X:\\user_test");
            env::remove_var("HOME");
        }
        let p = root_path();
        restore_env(saved);
        assert_eq!(p, PathBuf::from("X:\\user_test").join(".hit"));
    }

    #[test]
    fn home_fallback_when_userprofile_missing() {
        let _guard = acquire_env_lock();
        let saved = save_env();
        unsafe {
            env::remove_var("HIT_ROOT");
            env::remove_var("SCOOP");
            env::remove_var("USERPROFILE");
            env::set_var("HOME", "/home/hit_test");
        }
        let p = root_path();
        restore_env(saved);
        assert_eq!(p, PathBuf::from("/home/hit_test").join(".hit"));
    }

    #[test]
    fn panics_when_no_home_available() {
        let _guard = acquire_env_lock();
        let saved = save_env();
        unsafe {
            env::remove_var("HIT_ROOT");
            env::remove_var("SCOOP");
            env::remove_var("USERPROFILE");
            env::remove_var("HOME");
        }
        // catch_unwind 捕获 panic，保证 env 一定被恢复（避免污染后续测试）
        let result = std::panic::catch_unwind(|| root_path());
        restore_env(saved);
        assert!(result.is_err(), "root_path 应在无 home 时 panic");
    }

    #[test]
    fn empty_env_treated_as_unset() {
        let _guard = acquire_env_lock();
        let saved = save_env();
        unsafe {
            env::set_var("HIT_ROOT", "");
            env::set_var("SCOOP", "");
            env::set_var("USERPROFILE", "W:\\fallback");
            env::remove_var("HOME");
        }
        let p = root_path();
        restore_env(saved);
        // 空字符串视为未设置，应回退到 USERPROFILE/.hit
        assert_eq!(p, PathBuf::from("W:\\fallback").join(".hit"));
    }

    #[test]
    fn derivative_paths_build_on_root() {
        let _guard = acquire_env_lock();
        let saved = save_env();
        unsafe { env::set_var("HIT_ROOT", "R:\\hit") };
        assert_eq!(cache_path(), PathBuf::from("R:\\hit\\cache"));
        assert_eq!(apps_path(), PathBuf::from("R:\\hit\\apps"));
        assert_eq!(shims_path(), PathBuf::from("R:\\hit\\shims"));
        assert_eq!(persist_path(), PathBuf::from("R:\\hit\\persist"));
        assert_eq!(buckets_path(), PathBuf::from("R:\\hit\\buckets"));
        assert_eq!(logs_path(), PathBuf::from("R:\\hit\\logs"));
        assert_eq!(
            app_version_path("git", "2.45.1"),
            PathBuf::from("R:\\hit\\apps\\git\\2.45.1")
        );
        assert_eq!(
            app_persist_path("python"),
            PathBuf::from("R:\\hit\\persist\\python")
        );
        restore_env(saved);
    }
}
