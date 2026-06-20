//! HTTP 下载器
//!
//! 基于 reqwest blocking client 实现，支持：
//! - 从 Session config 读取 proxy 配置
//! - 流式读取响应体，100ms 节流发送 DownloadProgress 事件
//! - 先写入 `.download` 临时文件，完成后 rename 到目标路径
//! - AtomicBool 中断检查

use std::fs;
use std::io::{BufWriter, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use hit_common::error::{HitError, Result};
use hit_common::event::Event;
use hit_common::Session;
use tracing::warn;

/// 下载缓冲区大小（8 KB）
const BUF_SIZE: usize = 8192;

/// 进度事件节流间隔（毫秒）
const THROTTLE_MS: u128 = 100;

/// 构造 reqwest blocking client（带 proxy 配置）
///
/// 从 `session.config().proxy` 读取代理设置：
/// - `None` 或空字符串：不使用代理
/// - `"none"`（不区分大小写）：显式禁用代理
/// - 其他：作为 proxy URL 配置到 reqwest
///
/// proxy 格式无效时 `warn` 并跳过（不阻断下载）。
pub fn build_client(session: &Session) -> Result<reqwest::blocking::Client> {
    let mut builder = reqwest::blocking::Client::builder();

    let config = session.config();
    let proxy_str = config
        .proxy
        .as_deref()
        .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"));

    if let Some(proxy_url) = proxy_str {
        match reqwest::Proxy::all(proxy_url) {
            Ok(proxy) => {
                builder = builder.proxy(proxy);
            }
            Err(e) => {
                warn!("无效的 proxy 配置 '{}': {}，将跳过代理", proxy_url, e);
            }
        }
    }

    builder.build().map_err(|e| HitError::Download {
        url: String::new(),
        message: format!("构造 HTTP client 失败: {}", e),
    })
}

/// 下载单个文件到指定路径
///
/// - 从 Session config 读取 proxy 配置
/// - 流式读取响应体，每 100ms 发送一次 DownloadProgress 事件
/// - 先写入 `.download` 临时文件，完成后 rename 到目标路径
/// - 支持中断检查（should_interrupt）
/// - 返回下载字节数
pub fn download_file(
    session: &Session,
    url: &str,
    app: &str,
    target: &Path,
    should_interrupt: &AtomicBool,
) -> Result<u64> {
    if url.is_empty() {
        return Err(HitError::Download {
            url: String::new(),
            message: "下载 URL 不能为空".to_string(),
        });
    }

    let temp_path = target.with_extension("download");
    match do_download(session, url, app, target, &temp_path, should_interrupt) {
        Ok(bytes) => Ok(bytes),
        Err(e) => {
            let _ = fs::remove_file(&temp_path);
            Err(e)
        }
    }
}

/// 下载内部实现
fn do_download(
    session: &Session,
    url: &str,
    app: &str,
    target: &Path,
    temp_path: &Path,
    should_interrupt: &AtomicBool,
) -> Result<u64> {
    let client = build_client(session)?;

    let mut response = client.get(url).send().map_err(|e| HitError::Download {
        url: url.to_string(),
        message: format!("请求失败: {}", e),
    })?;

    if !response.status().is_success() {
        return Err(HitError::Download {
            url: url.to_string(),
            message: format!("HTTP 错误: {}", response.status()),
        });
    }

    let total = response.content_length().unwrap_or(0);

    // 创建父目录
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| HitError::Download {
            url: url.to_string(),
            message: format!("创建目录失败: {}", e),
        })?;
    }

    // 打开临时文件
    let file = fs::File::create(temp_path).map_err(|e| HitError::Download {
        url: url.to_string(),
        message: format!("创建临时文件失败: {}", e),
    })?;
    let mut writer = BufWriter::new(file);

    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; BUF_SIZE];
    let mut last_emit = Instant::now();
    let mut last_bytes: u64 = 0;
    let mut last_bps: u64 = 0;

    loop {
        if should_interrupt.load(Ordering::Relaxed) {
            return Err(HitError::Download {
                url: url.to_string(),
                message: "下载被中断".to_string(),
            });
        }

        let bytes_read = response.read(&mut buffer).map_err(|e| HitError::Download {
            url: url.to_string(),
            message: format!("读取响应失败: {}", e),
        })?;

        if bytes_read == 0 {
            break;
        }

        writer
            .write_all(&buffer[..bytes_read])
            .map_err(|e| HitError::Download {
                url: url.to_string(),
                message: format!("写入文件失败: {}", e),
            })?;

        downloaded += bytes_read as u64;

        // 节流进度上报
        if last_emit.elapsed().as_millis() >= THROTTLE_MS {
            let elapsed = last_emit.elapsed().as_secs_f64();
            let bytes_delta = downloaded - last_bytes;
            last_bps = (bytes_delta as f64 / elapsed) as u64;

            session.emit(Event::DownloadProgress {
                app: app.to_string(),
                downloaded,
                total,
                bytes_per_sec: last_bps,
            });

            last_emit = Instant::now();
            last_bytes = downloaded;
        }
    }

    writer.flush().map_err(|e| HitError::Download {
        url: url.to_string(),
        message: format!("刷新文件缓冲失败: {}", e),
    })?;

    // 最终进度上报
    session.emit(Event::DownloadProgress {
        app: app.to_string(),
        downloaded,
        total: downloaded,
        bytes_per_sec: last_bps,
    });

    // 原子重命名
    fs::rename(temp_path, target).map_err(|e| HitError::Download {
        url: url.to_string(),
        message: format!("重命名临时文件失败: {}", e),
    })?;

    Ok(downloaded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::HitConfig;
    use std::sync::atomic::AtomicBool;

    fn make_session(proxy: Option<&str>) -> Session {
        Session::with_config(HitConfig {
            proxy: proxy.map(|s| s.to_string()),
            ..HitConfig::default()
        })
    }

    #[test]
    fn build_client_no_proxy() {
        let session = make_session(None);
        assert!(build_client(&session).is_ok());
    }

    #[test]
    fn build_client_with_proxy() {
        let session = make_session(Some("http://127.0.0.1:8080"));
        assert!(build_client(&session).is_ok());
    }

    #[test]
    fn build_client_proxy_none_string() {
        let session = make_session(Some("none"));
        assert!(build_client(&session).is_ok());
    }

    #[test]
    fn build_client_proxy_none_uppercase() {
        let session = make_session(Some("NONE"));
        assert!(build_client(&session).is_ok());
    }

    #[test]
    fn build_client_invalid_proxy_warns() {
        let session = make_session(Some("://invalid"));
        // 无效 proxy 应 warn 但不报错
        assert!(build_client(&session).is_ok());
    }

    #[test]
    fn download_file_rejects_empty_url() {
        let session = make_session(None);
        let target = std::env::temp_dir().join("hit_test_empty_url.bin");
        let should_interrupt = AtomicBool::new(false);

        let result = download_file(&session, "", "test-app", &target, &should_interrupt);
        assert!(result.is_err());

        let _ = fs::remove_file(&target);
        let _ = fs::remove_file(target.with_extension("download"));
    }
}
