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
}
