//! Git 仓库克隆
//!
//! 使用 `gix`（纯 Rust）实现 bucket 仓库的克隆操作：
//! - 默认浅克隆（depth=1）以节省带宽与磁盘空间
//! - 支持通过 `session.config().proxy` 配置 HTTP 代理
//! - 进度通过 EventBus `LogInfo` 事件上报

use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use gix::remote::fetch::Shallow;
use hit_common::error::{HitError, Result};
use hit_common::event::Event;
use hit_common::Session;

/// 克隆选项
#[derive(Debug, Clone, Default)]
pub struct CloneOptions {
    /// 完整克隆（默认 false = 浅克隆 depth=1）
    pub full_clone: bool,
}

/// RAII 代理守卫：设置 `HTTP_PROXY`/`HTTPS_PROXY`，Drop 时恢复原值
///
/// 无协议前缀时自动补 `http://`（与 Scoop/Hok 行为一致）。
/// 注意：env var 是进程全局状态，并发克隆会竞争；当前 bucket 操作为顺序执行。
struct ProxyGuard {
    original_http: Option<String>,
    original_https: Option<String>,
}

impl ProxyGuard {
    /// 激活代理环境变量。`proxy` 为 None 时返回空守卫（不做任何修改）。
    fn activate(proxy: Option<&str>) -> Self {
        let original_http = std::env::var("HTTP_PROXY").ok();
        let original_https = std::env::var("HTTPS_PROXY").ok();

        if let Some(proxy_url) = proxy {
            let normalized = if proxy_url.contains("://") {
                proxy_url.to_string()
            } else {
                format!("http://{proxy_url}")
            };
            // SAFETY: bucket 操作为顺序执行，不存在并发竞争
            unsafe {
                std::env::set_var("HTTP_PROXY", &normalized);
                std::env::set_var("HTTPS_PROXY", &normalized);
            }
        }

        Self {
            original_http,
            original_https,
        }
    }
}

impl Drop for ProxyGuard {
    fn drop(&mut self) {
        // SAFETY: bucket 操作为顺序执行，不存在并发竞争
        unsafe {
            match &self.original_http {
                Some(val) => std::env::set_var("HTTP_PROXY", val),
                None => std::env::remove_var("HTTP_PROXY"),
            }
            match &self.original_https {
                Some(val) => std::env::set_var("HTTPS_PROXY", val),
                None => std::env::remove_var("HTTPS_PROXY"),
            }
        }
    }
}

/// 克隆 Bucket Git 仓库
///
/// - 默认浅克隆（depth=1），节省带宽与磁盘
/// - 代理从 `session.config().proxy` 读取
/// - 进度通过 EventBus `LogInfo` 事件上报
///
/// # 参数
/// - `session`：会话上下文（配置 / 事件 / 路径）
/// - `name`：bucket 名称（目录名 + 事件标识）
/// - `url`：Git 仓库 URL
/// - `opts`：克隆选项
/// - `should_interrupt`：中断信号（调用方绑定 Ctrl+C）
///
/// # 返回
/// 本地仓库路径 `<buckets_path>/<name>/`
pub fn clone_bucket(
    session: &Session,
    name: &str,
    url: &str,
    opts: &CloneOptions,
    should_interrupt: &AtomicBool,
) -> Result<PathBuf> {
    let target = session.buckets_path().join(name);

    // 前置检查：目标目录已存在且非空
    if target.exists() && target.read_dir().map_err(|e| HitError::io("读取 bucket 目录", e))?.next().is_some() {
        return Err(HitError::Bucket {
            bucket: name.into(),
            message: "目录已存在且非空".into(),
        });
    }

    // 确保 buckets 父目录存在
    std::fs::create_dir_all(session.buckets_path())
        .map_err(|e| HitError::io("创建 bucket 目录", e))?;

    // 激活代理环境变量（Drop 时自动恢复）
    let _proxy_guard = ProxyGuard::activate(session.config().proxy.as_deref());

    session.emit(Event::LogInfo {
        message: format!("正在克隆 bucket '{name}'..."),
    });

    let mut prepare = gix::prepare_clone(url, &target)
        .map_err(|e| gix_clone_err(name, e))?;

    if !opts.full_clone {
        prepare = prepare.with_shallow(Shallow::DepthAtRemote(NonZeroU32::new(1).unwrap()));
    }

    let (mut checkout, _fetch_outcome) = prepare
        .fetch_then_checkout(gix::progress::Discard, should_interrupt)
        .map_err(|e| gix_clone_err(name, e))?;

    session.emit(Event::LogInfo {
        message: format!("bucket '{name}'：检出文件中..."),
    });

    let (_repo, _checkout_outcome) = checkout
        .main_worktree(gix::progress::Discard, should_interrupt)
        .map_err(|e| gix_clone_err(name, e))?;

    session.emit(Event::LogInfo {
        message: format!("bucket '{name}' 克隆完成"),
    });

    Ok(target)
}

/// 更新已有 Bucket 仓库
///
/// 读取远程 URL 后重新浅克隆（覆盖目录）。Bucket 仓库无本地修改需求，
/// 此策略简单可靠，浅克隆带宽开销可控。
///
/// # 参数
/// - `session`：会话上下文
/// - `name`：bucket 名称
/// - `should_interrupt`：中断信号
///
/// # 返回
/// 本地仓库路径
pub fn pull_bucket(
    session: &Session,
    name: &str,
    should_interrupt: &AtomicBool,
) -> Result<PathBuf> {
    let target = session.buckets_path().join(name);

    // 前置检查：目标目录必须存在
    if !target.exists() {
        return Err(HitError::Bucket {
            bucket: name.into(),
            message: "bucket 目录不存在".into(),
        });
    }

    // 读取 remote URL（必须是 git 仓库）
    let remote_url = {
        let repo = match gix::open(&target) {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(
                    "bucket '{}' 不是有效 git 仓库，重新克隆: {}",
                    name,
                    e
                );
                return clone_bucket_recreate(session, name, &target, should_interrupt);
            }
        };
        let remote = repo.find_remote("origin").map_err(|e| HitError::Bucket {
            bucket: name.into(),
            message: format!("无法读取 origin：{e}"),
        })?;
        remote
            .url(gix::remote::Direction::Fetch)
            .map(|url| url.to_bstring().to_string())
            .ok_or_else(|| HitError::Bucket {
                bucket: name.into(),
                message: "origin 无 fetch URL".into(),
            })?
    };

    session.emit(Event::LogInfo {
        message: format!("正在更新 bucket '{name}'..."),
    });

    // 删除现有目录后重新浅克隆
    match std::fs::remove_dir_all(&target) {
        Ok(()) => {}
        Err(e) => {
            tracing::warn!("删除旧 bucket 目录失败，尝试强制删除: {}", e);
            force_remove_dir_all(&target)?;
        }
    }

    clone_bucket(
        session,
        name,
        &remote_url,
        &CloneOptions::default(),
        should_interrupt,
    )
}

fn clone_bucket_recreate(
    session: &Session,
    name: &str,
    target: &PathBuf,
    should_interrupt: &AtomicBool,
) -> Result<PathBuf> {
    let known_url = crate::bucket::resolve_known_bucket(name);
    let url = match known_url {
        Some(u) => u.to_string(),
        None => {
            return Err(HitError::Bucket {
                bucket: name.into(),
                message: "非 git 仓库且未知 bucket，无法重新克隆".into(),
            });
        }
    };

    session.emit(Event::LogInfo {
        message: format!("正在更新 bucket '{name}'..."),
    });

    force_remove_dir_all(target)?;

    clone_bucket(session, name, &url, &CloneOptions::default(), should_interrupt)
}

fn force_remove_dir_all(path: &PathBuf) -> Result<()> {
    let path_str = path.to_str().unwrap_or_default();
    
    let success = std::process::Command::new("cmd.exe")
        .args(["/C", "rmdir", "/S", "/Q", path_str])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !success {
        return Err(HitError::io("强制删除目录", std::io::Error::last_os_error()));
    }

    Ok(())
}

/// 将 gix 克隆错误转换为 `HitError::Bucket`
fn gix_clone_err(name: &str, e: impl std::fmt::Display) -> HitError {
    HitError::Bucket {
        bucket: name.into(),
        message: format!("克隆失败：{e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn clone_options_default_is_shallow() {
        let opts = CloneOptions::default();
        assert!(!opts.full_clone);
    }

    #[test]
    fn proxy_guard_sets_and_restores_env() {
        // 合并测试以避免并行测试间 env var 竞争

        // 1. 设置带协议前缀的代理
        let before_http = std::env::var("HTTP_PROXY").ok();
        let before_https = std::env::var("HTTPS_PROXY").ok();

        {
            let _guard = ProxyGuard::activate(Some("http://127.0.0.1:7890"));
            assert_eq!(std::env::var("HTTP_PROXY").unwrap(), "http://127.0.0.1:7890");
            assert_eq!(std::env::var("HTTPS_PROXY").unwrap(), "http://127.0.0.1:7890");
        }

        assert_eq!(std::env::var("HTTP_PROXY").ok(), before_http);
        assert_eq!(std::env::var("HTTPS_PROXY").ok(), before_https);

        // 2. 无协议前缀时自动补 http://
        {
            let _guard = ProxyGuard::activate(Some("example.com:8080"));
            assert_eq!(std::env::var("HTTP_PROXY").unwrap(), "http://example.com:8080");
            assert_eq!(std::env::var("HTTPS_PROXY").unwrap(), "http://example.com:8080");
        }

        assert_eq!(std::env::var("HTTP_PROXY").ok(), before_http);
        assert_eq!(std::env::var("HTTPS_PROXY").ok(), before_https);

        // 3. None 不修改环境变量
        {
            let _guard = ProxyGuard::activate(None);
            assert_eq!(std::env::var("HTTP_PROXY").ok(), before_http);
            assert_eq!(std::env::var("HTTPS_PROXY").ok(), before_https);
        }
    }

    #[test]
    fn clone_rejects_existing_nonempty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("test-bucket");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        std::fs::write(bucket_dir.join("file.txt"), "content").unwrap();

        let config = hit_common::config::HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let interrupt = AtomicBool::new(false);

        let result = clone_bucket(
            &session,
            "test-bucket",
            "https://example.com/repo.git",
            &CloneOptions::default(),
            &interrupt,
        );

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("已存在"), "错误消息应提及目录已存在：{err_msg}");
    }

    #[test]
    #[ignore = "需要网络访问"]
    fn clone_shallow_public_repo() {
        let dir = tempfile::tempdir().unwrap();
        let config = hit_common::config::HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let interrupt = AtomicBool::new(false);

        let result = clone_bucket(
            &session,
            "test-bucket",
            "https://github.com/ScoopInstaller/Main.git",
            &CloneOptions::default(),
            &interrupt,
        );

        let path = result.expect("浅克隆应成功");
        assert!(path.join(".git").exists(), "应包含 .git 目录");
        assert!(path.join(".git/shallow").exists(), "浅克隆应有 .git/shallow 文件");
    }

    #[test]
    #[ignore = "需要网络访问"]
    fn clone_full_public_repo() {
        let dir = tempfile::tempdir().unwrap();
        let config = hit_common::config::HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let interrupt = AtomicBool::new(false);

        let result = clone_bucket(
            &session,
            "test-bucket",
            "https://github.com/ScoopInstaller/Main.git",
            &CloneOptions { full_clone: true },
            &interrupt,
        );

        let path = result.expect("完整克隆应成功");
        assert!(path.join(".git").exists(), "应包含 .git 目录");
        assert!(!path.join(".git/shallow").exists(), "完整克隆不应有 .git/shallow 文件");
    }

    #[test]
    fn pull_nonexistent_bucket_errors() {
        let dir = tempfile::tempdir().unwrap();
        let config = hit_common::config::HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let interrupt = AtomicBool::new(false);

        let result = pull_bucket(&session, "nonexistent", &interrupt);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("不存在"), "错误应提及目录不存在：{msg}");
    }

    #[test]
    fn pull_non_git_dir_errors() {
        let dir = tempfile::tempdir().unwrap();
        let bucket_dir = dir.path().join("buckets").join("not-git");
        std::fs::create_dir_all(&bucket_dir).unwrap();
        std::fs::write(bucket_dir.join("file.txt"), "content").unwrap();

        let config = hit_common::config::HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let interrupt = AtomicBool::new(false);

        let result = pull_bucket(&session, "not-git", &interrupt);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("git 仓库") || msg.contains("git"),
            "错误应提及 git 仓库：{msg}"
        );
    }

    #[test]
    #[ignore = "需要网络访问"]
    fn pull_updates_shallow_repo() {
        let dir = tempfile::tempdir().unwrap();
        let config = hit_common::config::HitConfig {
            root_path: Some(dir.path().to_string_lossy().into()),
            ..Default::default()
        };
        let session = Session::with_config(config);
        let interrupt = AtomicBool::new(false);

        clone_bucket(
            &session,
            "main",
            "https://github.com/ScoopInstaller/Main.git",
            &CloneOptions::default(),
            &interrupt,
        )
        .expect("克隆应成功");

        let path = pull_bucket(&session, "main", &interrupt).expect("pull 应成功");
        assert!(path.join(".git").exists());
        assert!(path.join(".git/shallow").exists(), "pull 后仍应为浅克隆");
    }
}
