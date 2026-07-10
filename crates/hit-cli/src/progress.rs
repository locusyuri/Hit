use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::time::Duration;

use flume::Receiver;
use hit_common::event::{Event, InstallPhase};
use hit_common::Session;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rusty_rich::{Console, Text};

pub struct ProgressRenderer {
    handle: Option<std::thread::JoinHandle<()>>,
    stop_tx: flume::Sender<()>,
}

impl ProgressRenderer {
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

fn render_loop(receiver: Receiver<Event>, stop_rx: Receiver<()>) {
    let multi = MultiProgress::new();
    let mut bars: HashMap<String, ProgressBar> = HashMap::new();

    loop {
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
            let mut console = Console::new();
            console.println(&Text::from_markup(&format!(
                "[cyan]解压[/cyan] {} ({})",
                app, name
            )));
        }

        Event::InstallPhaseStart { app, phase } => {
            let label = phase_label(phase);
            let mut console = Console::new();
            console.println(&Text::from_markup(&format!(
                "[blue]▶[/blue] [{}] {}...",
                label, app
            )));
        }

        Event::InstallPhaseEnd { app, phase } => {
            let label = phase_label(phase);
            let mut console = Console::new();
            console.println(&Text::from_markup(&format!(
                "[green]✔[/green] [{}] {} 完成",
                label, app
            )));
        }

        Event::PromptConfirm { message, reply } => {
            let confirmed = prompt_user(&message);
            let _ = reply.send(confirmed);
        }

        Event::LogInfo { message } => {
            println!("{message}");
        }

        Event::LogWarn { message } => {
            let mut console = Console::new();
            console.println(&Text::from_markup(&format!(
                "[yellow][WARN][/yellow] {}",
                message
            )));
        }

        _ => {}
    }
}

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

fn prompt_user(message: &str) -> bool {
    eprint!("{} {message} [y/N] ", Text::from_markup("[yellow]?[/yellow]"));
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
        let _ = session.event_bus();
        let renderer = ProgressRenderer::start(&session);
        renderer.stop();
    }
}
