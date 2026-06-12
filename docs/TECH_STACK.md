# 技术栈清单

## 核心依赖总览

| 模块 | 依赖 | 用途 | 评估 |
|------|------|------|------|
| CLI | clap | 参数解析 | ✅ 最佳选择，Rust CLI 事实标准 |
| CLI | indicatif | 进度条 | ✅ 最佳选择 |
| CLI | colored | 彩色输出 | ✅ 合理选择，简单成熟 |
| CLI | **ratatui** | **TUI 交互界面** | ✅ 替换 dialoguer，支持 `si` 交互表格 |
| Core | serde + **sonic-rs** | JSON 序列化 | ⚡ 升级：sonic-rs 比 serde_json **快 1.5~2x** |
| Core | thiserror | 错误处理 | ✅ 最佳选择 |
| Core | petgraph | 依赖图 | ✅ 最佳选择 |
| Bucket | git2 | Git 仓库操作 | ✅ 短期稳妥；长期关注纯 Rust 的 **gix** |
| Downloader | reqwest | HTTP 下载 | ✅ 功能最全，async/blocking 双模式 |
| Downloader | blake3/sha2 | 哈希计算 | ✅ 合理选择 |
| Compression | zip/sevenz-rust/tar | 压缩解压 | ✅ 合理选择 |
| Store | **纯 JSON 文件** | 数据存储 | ♻️ 暂去 rusqlite，当前 JSON 文件足够 |
| Windows | windows | Windows API | ✅ 官方 SDK 绑定 |
| Windows | winreg | 注册表操作 | ✅ 合理选择 |

---

## 变更说明

### ⚡ JSON：serde_json → sonic-rs

**sonic-rs**（字节跳动 CloudWeGo）是 serde 兼容的高性能 JSON 库，利用 SIMD 加速。

| 场景 | serde_json | sonic-rs | 提升 |
|------|:-:|:-:|:-:|
| 非结构化序列化（twitter） | 797µs | **390µs** | **2.0x** |
| 结构体序列化（twitter） | 740µs | **448µs** | **1.6x** |
| 非结构化序列化（citm） | 1.88ms | **822µs** | **2.3x** |

**迁移成本**：极低。sonic-rs 完全兼容 serde，多数情况下只需改 import：
```rust
// 之前
use serde_json;
// 之后
use sonic_rs;
```

Hit 需要频繁解析 Manifest JSON（搜索时遍历数百个清单文件），这一替换收益明显。

### 🖥️ TUI：dialoguer → ratatui

**ratatui** 是 Rust TUI 的事实标准（Benchmark Score 86），比 dialoguer 更强大：

| 对比 | dialoguer | ratatui |
|------|-----------|---------|
| 定位 | 简单交互提示 | **完整 TUI 框架** |
| 表格 | ❌ 不支持 | ✅ 表格/列表/多列 |
| 快捷键 | ❌ 有限 | ✅ 自定义按键绑定 |
| 预览面板 | ❌ 不支持 | ✅ 分屏预览 |
| 适用场景 | 简单 yes/no 选择 | `si` 交互搜索、进度面板 |

用于 `si` 命令的交互式搜索界面，可实现表格列表 + 详情预览面板。

### ♻️ 存储：rusqlite → 纯 JSON 文件

当前阶段（Phase 1-2），Hit 的存储需求仅为安装清单和配置，`db.json` 单文件足够。移除 rusqlite 可减少编译时间和依赖体积。若后续需要全局索引或复杂查询，再引入不迟。

### 🔮 长期观察：git2 → gix

**gix**（gitoxide）是纯 Rust 的 Git 实现，编译快、无 C 依赖。但目前 API 仍在快速迭代，git2 更成熟。建议 Phase 1 用 git2，后续评估迁移。

---

> 详见 [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) — 项目结构与模块说明
