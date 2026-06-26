//! EventBus 事件流测试
//!
//! 验证安装流程中事件按正确顺序发送。

use hit_common::event::{Event, InstallPhase};
use hit_common::config::HitConfig;
use hit_common::Session;
use hit_core::install::{install, InstallOptions};
use hit_core::manifest::{License, Manifest};
use hit_test_utils::temp_scoop_root;

/// 创建测试 session
fn test_session(root: &std::path::Path) -> Session {
    let config = HitConfig {
        root_path: Some(root.to_string_lossy().into()),
        ..Default::default()
    };
    Session::with_config(config)
}

/// 极简 manifest
fn minimal_manifest(version: &str) -> Manifest {
    Manifest {
        version: version.to_string(),
        description: "test app".into(),
        homepage: "https://example.com".into(),
        license: License::Identifier("MIT".into()),
        ..Default::default()
    }
}

/// 从 receiver 收集所有事件
fn collect_events(session: &Session) -> Vec<Event> {
    let receiver = session.event_bus().receiver();
    let mut events = Vec::new();
    while let Ok(event) = receiver.try_recv() {
        events.push(event);
    }
    events
}

#[test]
fn install_emits_resolve_start_and_end() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    // 提前初始化 event_bus
    let _bus = session.event_bus();

    let manifest = minimal_manifest("1.0");
    let options = InstallOptions::default();
    let _ = install(&session, "testapp", &manifest, "main", &options);

    let events = collect_events(&session);

    // 验证包含 Resolve 阶段的 start 和 end 事件
    let has_resolve_start = events.iter().any(|e| {
        matches!(
            e,
            Event::InstallPhaseStart {
                phase: InstallPhase::Resolve,
                ..
            }
        )
    });
    let has_resolve_end = events.iter().any(|e| {
        matches!(
            e,
            Event::InstallPhaseEnd {
                phase: InstallPhase::Resolve,
                ..
            }
        )
    });

    assert!(has_resolve_start, "应发出 InstallPhaseStart(Resolve)");
    assert!(has_resolve_end, "应发出 InstallPhaseEnd(Resolve)");
}

#[test]
fn install_emits_download_events() {
    let (_dir, root) = temp_scoop_root().unwrap();
    let session = test_session(&root);

    let _bus = session.event_bus();

    let manifest = minimal_manifest("1.0");
    let options = InstallOptions::default();
    let _ = install(&session, "testapp", &manifest, "main", &options);

    let events = collect_events(&session);

    // 应包含 Download 阶段事件
    let has_download_start = events.iter().any(|e| {
        matches!(
            e,
            Event::InstallPhaseStart {
                phase: InstallPhase::Download,
                ..
            }
        )
    });

    assert!(has_download_start, "应发出 InstallPhaseStart(Download)");
}

#[test]
fn log_events_are_received() {
    let dir = tempfile::tempdir().unwrap();
    let config = HitConfig {
        root_path: Some(dir.path().to_string_lossy().into()),
        ..Default::default()
    };
    let session = Session::with_config(config);

    session.emit(Event::LogInfo {
        message: "test info".into(),
    });
    session.emit(Event::LogWarn {
        message: "test warn".into(),
    });

    let events = collect_events(&session);
    assert_eq!(events.len(), 2);

    assert!(matches!(&events[0], Event::LogInfo { message } if message == "test info"));
    assert!(matches!(&events[1], Event::LogWarn { message } if message == "test warn"));
}

#[test]
fn prompt_confirm_event_has_reply_channel() {
    use flume;

    let dir = tempfile::tempdir().unwrap();
    let config = HitConfig {
        root_path: Some(dir.path().to_string_lossy().into()),
        ..Default::default()
    };
    let session = Session::with_config(config);

    let (reply_tx, reply_rx) = flume::bounded(1);

    session.emit(Event::PromptConfirm {
        message: "确认？".into(),
        reply: reply_tx,
    });

    let events = collect_events(&session);
    assert_eq!(events.len(), 1);

    // 发送回复
    if let Event::PromptConfirm { reply, .. } = &events[0] {
        reply.send(true).unwrap();
    }

    // 验证回复通过 channel 传递
    assert!(reply_rx.recv().unwrap());
}
