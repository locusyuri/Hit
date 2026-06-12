//! EventBus 事件总线
//!
//! 使用 `flume` 有界 channel（容量 20）实现 hit-core → hit-cli 的双向事件传输。
//! - hit-core 通过 `session.emitter()` 发送事件（下载进度、安装步骤、提示确认等）
//! - hit-cli 通过 `session.event_bus().receiver()` 接收事件并渲染 UI
//! - `Event` 枚举使用 `#[non_exhaustive]` 保证向后兼容扩展
//!
//! 参考：`ref/Hok/crates/libscoop/src/event.rs`

use std::path::PathBuf;

/// 安装/卸载流水线阶段标识
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InstallPhase {
    /// 解析 Manifest
    Resolve,
    /// 下载软件包
    Download,
    /// 校验哈希
    HashVerify,
    /// 解压归档
    Extract,
    /// 提交安装（原子移动）
    Commit,
    /// 同步元数据（db.json / shim / persist）
    Sync,
}

/// Hit 事件总线事件枚举
///
/// 使用 `#[non_exhaustive]` 保证后续扩展不会破坏下游匹配。
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    /// 下载进度（已下载字节、总字节、当前速率 B/s）
    DownloadProgress {
        app: String,
        downloaded: u64,
        total: u64,
        bytes_per_sec: u64,
    },
    /// Bucket 更新进度（已处理 manifest 数 / 总数）
    BucketUpdateProgress {
        bucket: String,
        processed: usize,
        total: usize,
    },
    /// 解压开始
    ExtractStart { app: String, archive: PathBuf },
    /// 安装阶段开始
    InstallPhaseStart { app: String, phase: InstallPhase },
    /// 安装阶段结束
    InstallPhaseEnd { app: String, phase: InstallPhase },
    /// 用户确认提示（阻塞等待 receiver 回复）
    PromptConfirm {
        message: String,
        /// 回复通道（true=确认 / false=取消）
        reply: flume::Sender<bool>,
    },
    /// 普通日志消息
    LogInfo { message: String },
    /// 警告消息
    LogWarn { message: String },
}

/// EventBus 内部发送端
pub type Sender = flume::Sender<Event>;
/// EventBus 内部接收端
pub type Receiver = flume::Receiver<Event>;

/// EventBus 默认 channel 容量
pub const DEFAULT_CAPACITY: usize = 20;

/// 事件总线（持有 flume channel 的两端）
pub struct EventBus {
    sender: Sender,
    receiver: Receiver,
}

impl EventBus {
    /// 创建有界 channel（默认容量 `DEFAULT_CAPACITY`）
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// 创建指定容量的有界 channel
    pub fn with_capacity(cap: usize) -> Self {
        let (sender, receiver) = flume::bounded(cap);
        Self { sender, receiver }
    }

    /// 获取发送端（用于 Session 内部转发）
    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    /// 获取接收端（用于 CLI 订阅事件）
    pub fn receiver(&self) -> &Receiver {
        &self.receiver
    }

    /// 发送事件；channel 已满时静默丢弃（避免阻塞核心操作）
    pub fn emit(&self, event: Event) {
        let _ = self.sender.try_send(event);
    }

    /// 便捷发送器构造：克隆 Sender 给 hit-core 内部使用
    pub fn emitter(&self) -> Sender {
        self.sender.clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("capacity", &self.sender.capacity())
            .field("len", &self.sender.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_and_receive() {
        let bus = EventBus::new();
        bus.emit(Event::LogInfo {
            message: "hello".into(),
        });
        let received = bus.receiver().try_recv().unwrap();
        match received {
            Event::LogInfo { message } => assert_eq!(message, "hello"),
            _ => panic!("期望 LogInfo 事件"),
        }
    }

    #[test]
    fn drop_when_full() {
        let bus = EventBus::with_capacity(2);
        bus.emit(Event::LogInfo {
            message: "a".into(),
        });
        bus.emit(Event::LogInfo {
            message: "b".into(),
        });
        bus.emit(Event::LogInfo {
            message: "c".into(),
        }); // 应被静默丢弃
        assert_eq!(bus.receiver().len(), 2);
    }

    #[test]
    fn emitter_is_clonable() {
        let bus = EventBus::new();
        let emitter = bus.emitter();
        emitter
            .send(Event::LogWarn {
                message: "warn".into(),
            })
            .unwrap();
        assert_eq!(bus.receiver().len(), 1);
    }
}
