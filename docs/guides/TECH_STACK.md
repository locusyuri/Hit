# 技术栈清单

## 核心依赖总览

| 模块 | 依赖 | 用途 | 评估 |
|------|------|------|------|
| CLI | clap | 参数解析 | ✅ 事实标准，稳定 |
| CLI | indicatif | 进度条 | ✅ 事实标准 |
| CLI | colored | 彩色输出 | ✅ 简单够用，不引入额外复杂度 |
| CLI | ratatui | TUI 交互界面 | ✅ 生态最活跃的 Rust TUI 框架 |
| Core | serde + sonic-rs | JSON 序列化 | ✅ sonic-rs 比 serde_json 快 1.5~2x |
| Core | thiserror | 错误处理 | ✅ 事实标准 |
| Core | petgraph | 依赖图 | ✅ 唯一成熟的依赖图库 |
| Core | regex | 正则校验 | ✅ 事实标准 |
| Core | url | URL 解析 | ✅ 事实标准 |
| Bucket | **gix** | Git 仓库操作 | ⬆️ **从 git2 迁移到 gix**（纯 Rust，编译快） |
| Downloader | reqwest | HTTP 下载 | ✅ 功能最全，async/blocking 双模式 |
| Downloader | blake3/sha2 | 哈希计算 | ✅ 合理选择 |
| Compression | zip / sevenz-rust / tar | 压缩解压 | ⚠️ sevenz-rust 维护较慢，关注替代 |
| Store | 纯 JSON 文件 | 数据存储 | ✅ Phase 1-2 足够，后续可按需升级 |
| Windows | windows | Windows API | ✅ 官方 SDK 绑定 |
| Windows | winreg | 注册表操作 | ✅ 成熟稳定 |
| Core | junction | 目录连接 | ✅ 适配 Windows 的轻量 crate |
| Core | sysinfo | 进程检测 | ✅ 跨平台进程管理 |
| Core | tempfile | 临时目录 | ✅ 事实标准 |
| Core | rayon | 并行遍历/索引 | ✅ 事实标准，无 unsafe 滥用 |
| Core | tracing | 日志 | ✅ 事实标准 |
| Core | flume | 事件总线 | ✅ 比 crossbeam 更轻量，API 更友好 |
| Core | once_cell | 惰性初始化 | ✅ 已逐步被 std::sync::OnceLock 替代 |

---

## 变更说明

### ⬆️ git2 → gix

**决策**：从 `git2`（libgit2 C 绑定）迁移到 `gix`（gitoxide，纯 Rust）。

| 对比 | git2 | gix |
|------|------|-----|
| 语言绑定 | C 库 libgit2 | **纯 Rust** |
| 编译时间 | 慢（需编译 C 源码） | **快**（无 C 依赖） |
| 二进制体积 | 大（静态链接 libgit2） | **小** |
| 安装依赖 | 需 `cmake` 或预装 libgit2 | **零** |
| API 风格 | C 风格 wrap | **Rust 原生** |
| 浅克隆支持 | 需手动配置 | ✅ **原生支持**（`with_shallow`） |
| 成熟度 | ✅ 极稳定 | 🟡 快速迭代中（v0.x） |
| 代码片段（Context7） | — | **23,876**（活跃度高） |

**gix 能力覆盖**（Hit 需要的基础操作均已支持）：
- `gix::prepare_clone(url, path)` — 克隆仓库
- `PrepareFetch::fetch_only(progress, &should_interrupt)` — 拉取更新
- `gix::open(path)` — 打开已有仓库
- `Repository::find_fetch_remote(None)` — 获取远程配置
- `.with_shallow(remote::fetch::Shallow::DepthAtRemote(1))` — 浅克隆
- 支持进度回调、中断信号

**结论**：Hit 的 bucket 操作（clone / fetch / shallow）gix 全部覆盖，且不需要 `cmake` 等构建依赖。建议立即迁移。

### ⚡ JSON：serde_json → sonic-rs

**sonic-rs**（字节跳动 CloudWeGo）是 serde 兼容的高性能 JSON 库，利用 SIMD 加速。

| 场景 | serde_json | sonic-rs | 提升 |
|------|:-:|:-:|:-:|
| 非结构化序列化（twitter） | 797µs | **390µs** | **2.0x** |
| 结构体序列化（twitter） | 740µs | **448µs** | **1.6x** |
| 非结构化序列化（citm） | 1.88ms | **822µs** | **2.3x** |

**迁移成本**：极低，sonic-rs 完全兼容 serde。

### 🖥️ TUI：dialoguer → ratatui

| 对比 | dialoguer | ratatui |
|------|-----------|---------|
| 定位 | 简单交互提示 | 完整 TUI 框架 |
| 表格 | ❌ 不支持 | ✅ 表格/列表/多列 |
| 快捷键 | ❌ 有限 | ✅ 自定义按键绑定 |
| 预览面板 | ❌ 不支持 | ✅ 分屏预览 |
| 适用场景 | 简单 yes/no 选择 | `si` 交互搜索、进度面板 |

### ♻️ 存储：rusqlite → 纯 JSON 文件

当前阶段（Phase 1-2），Hit 的存储需求仅为安装清单和配置，`db.json` 单文件足够。

---

## 各依赖详细分析

### ✅ 成熟稳定（无需更换）

| 依赖 | 理由 |
|------|------|
| **clap** | Rust CLI 事实标准，derive 宏成熟，文档丰富 |
| **indicatif** | 唯一成熟的进度条库，MultiProgress 支持并行下载 |
| **colored** | API 极简，无依赖，够用。`owo-colors` 虽有更现代的设计但迁移收益低 |
| **serde** | 无竞品，绑定整个 Rust 生态 |
| **thiserror** | 错误 derive 库标准 |
| **petgraph** | 唯一成熟的 Rust 依赖图库 |
| **reqwest** | HTTP 客户端标准，blocking 模式在 CLI 场景下合适 |
| **windows / winreg** | 微软官方绑定，同步更新 SDK |
| **tracing** | 日志事实标准，性能优于 log + env_logger |
| **rayon** | 并行迭代事实标准 |
| **tempfile** | 临时目录管理事实标准 |
| **junction** | 依赖极少，专注 Windows 目录连接，适配 Hit 的 Junction 策略 |

### ⚠️ 谨慎使用

| 依赖 | 问题 | 建议 |
|------|------|------|
| **sevenz-rust** | 更新频率低，最后一次发布间隔较长 | 关注其维护状态；备选：`sevenz` 或直接调 7z.exe（不推荐） |
| **once_cell** | std::sync::OnceLock 已在 Rust 1.70 稳定 | 新代码可直接用 `std::sync::OnceLock`，逐步减少 once_cell 依赖 |
| **blake3** | 仅用于 Manifest hash 校验，sha2 已覆盖多数场景 | 可在 Phase 2 评估是否真正需要，不需要可移除 |

### 🟢 备选方案观察

| 替代库 | 当前选择 | 何时考虑 |
|--------|---------|----------|
| **gix**（已选） | git2 | 已决定迁移，见上方 |
| **ureq** | reqwest | 如果 Hit 改为纯 blocking 模式（无 async），ureq 编译更快、API 更简洁 |
| **sqlx / redb** | 纯 JSON | 当需要全局搜索索引或复杂查询时评估 |
| **toml_edit** | JSON | 如果配置需用户手动编辑，TOML 更友好（Phase 3+ 评估） |

---

> 详见 [PROJECT.md](../plan/PROJECT.md) — 项目描述与模块说明
