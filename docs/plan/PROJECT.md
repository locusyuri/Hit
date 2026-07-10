# Hit — Windows 软件包管理器

> **正式项目描述文件**。涵盖项目定位、功能特性、目录结构、模块职责与数据布局。
> 开发推进请以 [TODO.md](./TODO.md) 为权威任务清单，灵感记录见 [EUREKA.md](./EUREKA.md)。

---

## 📁 项目概览

Hit 是一个用 Rust 编写的 Windows 软件包管理器，**完全兼容 Scoop 的软件仓库（Bucket）**。

### 核心设计理念

- **Scoop 兼容**：直接复用 Scoop 的所有 bucket，无需重新维护软件清单
- **零污染**：所有软件安装在用户目录，无需管理员权限
- **Shim 代理**：通过轻量级代理实现 PATH 不污染
- **版本管理**：支持 JDK、Python、Node.js 等 SDK 的多版本切换
- **深度卸载**：集成类似 Geek Uninstaller 的残留扫描清理功能
- **便携化**：解压即用，卸载干净
- **高性能**：Rust 编写，启动快、内存占用低
- **安全性**：软件完整性校验，安全扫描集成
- **开箱即用**：零前置依赖，用户下载即用（无需预装 git/7zip）

---

## ✨ 功能特性

### 命令简写系统

Rust CLI 内置子命令简写，无需 shell 包装：

| 完整命令 | 简写 |
|----------|------|
| `hit install` | `hit i` |
| `hit search` | `hit s` |
| `hit status` | `hit st` |
| `hit update` | `hit u` |
| `hit uninstall` | `hit rm` |
| `hit list` | `hit ls` |
| `hit bucket` | `hit b` |
| `hit cleanup` | `hit c` |

### 零污染安装

- 所有软件安装在用户目录 `~/.hit/apps/`，无需管理员权限
- 不写注册表、不修改系统文件
- **便携化**：解压即用，通过脚本卸载干净（`scripts\uninstall-env.ps1` 清理环境变量 / `scripts\uninstall-hit.ps1` 彻底删除全部内容）

### Shim 代理机制

- 在 `~/.hit/shims/` 生成轻量级代理 exe（~200KB）
- 加入 PATH 后自动转发命令到实际程序
- 版本切换时只需更新 junction 指向

### SDK 多版本管理

- 支持 JDK、Python、Node.js 等 SDK 多版本共存
- `current` junction 切换当前激活版本
- 版本约束语法：`@latest`、`@stable`、`@^3.12`、`@3.12.0`

### 深度卸载

- 扫描注册表卸载信息（`winreg`）
- 并行残留文件扫描（`walkdir` + `rayon`）
- 清理注册表键值、服务、计划任务
- 强制进程终止（`windows-rs` API）

### 事务性安装

- 原子操作：任意阶段失败自动回滚
- 下载 → 校验哈希 → 解压 → 安装 → 生成 Shim 全链路保护
- 使用临时目录 + `std::fs::rename` 原子移动

### 依赖解析

- Manifest 中声明依赖关系（版本约束、可选/必需）
- 依赖图检测循环依赖和版本冲突
- 先安装依赖，再安装主包

### Bucket 优化

- **三层索引**：全局索引 → Bucket 缓存 → 源仓库
- **优先级系统**：main(100) > sdk(50) > extras(30)
- **软件别名**：`py` → `python`
- **安装前预览**：版本、大小、依赖、来源
- **浅克隆**：默认 `depth=1` 浅克隆加速拉取，按需通过 `hit bucket config <name> --full-clone` 切换全量
- **快速创建**：`hit bucket create` 交互式引导，初始化目录结构、生成 `bucket.json`，可配合 `gh` CLI 推送至 GitHub

### 首次启动引导

首次运行 `hit`（检测到 `config.json` 不存在）时显示欢迎引导：

```
👋 欢迎使用 Hit！请选择快速开始方式：
  1. 快速开始（导入 main + extras + versions bucket）
  2. 自定义选择
  3. 跳过
```

降低新手入门门槛，快速进入可用状态。

### 远期功能（Phase 3-5）

| 功能 | 说明 | 阶段 |
|------|------|:----:|
| 健康检查 | 定期校验文件完整性，`hit check` / `hit repair` | Phase 3 |
| 镜像源管理 | 内置中国镜像，自动速度测试与切换 | Phase 3 |
| 软件束（Bundle） | 一键安装多个软件，适合团队标准化环境 | Phase 4 |
| 沙盒环境（Shadow） | 隔离运行时环境 | Phase 4 |
| 生命周期管理 | 归档、孤立文件清理、跨软件去重 | Phase 4-5 |
| 运行时监控 | `hit top` / `hit ps` / `hit trace` | Phase 4 |
| 插件系统 | Lua 脚本引擎、插件钩子 | Phase 5 |
| 配置同步 | 跨设备同步配置和已安装列表 | Phase 5 |
| 增量更新 | 仅下载差异部分 | Phase 5 |
| 跨平台支持 | Linux / macOS 适配 | — |

---

## 🗂️ 完整目录结构

```
hit/
├── Cargo.toml                    # 主工作区配置
├── Cargo.lock                    # 依赖锁定文件
├── README.md                     # 项目说明文档
├── LICENSE                       # 开源许可证
├── .gitignore                    # Git 忽略配置
│
├── docs/                         # 项目文档
│   ├── OVERVIEW.md               # 文档总入口（本文件所处位置）
│   ├── TODO.md                   # 实现任务清单（Phase 1-3 权威）
│   ├── plan/                     # 项目规划
│   │   ├── PROJECT.md            # 项目描述（本文件）
│   │   └── EUREKA.md             # 灵感速记
│   ├── guides/                   # 开发指南
│   │   ├── DEV_FLOW.md           # 开发流程
│   │   └── TECH_STACK.md         # 技术栈清单
│   ├── spec/                     # 规范与参考
│   │   ├── MANIFEST_FORMAT.md    # Manifest 清单格式
│   │   └── REFERENCE_PROJECTS.md # 参考项目
│   └── review/                   # 评审记录
│       ├── REVIEW_1.1.md
│       └── REVIEW_1.2.md
│
├── crates/                       # Rust 工作区子模块
│   │
│   ├── hit-common/               # 基础类型库（lib）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs          # HitError 枚举（thiserror）
│   │       ├── config.rs         # Config 结构体（sonic-rs）
│   │       ├── paths.rs          # Scoop 兼容路径计算
│   │       ├── log.rs            # tracing 日志
│   │       ├── session.rs        # Session/Context 模式
│   │       └── event.rs          # EventBus（flume bounded channel）
│   │
│   ├── hit-core/                 # 核心业务逻辑库（lib）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── manifest/         # 解析（schema / parser / validator / variables）
│   │       ├── bucket/           # Bucket 管理（git_client / index / registry）
│   │       ├── download/         # HTTP 下载与缓存
│   │       ├── hash/             # 哈希校验（sha256 / sha512 / blake3）
│   │       ├── compress/         # 解压（zip / sevenz-rust2 / tar / flate2 / lzma-rs / installer）
│   │       ├── store/            # JSON 文件存储（db.json）
│   │       ├── install/          # 安装流水线（controller / transaction / persist / dependency）
│   │       ├── shim_mgmt/        # Shim 创建/移除/枚举
│   │       └── win/              # Windows 集成（process / registry / fs / uac / env）
│   │
│   ├── hit-shim/                 # Shim 代理（独立 bin，~200KB）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs           # 读 db.json → 解析真 exe → spawn → 转发 stdio
│   │
│   ├── hit-cli/                  # CLI 入口（bin）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── cli.rs            # clap 命令树（含 alias）
│   │       ├── progress.rs       # EventBus → indicatif 进度条 + owo-colors 彩色文本
│   │       ├── tables.rs         # tabled 表格渲染（search/list/cache/bucket）
│   │       ├── output.rs         # 统一色彩主题与语义化输出函数
│   │       └── commands/         # install / uninstall / list / search / ...
│   │
│   └── hit-test-utils/           # 共享测试 fixture（仅 [dev-dependencies]）
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs            # mock_config(), sample_manifest(), temp_scoop_root()
│
├── ref/                          # 参考源码
│   ├── Scoop/                    # 原版 Scoop PowerShell 源码
│   ├── Main/                     # Scoop 官方 Main Bucket
│   └── Hok/                      # Rust 实现的 Scoop 替代品
│
├── buckets/                      # 默认 Bucket 仓库
│   ├── main/                     # 主仓库（命令行工具）
│   ├── extras/                   # 扩展仓库（GUI 应用）
│   ├── versions/                 # 历史版本仓库
│   └── ...
│
└── scripts/                      # 辅助脚本
    ├── install-hit.ps1           # 一键安装（部署 binary + 注册 PATH + 首次引导）
    ├── uninstall-env.ps1        # 卸载模式1：只清理环境变量，保留已安装软件
    ├── uninstall-hit.ps1        # 卸载模式2：彻底删除全部内容 + 清理环境变量
    ├── release.ps1              # 本地发布：编译 release 并复制到 scripts/
    ├── run-tests.ps1            # 自动批量实测
    └── checkver.ps1             # 检查 manifest 版本更新
```

---

## 🔑 核心模块说明

> 模块架构以 [TODO.md](./TODO.md) 为 Phase 1-3 权威清单；本节提供设计意图与职责概览。

### hit-common — 基础类型库

跨 crate 共享的基础设施：
- **HitError**：`thiserror` 统一错误枚举，覆盖 IO / Manifest / Bucket / Download / Install / Config 等类别
- **Config**：`sonic-rs` 序列化的用户配置（proxy, mirror, no_junction, root_path 等）；链接策略仅用 Junction
- **paths**：Scoop 兼容路径计算（root_path, cache_path, apps_path, shims_path, persist_path）
- **Session/Context**：参考 `ref/Hok/libscoop/session.rs`，持有 `RefCell<Config>`、`OnceCell<EventBus>`、路径缓存
- **EventBus**：`flume` bounded channel（容量 20），定义 `Event` 枚举
- **log**：`tracing` 日志初始化

### hit-core — 核心业务逻辑库

实现软件安装、卸载、版本管理等核心功能：
- **manifest/**：Scoop Manifest 解析（schema / parser / validator / variables）
- **bucket/**：Bucket git 仓库克隆/更新/索引（gix + rayon 并行）
- **download/**：HTTP 下载与缓存（reqwest blocking），进度通过 EventBus 上报
- **hash/**：sha256 / sha512 / blake3 流式哈希校验
- **compress/**：分层解压策略——ZIP/7z/TAR/XZ 用纯 Rust 库（zip + sevenz-rust2 + tar + flate2 + lzma-rs），NSIS/Inno Setup 用静默运行，7z.exe 仅作 SFX 强制提取兜底
- **store/**：JSON 文件存储（db.json 原子写入 + 版本迁移）
- **install/**：安装流水线（controller / transaction / persist / dependency / hooks）
- **shim_mgmt/**：Shim 创建/移除/枚举
- **win/**（`#[cfg(windows)]`）：进程检测、注册表、文件系统（Junction）、UAC、环境变量

### hit-shim — Shim 代理程序

独立轻量级 exe（~200KB），负责命令转发：
1. 接收命令行参数
2. 读取 `~/.hit/db.json` 获取当前激活版本
3. 拼接真实路径
4. 启动真实进程并转发 stdin/stdout/stderr
5. 返回退出码

### hit-cli — 命令行界面

用户交互入口，`clap` 参数解析（含 alias）、`tabled` 表格渲染、`indicatif` 进度条、`owo-colors` 统一色彩输出。

**新增模块**：
- `output.rs`：统一色彩主题与语义化输出函数，定义成功绿色、错误红色、警告黄色、步骤青色、表格表头青色粗体等样式

**统一色彩输出**：
- 成功消息使用绿色（✔）
- 错误消息使用红色（✘）
- 警告消息使用黄色（⚠）
- 步骤提示使用青色（▶）
- 表格表头使用青色粗体高亮
- 次要信息使用灰色（dim）

**核心依赖**：
- `owo-colors`：零依赖终端着色，支持 ANSI 颜色和样式
- `anstyle`：ANSI 颜色控制核心
- `supports-color`：NO_COLOR / TTY 检测
- `terminal-size`：终端宽度检测
- `is-terminal`：TTY 检测

### hit-test-utils — 共享测试 fixture

`mock_config()`、`sample_manifest()`、`temp_scoop_root()` 等辅助函数，仅 `[dev-dependencies]`。

---

## 📦 数据存储结构

### 用户目录布局（默认 `~/.hit/`）

```
C:\Users\<username>\.hit\
├── apps/                     # 软件安装目录
│   ├── git/
│   │   ├── 2.40.0/          # 具体版本
│   │   │   ├── bin/
│   │   │   └── ...
│   │   └── current [JUNCTION → 2.40.0]  # junction 指向当前版本
│   ├── python/
│   │   ├── 3.11.0/
│   │   ├── 3.12.0/
│   │   └── current [JUNCTION → 3.12.0]
│   └── ...
│
├── shims/                    # Shim 代理目录（加入 PATH）
│   ├── git.exe               # shim.exe 副本
│   ├── git.shim              # 元数据：path = ...\apps\git\current\bin\git.exe
│   └── ...
│
├── persist/                  # 持久化数据（junction + hard_link）
│   ├── git/                  # junction 到 apps/git/current/etc
│   │   └── config            # hard_link 到源文件
│   └── ...
│
├── cache/                    # 下载缓存
│   └── <hash>.zip
│
├── buckets/                  # Bucket git 仓库（默认浅克隆）
├── db.json                   # 已安装软件清单
├── config.json               # 用户配置
├── logs/                     # 日志文件
└── plugins/                  # 插件目录
```

---

## 📚 参考目录

| 目录 | 来源 | 用途 |
|------|------|------|
| [`ref/Scoop/`](../ref/Scoop/) | [Scoop PowerShell](https://github.com/ScoopInstaller/Scoop) | 原版 Scoop 实现 |
| [`ref/Main/`](../ref/Main/) | [Scoop Main Bucket](https://github.com/ScoopInstaller/Main) | 官方软件清单，兼容性测试 |
| [`ref/Hok/`](../ref/Hok/) | [hok](https://github.com/chawyehsu/hok) | Rust 实现的 Scoop 替代品（较久未更新） |

详情见 [REFERENCE_PROJECTS.md](../spec/REFERENCE_PROJECTS.md)。

---

> 开发推进：[TODO.md](./TODO.md) | 灵感记录：[EUREKA.md](./EUREKA.md) | 开发流程：[DEV_FLOW.md](../guides/DEV_FLOW.md) | 技术栈：[TECH_STACK.md](../guides/TECH_STACK.md)
