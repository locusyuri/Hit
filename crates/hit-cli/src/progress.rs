//! EventBus 进度渲染器
//!
//! 后台线程订阅 `Session` 的 EventBus 事件，使用 `indicatif` 渲染进度条、
//! `colored` 渲染彩色日志，并处理 `PromptConfirm` 的交互式确认。

use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::time::Duration;

use colored::Colorize;
use flume::Receiver;
use hit_common::event::{Event, InstallPhase};
use hit_common::Session;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// 进度渲染器（后台线程）
pub struct ProgressRenderer {
    handle: Option<std::thread::JoinHandle<()>>,
    stop_tx: flume::Sender<()>,
}

impl ProgressRenderer {
    /// 启动后台渲染线程，订阅 Session 的 EventBus
    pub fn start(session: &Session) -> Self {
        let receiver = session.receiver().clone();
        let (stop_tx, stop_rx) = flume::bounded(1);

        let handle = std::thread::spawn(move || {
            render_loop(receiver, stop_rx);
        });

        Self {
            handle: Some(handle),
            stop_tx,
        }
    }

    /// 停止渲染线程并等待退出
    pub fn stop(mut self) {
        let _ = self.stop_tx.send(());
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for ProgressRenderer {
    fn drop(&mut self) {
        let _ = self.stop_tx.send(());
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// 后台渲染主循环
fn render_loop(receiver: Receiver<Event>, stop_rx: Receiver<()>) {
    let multi = MultiProgress::new();
    let mut bars: HashMap<String, ProgressBar> = HashMap::new();

    loop {
        // 检查停止信号
        if stop_rx.try_recv().is_ok() {
            break;
        }

        match receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(event) => handle_event(event, &multi, &mut bars),
            Err(flume::RecvTimeoutError::Timeout) => continue,
            Err(flume::RecvTimeoutError::Disconnected) => break,
        }
    }
}

/// 处理单个事件
fn handle_event(
    event: Event,
    multi: &MultiProgress,
    bars: &mut HashMap<String, ProgressBar>,
) {
    match event {
        Event::DownloadProgress {
            app,
            downloaded,
            total,
            bytes_per_sec,
        } => {
            let bar = bars.entry(app.clone()).or_insert_with(|| {
                let pb = multi.add(ProgressBar::new(total));
                pb.set_style(
                    ProgressStyle::with_template(
                        "{prefix:.bold} [{bar:30.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec}",
                    )
                    .unwrap()
                    .progress_chars("=> "),
                );
                pb.set_prefix(app.clone());
                pb
            });
            bar.set_length(total);
            bar.set_position(downloaded);
            if bytes_per_sec > 0 {
                bar.set_message(format!("{}/s", format_bytes(bytes_per_sec)));
            }
            if downloaded >= total && total > 0 {
                bar.finish_and_clear();
                bars.remove(&app);
            }
        }

        Event::BucketUpdateProgress {
            bucket,
            processed,
            total,
        } => {
            let bar = bars.entry(format!("bucket:{bucket}")).or_insert_with(|| {
                let pb = multi.add(ProgressBar::new(total as u64));
                pb.set_style(
                    ProgressStyle::with_template(
                        "{prefix:.bold} [{bar:30.green/white}] {pos}/{len}",
                    )
                    .unwrap()
                    .progress_chars("=> "),
                );
                pb.set_prefix(bucket.clone());
                pb
            });
            bar.set_length(total as u64);
            bar.set_position(processed as u64);
            if processed >= total {
                bar.finish_and_clear();
                bars.remove(&format!("bucket:{bucket}"));
            }
        }

        Event::ExtractStart { app, archive } => {
            let name = archive
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "archive".into());
            println!("{} {} ({})", "解压".cyan(), app, name);
        }

        Event::InstallPhaseStart { app, phase } => {
            let label = phase_label(phase);
            println!("{} [{}] {}...", "▶".blue(), label, app);
        }

        Event::InstallPhaseEnd { app, phase } => {
            let label = phase_label(phase);
            println!("{} [{}] {} 完成", "✔".green(), label, app);
        }

        Event::PromptConfirm { message, reply } => {
            let confirmed = prompt_user(&message);
            let _ = reply.send(confirmed);
        }

        Event::LogInfo { message } => {
            println!("{message}");
        }

        Event::LogWarn { message } => {
            println!("{} {message}", "[WARN]".yellow());
        }

        // #[non_exhaustive] 兜底
        _ => {}
    }
}

/// 安装阶段中文标签
fn phase_label(phase: InstallPhase) -> &'static str {
    match phase {
        InstallPhase::Resolve => "解析",
        InstallPhase::Download => "下载",
        InstallPhase::HashVerify => "校验",
        InstallPhase::Extract => "解压",
        InstallPhase::Commit => "提交",
        InstallPhase::Sync => "同步",
        _ => "未知",
    }
}

/// 字节数格式化（简易版）
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// 交互式确认提示（阻塞直到用户输入 y/n）
fn prompt_user(message: &str) -> bool {
    eprint!("{} {message} [y/N] ", "?".yellow());
    let _ = io::stderr().flush();

    let stdin = io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).is_err() {
        return false;
    }

    matches!(line.trim().to_lowercase().as_str(), "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_units() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1_500_000), "1.4 MB");
        assert_eq!(format_bytes(2_000_000_000), "1.9 GB");
    }

    #[test]
    fn phase_labels_cover_all_variants() {
        assert_eq!(phase_label(InstallPhase::Resolve), "解析");
        assert_eq!(phase_label(InstallPhase::Download), "下载");
        assert_eq!(phase_label(InstallPhase::HashVerify), "校验");
        assert_eq!(phase_label(InstallPhase::Extract), "解压");
        assert_eq!(phase_label(InstallPhase::Commit), "提交");
        assert_eq!(phase_label(InstallPhase::Sync), "同步");
    }

    #[test]
    fn renderer_start_stop() {
        use hit_common::Session;

        let session = Session::with_defaults();
        // 触发 EventBus 初始化
        let _ = session.event_bus();
        let renderer = ProgressRenderer::start(&session);
        renderer.stop();
    }

    #[test]
    fn renderer_handles_log_events() {
        use hit_common::event::{Event, EventBus};

        let bus = EventBus::new();
        bus.emit(Event::LogInfo {
            message: "test info".into(),
        });
        bus.emit(Event::LogWarn {
            message: "test warn".into(),
        });

        // 直接测试 handle_event 不 panic
        let multi = MultiProgress::new();
        let mut bars = HashMap::new();
        while let Ok(event) = bus.receiver().try_recv() {
            handle_event(event, &multi, &mut bars);
        }
    }
}
