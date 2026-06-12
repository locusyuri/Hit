# Hit 项目实现 TODO 清单

## 🎯 目标：将 Scoop 重构为 Rust，完全兼容 Scoop Bucket

> **文档定位**：本文档是 Phase 1-3 实现的 **权威任务清单**。
> Phase 4-5 的远期规划请参阅 [ROADMAP.md](./plan/ROADMAP.md)。
> 项目结构与模块划分请参阅 [PROJECT_STRUCTURE.md](./plan/PROJECT_STRUCTURE.md)。
> 当其他文档与本文档在 Phase 1-3 范围内存在冲突时，以本文档为准。

---

## 架构设计概述

### Scoop PowerShell 模块 → Rust 模块映射

| Scoop 模块           | 功能描述                     | 对应 Rust 位置           |
| -------------------- | ---------------------------- | ------------------------ |
| `lib/core.ps1`       | 基础工具、配置管理、路径处理 | `hit-common` crate       |
| `lib/manifest.ps1`   | Manifest 解析与验证          | `hit-core::manifest`     |
| `lib/install.ps1`    | 安装流程控制                 | `hit-core::install`      |
| `lib/database.ps1`   | JSON 文件存储（db.json）     | `hit-core::store`        |
| `lib/buckets.ps1`    | Bucket 仓库管理              | `hit-core::bucket`       |
| `lib/download.ps1`   | 文件下载                     | `hit-core::download`     |
| `lib/decompress.ps1` | 解压功能                     | `hit-core::compress`     |
| `lib/depends.ps1`    | 依赖解析                     | `hit-core::install`      |
| `libexec/*.ps1`      | 命令实现                     | `hit-cli` crate          |
| `supporting/shims`   | Shim 代理                    | `hit-shim` crate         |

### Rust 工作区结构（5-crate 方案）

> 参考 `ref/Hok/` 的 3-crate 方案（binary + libscoop + scoop_hash），Hit 按"职责边界 + 体积约束"拆分为 5 crate。细粒度子模块（manifest / bucket / download / compress / store / win）合并入 `hit-core` 内部模块而非独立 crate，避免在实现初期就维护过多 API 边界。

```
crates/
├── hit-common/           # 基础类型库（lib）
│   └── src/
│       ├── lib.rs
│       ├── error.rs      # HitError 枚举（thiserror）
│       ├── config.rs     # Config 结构体（sonic-rs 序列化）
│       ├── paths.rs      # Scoop 兼容路径计算
│       ├── log.rs        # tracing 初始化
│       ├── session.rs    # Session/Context 模式（参考 ref/Hok/libscoop/session.rs）
│       └── event.rs      # EventBus + Event 枚举（flume bounded channel）
├── hit-core/             # 核心业务逻辑库（lib）
│   └── src/
│       ├── lib.rs
│       ├── manifest/     # Manifest 解析（schema.rs, parser.rs, validator.rs, variables.rs）
│       ├── bucket/       # Bucket 管理（git_client.rs, index.rs, registry.rs）
│       ├── download/     # 下载与缓存（http.rs, cache.rs）
│       ├── hash/         # 哈希校验（sha256/sha512/blake3，流式计算）
│       ├── compress/     # 解压（zip.rs, sevenz.rs, tar.rs, installer.rs）
│       ├── store/        # JSON 文件存储（db.json：load/save/migration/models）
│       ├── install/      # 安装流水线（controller.rs, transaction.rs, persist.rs, dependency.rs, hooks.rs）
│       ├── shim_mgmt/    # Shim 创建/移除/枚举
│       └── win/          # Windows 平台集成（#[cfg(windows)]）
│                         #   process.rs（sysinfo）, registry.rs（winreg）,
│                         #   fs.rs（symlink + junction fallback）,
│                         #   uac.rs（ShellExecuteW RunAs）, env.rs（WM_SETTINGCHANGE）
├── hit-shim/             # Shim 代理（独立 bin，~200KB）
│   └── src/
│       └── main.rs       # 读 db.json → 解析真 exe → spawn → 转发 stdio
├── hit-cli/              # CLI 入口（bin）
│   └── src/
│       ├── main.rs
│       ├── cli.rs        # clap 命令树（含 alias：i/s/u/rm/ls/st/b/c）
│       ├── progress.rs   # EventBus 订阅 → indicatif / colored 渲染
│       ├── tui.rs        # ratatui 交互搜索（Phase 3）
│       └── commands/     # install.rs, uninstall.rs, list.rs, search.rs,
│                         # info.rs, update.rs, bucket.rs, hold.rs, ...
└── hit-test-utils/       # 共享测试 fixture（仅 [dev-dependencies]）
    └── src/
        └── lib.rs        # mock_config(), sample_manifest(), temp_scoop_root()
```

#### 合并/拆分理由

| 原 10-crate 方案 | 5-crate 方案处置 | 理由 |
| ---------------- | ---------------- | ---- |
| `hit-manifest`   | 并入 `hit-core::manifest` | 解析与 install 强耦合，独立 crate 会反复协商 API |
| `hit-bucket`     | 并入 `hit-core::bucket` | 操作仅被 install/search/update 调用，无需 pub 边界 |
| `hit-downloader` | 并入 `hit-core::download` + `hit-core::hash` | 单一调用点 |
| `hit-compression`| 并入 `hit-core::compress` | 单一调用点 |
| `hit-store`      | 并入 `hit-core::store` | JSON 文件 I/O 约 200 行，无需独立 crate |
| `hit-windows`    | 并入 `hit-core::win`（`#[cfg(windows)]`） | 被多处 hit-core 模块使用，避免循环依赖 |
| `hit-shim`       | **保留独立 bin** | 每个安装二进制对应一个 shim，必须控制体积 ~200KB |

### Rust 优势利用

| Rust 特性  | 应用场景             | 带来的好处                 |
| ---------- | -------------------- | -------------------------- |
| 类型安全   | Manifest 数据结构    | 编译期检查，避免运行时错误 |
| 所有权系统 | 文件操作、资源管理   | 自动资源清理，无内存泄漏   |
| 并发支持   | 并行下载、索引构建   | 提升性能                   |
| 零成本抽象 | 模块化设计           | 代码复用，无运行时开销     |
| 宏系统     | 命令行解析、错误处理 | 减少样板代码               |
| Cargo 生态 | 依赖管理、构建系统   | 简化开发流程               |

---

## Phase 1：Scoop 基础能力实现（3个月）

### 1.1 项目初始化与基础架构

| 序号  | 任务                                                                                              | 状态 | 负责人 | 预估时间 | 依赖  |
| ----- | ------------------------------------------------------------------------------------------------- | ---- | ------ | -------- | ----- |
| 1.1.1 | 创建 Cargo workspace 结构：根 Cargo.toml 添加 `[workspace]`，创建 `crates/`，按 5-crate 方案初始化子 crate 骨架（空 lib.rs / main.rs） | ✅    | -      | 已完成   | -     |
| 1.1.2 | 配置 workspace 级 Cargo.toml：`[workspace.dependencies]`（serde, sonic-rs, thiserror, anyhow, tracing, flume）；`[profile.release]`（LTO, strip, opt-level=3, codegen-units=1） | ✅    | -      | 已完成   | 1.1.1 |
| 1.1.3 | 完善 .gitignore：在现有 `/target`、`.codegraph/`、`.agents/graph.bin` 基础上追加 `Cargo.lock`（库 crate 不锁）、`*.swp`、`.vs/`、`*.pdb` | ✅    | -      | 已完成   | -     |
| 1.1.4 | 创建项目文档结构（已完成：docs/ 含 10 个 .md 文件）                                               | ✅    | -      | 已完成   | -     |
| 1.1.5 | 初始化 hit-common crate：定义 error.rs / config.rs / paths.rs / log.rs                            | ✅    | -      | 已完成   | 1.1.1 |
| 1.1.6 | 定义 HitError 枚举（hit-common/src/error.rs）：thiserror derive，覆盖 IO、Manifest、Bucket、Download、Install、Config 等错误类别；对外暴露 `type Result<T> = std::result::Result<T, HitError>` | ✅    | -      | 已完成   | 1.1.5 |
| 1.1.7 | 实现 Session/Context 模式（hit-common/src/session.rs）：Session 结构体持有 `RefCell<Config>`、`OnceCell<EventBus>`、路径缓存；所有核心操作以 `&Session` 为首参数；参考 `ref/Hok/crates/libscoop/src/session.rs` | ✅    | -      | 已完成   | 1.1.5 |
| 1.1.8 | 实现 EventBus 事件总线（hit-common/src/event.rs）：flume bounded channel（容量 20）；定义 `Event` 枚举（DownloadProgress, ExtractStart, InstallStep, BucketUpdateProgress, PromptConfirm 等）；Session 通过 `event_bus()` 暴露 sender/receiver | ✅    | -      | 已完成   | 1.1.5 |
| 1.1.9 | 初始化 hit-test-utils crate 骨架：dev-dependency 库，含 mock_config()、sample_manifest()、temp_scoop_root() 辅助函数 | ✅    | -      | 已完成   | 1.1.1 |

### 1.2 hit-core/manifest：Scoop Manifest 格式兼容解析

| 序号  | 任务                                                                    | 状态 | 负责人 | 预估时间 | 依赖  |
| ----- | ----------------------------------------------------------------------- | ---- | ------ | -------- | ----- |
| 1.2.1 | 分析 Scoop Manifest JSON Schema（参考 `ref/Main/` 真实 manifest）       | 📋    | -      | 3天      | -     |
| 1.2.2 | 定义 Manifest 数据结构（hit-core/src/manifest/schema.rs，serde derive）：完整反序列化 Scoop 字段（architecture/bin/env_set/persist/depends/pre_install/post_install/pre_uninstall/shortcuts/checkver/autoupdate 等）；**Hit 扩展字段**（alias, dependencies, health_check, mirrors 等）在此阶段声明但标记 `#[serde(default, skip_serializing_if)]` 跳过解析 | 📋    | -      | 5天      | 1.2.1 |
| 1.2.3 | 实现变量替换引擎（hit-core/src/manifest/variables.rs）：支持 `$version`, `$architecture`, `$url`, `$dir`, `$appdir`, `$scoopdir`, `$persist_dir` 等 Scoop 内置变量；递归替换 url、hash、bin、env_set 中的变量引用 | 📋    | -      | 5天      | 1.2.2 |
| 1.2.4 | 实现 Manifest 解析器（hit-core/src/manifest/parser.rs）                 | 📋    | -      | 5天      | 1.2.2 |
| 1.2.5 | 实现 Manifest 验证器（hit-core/src/manifest/validator.rs）              | 📋    | -      | 3天      | 1.2.2 |
| 1.2.6 | 支持 Scoop 特殊字段（architecture、depends、persist、pre/post_install） | 📋    | -      | 5天      | 1.2.2 |
| 1.2.7 | 编写 Manifest 解析单元测试（hit-core/tests/manifest_test.rs）：使用 `ref/Main/` 中的真实 manifest JSON 作为测试 fixtures，覆盖 git.json、python.json、7zip.json 等代表性清单 | 📋    | -      | 3天      | 1.2.2-1.2.5 |

### 1.3 hit-core/bucket：Scoop Bucket 仓库支持

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖  |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ----- |
| 1.3.1 | 实现 Git 仓库克隆（hit-core/src/bucket/git_client.rs，git2 crate）：clone with progress；支持 proxy 配置（从 Session config 读取）；进度通过 EventBus 发送 `BucketUpdateProgress` 事件 | 📋    | -      | 5天      | 1.1.7, 1.1.8 |
| 1.3.2 | 实现 Bucket 更新（git pull）                                                           | 📋    | -      | 3天      | 1.3.1 |
| 1.3.3 | 实现 Bucket 列表管理                                                                   | 📋    | -      | 3天      | 1.3.1 |
| 1.3.4 | 解析 bucket.json 元数据                                                                | 📋    | -      | 3天      | 1.2.2 |
| 1.3.5 | 默认添加 Scoop 官方 bucket（main, extras）                                             | 📋    | -      | 2天      | 1.3.1 |
| 1.3.6 | 构建软件索引（hit-core/src/bucket/index.rs）：遍历 bucket 目录下所有 .json 文件（rayon par_bridge 并行），解析为 Manifest 摘要（name + version + description），存入内存 `HashMap<String, Vec<PackageSummary>>` | 📋    | -      | 5天      | 1.2.2 |

### 1.4 hit-core/download：下载与哈希校验

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖  |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ----- |
| 1.4.1 | 实现 HTTP 下载器（hit-core/src/download/http.rs，reqwest blocking client）：支持 proxy 配置（Session config）；下载进度通过 EventBus 发送 `DownloadProgress` 事件（已下载字节/总字节/速率） | 📋    | -      | 5天      | 1.1.7, 1.1.8 |
| 1.4.2 | 实现缓存管理（hit-core/src/download/cache.rs）                                         | 📋    | -      | 3天      | 1.4.1 |
| 1.4.3 | 实现哈希校验（hit-core/src/hash/mod.rs）：支持 sha256、sha512、blake3；流式计算（避免大文件内存问题）；校验失败返回 `HashMismatch` 错误（含 expected/actual/path 上下文） | 📋    | -      | 3天      | -     |

> 1.4.x 说明：断点续传、GitHub API 下载延后到 Phase 2+；MVP 阶段失败即重下（参考 Hok）。

### 1.5 hit-core/compress：解压模块

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖 |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ---- |
| 1.5.1 | 实现 ZIP 解压（hit-core/src/compress/zip.rs，zip crate）                               | 📋    | -      | 3天      | -    |
| 1.5.2 | 实现 7z 解压（hit-core/src/compress/sevenz.rs，sevenz-rust）                           | 📋    | -      | 3天      | -    |
| 1.5.3 | 实现 TAR 解压（hit-core/src/compress/tar.rs，tar + flate2）                            | 📋    | -      | 3天      | -    |
| 1.5.4 | 支持安装程序处理（hit-core/src/compress/installer.rs）：NSIS 静默安装（/S flag）、Inno Setup 静默安装（/VERYSILENT）、MSI 安装（msiexec /qn）；通过 Session config 控制是否使用 lessmsi 提取 MSI | 📋    | -      | 5天      | -    |

### 1.6 hit-core/win：Windows 平台集成

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖  |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ----- |
| 1.6.1 | 实现进程管理（hit-core/src/win/process.rs，sysinfo crate）：检测运行中的进程；安装前检查目标进程是否在运行；支持优雅终止和强制终止 | 📋    | -      | 3天      | -     |
| 1.6.2 | 实现注册表操作（hit-core/src/win/registry.rs，winreg crate）：读写 `HKCU\Environment`（PATH 管理）；读写 `HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall`（已安装软件检测） | 📋    | -      | 3天      | -     |
| 1.6.3 | 实现文件系统操作（hit-core/src/win/fs.rs）：**junction fallback 策略** —— symlink_dir 先尝试 `std::os::windows::fs::symlink_dir`，失败后回退到 `junction::create`（无需管理员或开发者模式）；symlink_file 失败后回退到 `std::fs::hard_link`；实现 remove_symlink 函数（自动检测文件/目录类型）；参考 `ref/Hok/crates/libscoop/src/internal/fs.rs:133-159` | 📋    | -      | 4天      | -     |
| 1.6.4 | 实现 UAC 提权（hit-core/src/win/uac.rs）：`is_admin()` 检测当前是否管理员；`elevate_self()` 使用 ShellExecuteW + RunAs verb 重新以管理员启动自身；仅在需要 symlink（非 junction）时触发 | 📋    | -      | 5天      | -     |
| 1.6.5 | 实现环境变量管理（hit-core/src/win/env.rs）：修改用户级 PATH（添加/移除 shims 目录）；广播 `WM_SETTINGCHANGE` 消息通知其他进程刷新环境变量（使用 SendMessageTimeoutW） | 📋    | -      | 3天      | -     |
| 1.6.6 | 实现 `no_junction` 配置支持：Config 中添加 `no_junction: Option<bool>` 字段；当 `no_junction=true` 时，跳过 `current` 目录符号链接创建，shim 直接指向具体版本路径（兼容 Scoop 同名配置项） | 📋    | -      | 1天      | 1.1.5, 1.6.3 |

### 1.7 hit-shim：Shim 代理机制（独立 bin）

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖        |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ----------- |
| 1.7.1 | 创建 hit-shim 独立 binary crate：`crates/hit-shim/Cargo.toml` 仅依赖 hit-common + sonic-rs，不使用 workspace 默认 heavy dependencies；`[profile.release]` 单独优化体积 | 📋    | -      | 2天      | 1.1.1       |
| 1.7.2 | 实现命令转发逻辑：根据 shim 文件名查找 `~/.hit/apps/<name>/current/<binary>`；使用 `std::process::Command` 启动真实进程；完整转发 stdin/stdout/stderr 和所有命令行参数 | 📋    | -      | 5天      | 1.6.3       |
| 1.7.3 | 读取 db.json 获取当前版本（hit-shim/src/main.rs）：反序列化 hit-common 中定义的 `ShimResolveInfo` 结构体（app name → version → install path 的最小映射）；解析 shim 自身文件名确定目标 app | 📋    | -      | 2天      | 1.1.5       |
| 1.7.4 | 启动真实进程并转发 stdin/stdout/stderr：Windows 下使用 `CREATE_NEW_PROCESS_GROUP` 标志；正确处理 Ctrl+C 信号传播；返回子进程 exit code | 📋    | -      | 3天      | 1.6.1       |
| 1.7.5 | 最小化 shim 体积（~200KB）                                                             | 📋    | -      | 3天      | -           |

### 1.8 hit-core/install：核心安装逻辑

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖                |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ------------------- |
| 1.8.0 | 集成 Session 与 install 流程：所有安装/卸载函数签名以 `session: &Session` 为首参数；通过 `session.event_bus()` 发送安装步骤进度事件（PackageResolveStart, PackageDownloadStart, PackageExtractStart, PackageCommitStart, PackageSyncDone 等） | 📋    | -      | 2天      | 1.1.7, 1.1.8        |
| 1.8.1 | 实现事务管理器（hit-core/src/install/transaction.rs）：RAII 模式管理事务状态            | 📋    | -      | 5天      | -                   |
| 1.8.2 | 创建临时事务目录（tempfile crate）                                                     | 📋    | -      | 2天      | 1.6.3               |
| 1.8.3 | 实现原子移动（rename，使用 Windows `MoveFileEx` API）                                  | 📋    | -      | 2天      | 1.6.3               |
| 1.8.4 | 实现失败回滚机制：删除临时目录，保留已安装状态不变                                     | 📋    | -      | 3天      | 1.8.1               |
| 1.8.5 | 实现安装流程控制器（hit-core/src/install/controller.rs）：编排完整安装流水线：解析 manifest → 解析依赖 → 下载 → 校验哈希 → 解压 → 创建 shim → 设置 persist → 更新 db.json → 执行 post_install 脚本；每步通过 EventBus 发送进度事件 | 📋    | -      | 7天      | 所有上游模块        |
| 1.8.6 | 实现 Persist 持久化机制（hit-core/src/install/persist.rs）：使用 symlink_dir（含 junction fallback，依赖 1.6.3）将 app 目录中的配置文件/目录链接到 `~/.hit/persist/<app>/`；卸载时保留 persist 目录；版本切换时更新链接指向 | 📋    | -      | 5天      | 1.6.3               |
| 1.8.7 | 实现依赖解析器（hit-core/src/install/dependency.rs）：解析 Manifest 的 depends 字段    | 📋    | -      | 5天      | 1.2.2               |

### 1.9 hit-core/store：数据存储

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖     |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | -------- |
| 1.9.1 | 实现 JSON 文件存储（hit-core/src/store/mod.rs）：定义 `Db` 结构体（对应 db.json），使用 sonic-rs 序列化/反序列化；实现 `Db::load()` / `Db::save()` 原子写入（写临时文件后 rename） | 📋    | -      | 2天      | sonic-rs |
| 1.9.2 | 定义数据模型（hit-core/src/store/models.rs）：`InstalledPackage`（version, bucket, install_date, shims, persist_files, held, link_mode）、`BucketInfo`（name, url, last_update）、`HitConfig`（proxy, mirror, aria2_enabled, no_junction） | 📋    | -      | 3天      | -        |
| 1.9.3 | 实现数据库迁移（hit-core/src/store/migration.rs）：db.json 包含 `version` 字段；加载时检查版本号，自动执行迁移逻辑（字段重命名、默认值填充） | 📋    | -      | 2天      | -        |
| 1.9.4 | 实现安装记录管理                                                                       | 📋    | -      | 3天      | 1.9.1    |

### 1.10 hit-cli：命令行接口

#### 1.10.1 CLI 框架搭建

| 序号     | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖               |
| -------- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ------------------ |
| 1.10.1.1 | 使用 clap 定义命令结构（hit-cli/src/cli.rs）：`#[derive(Parser)]` 与 `#[derive(Subcommand)]` 定义子命令枚举；**添加命令简写别名**：`install` 加 `#[clap(alias = "i")]`、`search` 加 `alias = "s"`、`update` 加 `alias = "u"`、`uninstall` 加 `alias = "rm"`、`list` 加 `alias = "ls"`、`status` 加 `alias = "st"`、`bucket` 加 `alias = "b"`、`cleanup` 加 `alias = "c"`（参考 `ref/Hok/src/cmd/mod.rs`） | 📋    | -      | 3天      | -                  |
| 1.10.1.2 | 实现命令路由分发：各子命令模块接收 `&Session` 参数                                      | 📋    | -      | 2天      | 1.10.1.1, 1.1.7    |
| 1.10.1.3 | 添加进度条和彩色输出（indicatif, colored）                                             | 📋    | -      | 2天      | indicatif, colored |
| 1.10.1.4 | 集成 EventBus 进度渲染（hit-cli/src/progress.rs）：从 Session event_bus receiver 接收 Event；根据事件类型更新 indicatif ProgressBar（下载进度条、解压状态、安装步骤）；PromptConfirm 事件触发用户确认对话框 | 📋    | -      | 3天      | 1.1.8, 1.10.1.3    |

#### 1.10.2 install 命令

| 序号     | 任务                      | 状态 | 负责人 | 预估时间 | 依赖              |
| -------- | ------------------------- | ---- | ------ | -------- | ----------------- |
| 1.10.2.1 | 解析软件名和版本约束      | 📋    | -      | 2天      | hit-core/bucket   |
| 1.10.2.2 | 搜索 Bucket 获取 Manifest | 📋    | -      | 3天      | hit-core/bucket   |
| 1.10.2.3 | 调用 hit-core 执行安装    | 📋    | -      | 3天      | hit-core/install  |

#### 1.10.3 uninstall 命令

| 序号     | 任务                   | 状态 | 负责人 | 预估时间 | 依赖             |
| -------- | ---------------------- | ---- | ------ | -------- | ---------------- |
| 1.10.3.1 | 查找已安装软件         | 📋    | -      | 2天      | hit-core/store   |
| 1.10.3.2 | 调用 hit-core 执行卸载 | 📋    | -      | 3天      | hit-core/install |

#### 1.10.4 list 命令

| 序号     | 任务                     | 状态 | 负责人 | 预估时间 | 依赖             |
| -------- | ------------------------ | ---- | ------ | -------- | ---------------- |
| 1.10.4.1 | 读取数据库中的已安装列表 | 📋    | -      | 2天      | hit-core/store   |
| 1.10.4.2 | 格式化输出（表格形式）   | 📋    | -      | 2天      | -                |

#### 1.10.5 search 命令

| 序号     | 任务               | 状态 | 负责人 | 预估时间 | 依赖            |
| -------- | ------------------ | ---- | ------ | -------- | --------------- |
| 1.10.5.1 | 遍历 Bucket 索引   | 📋    | -      | 3天      | hit-core/bucket |
| 1.10.5.2 | 实现关键词模糊匹配 | 📋    | -      | 2天      | -               |
| 1.10.5.3 | 显示匹配结果       | 📋    | -      | 2天      | -               |

#### 1.10.6 info 命令

| 序号     | 任务               | 状态 | 负责人 | 预估时间 | 依赖            |
| -------- | ------------------ | ---- | ------ | -------- | --------------- |
| 1.10.6.1 | 查找软件 Manifest  | 📋    | -      | 2天      | hit-core/bucket |
| 1.10.6.2 | 格式化显示软件详情 | 📋    | -      | 2天      | -               |

#### 1.10.7 update 命令

| 序号     | 任务                 | 状态 | 负责人 | 预估时间 | 依赖                            |
| -------- | -------------------- | ---- | ------ | -------- | ------------------------------- |
| 1.10.7.1 | 更新所有 Bucket      | 📋    | -      | 3天      | hit-core/bucket                 |
| 1.10.7.2 | 检查已安装软件新版本 | 📋    | -      | 3天      | hit-core/store, hit-core/bucket |
| 1.10.7.3 | 执行软件升级         | 📋    | -      | 3天      | hit-core/install                |

#### 1.10.8 bucket 命令

| 序号     | 任务                            | 状态 | 负责人 | 预估时间 | 依赖            |
| -------- | ------------------------------- | ---- | ------ | -------- | --------------- |
| 1.10.8.1 | bucket add - 添加新 Bucket      | 📋    | -      | 3天      | hit-core/bucket |
| 1.10.8.2 | bucket remove - 移除 Bucket     | 📋    | -      | 2天      | hit-core/bucket |
| 1.10.8.3 | bucket list - 列出所有 Bucket   | 📋    | -      | 2天      | hit-core/bucket |
| 1.10.8.4 | bucket update - 更新指定 Bucket | 📋    | -      | 2天      | hit-core/bucket |

### 1.11 基础测试框架

| 序号   | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖                 |
| ------ | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | -------------------- |
| 1.11.1 | 设置单元测试框架                                                                       | 📋    | -      | 2天      | -                    |
| 1.11.2 | 编写 Manifest 解析测试                                                                 | 📋    | -      | 3天      | hit-core/manifest    |
| 1.11.3 | 编写 Bucket 管理测试                                                                   | 📋    | -      | 3天      | hit-core/bucket      |
| 1.11.4 | 编写安装卸载集成测试：使用 hit-test-utils 创建临时 Scoop root；测试完整安装流水线（manifest → download → extract → shim → db.json 更新）；测试卸载清理；测试安装失败回滚（模拟下载中断、哈希不匹配） | 📋    | -      | 5天      | hit-core/install     |
| 1.11.5 | 编写 EventBus 事件流测试：验证安装流程中事件按正确顺序发送（ResolveStart → DownloadStart → DownloadProgress... → ExtractStart → CommitStart → SyncDone） | 📋    | -      | 2天      | 1.1.8                |
| 1.11.6 | 编写 junction fallback 测试：在开发者模式关闭的环境下，验证 symlink_dir 正确回退到 `junction::create`；验证 `no_junction` 配置生效时跳过链接创建 | 📋    | -      | 2天      | 1.6.3, 1.6.6         |

---

## Phase 2：Scoop 高级功能实现（2个月）

### 2.1 高级命令实现

| 序号  | 任务                                                                                   | 状态 | 负责人 | 预估时间 | 依赖                            |
| ----- | -------------------------------------------------------------------------------------- | ---- | ------ | -------- | ------------------------------- |
| 2.1.1 | `hit reset` - 版本切换                                                                 | 📋    | -      | 5天      | hit-core/install                |
| 2.1.2 | `hit cleanup` - 清理旧版本                                                             | 📋    | -      | 4天      | hit-core/store                  |
| 2.1.3 | `hit cache` - 缓存管理                                                                 | 📋    | -      | 4天      | hit-core/download               |
| 2.1.4 | `hit status` - 状态检查                                                                | 📋    | -      | 3天      | hit-core/store, hit-core/bucket |
| 2.1.5 | `hit home` - 打开主页                                                                  | 📋    | -      | 2天      | hit-core/manifest               |
| 2.1.6 | `hit uninstall --purge` - 彻底卸载                                                     | 📋    | -      | 3天      | hit-core/install                |
| 2.1.7 | `hit which` - 查找命令对应的 shim 路径和真实 exe 路径                                  | 📋    | -      | 2天      | hit-shim                        |
| 2.1.8 | `hit prefix` - 显示安装路径                                                            | 📋    | -      | 2天      | hit-core/store                  |
| 2.1.9 | `hit hold <pkg>` - 版本锁定：在 db.json 的 InstalledPackage 中设置 `held: true` 字段；被 hold 的包在 `hit update` 时跳过升级；参考 Hok 的 `operation::package_hold` 实现 | 📋    | -      | 2天      | hit-core/store                  |
| 2.1.10| `hit unhold <pkg>` - 解除版本锁定：将 `held` 字段设回 `false`                          | 📋    | -      | 1天      | 2.1.9                           |
| 2.1.11| `hit list` 增加 held 标记：已 hold 的包在 list 输出中显示 `[held]` 标记                | 📋    | -      | 1天      | 2.1.9                           |
| 2.1.12| `hit config` 子命令：`hit config list` 显示当前配置；`hit config set <key> <value>` 修改配置（proxy, no_junction, mirror 等）；参考 Hok 的 config.rs `set()` 方法 | 📋    | -      | 3天      | hit-common/config               |

### 2.2 依赖解析增强

| 序号  | 任务                          | 状态 | 负责人 | 预估时间 | 依赖              |
| ----- | ----------------------------- | ---- | ------ | -------- | ----------------- |
| 2.2.1 | 解析 Manifest 的 depends 字段 | 📋    | -      | 3天      | hit-core/manifest |
| 2.2.2 | 构建依赖图                    | 📋    | -      | 3天      | petgraph          |
| 2.2.3 | 检测循环依赖                  | 📋    | -      | 2天      | 2.2.2             |
| 2.2.4 | 实现依赖安装顺序              | 📋    | -      | 3天      | hit-core/install  |

### 2.3 Bucket 全局索引

| 序号  | 任务                              | 状态 | 负责人 | 预估时间 | 依赖              |
| ----- | --------------------------------- | ---- | ------ | -------- | ----------------- |
| 2.3.1 | 构建内存索引（软件名 → 版本列表） | 📋    | -      | 3天      | hit-core/bucket   |
| 2.3.2 | 实现优先级系统                    | 📋    | -      | 2天      | hit-core/bucket   |
| 2.3.3 | 安装时自动选择最佳版本            | 📋    | -      | 3天      | hit-core/install  |

---

## Phase 3：Hit 增强功能（3个月）

### 3.1 健康检查

| 序号  | 任务                   | 状态 | 负责人 | 预估时间 | 依赖              |
| ----- | ---------------------- | ---- | ------ | -------- | ----------------- |
| 3.1.1 | 实现文件完整性检查     | 📋    | -      | 3天      | hit-core/download |
| 3.1.2 | 检查 Shim 指向是否正确 | 📋    | -      | 2天      | hit-shim          |
| 3.1.3 | 实现自动修复功能       | 📋    | -      | 3天      | hit-core/install  |

### 3.2 镜像源管理

| 序号  | 任务           | 状态 | 负责人 | 预估时间 | 依赖              |
| ----- | -------------- | ---- | ------ | -------- | ----------------- |
| 3.2.1 | 配置多镜像源   | 📋    | -      | 3天      | hit-common        |
| 3.2.2 | 实现速度测试   | 📋    | -      | 3天      | hit-core/download |
| 3.2.3 | 自动选择最快源 | 📋    | -      | 2天      | 3.2.2             |

### 3.3 交互式搜索

> ratatui 选型已在 TECH_STACK.md 确认，替换原 dialoguer 方案。

| 序号  | 任务                     | 状态 | 负责人 | 预估时间 | 依赖    |
| ----- | ------------------------ | ---- | ------ | -------- | ------- |
| 3.3.1 | 集成 TUI 交互界面（ratatui） | 📋    | -      | 5天      | ratatui |
| 3.3.2 | 上下箭头选择，Enter 安装 | 📋    | -      | 2天      | 3.3.1   |

---

## 远期展望（Phase 4-5）

> 以下功能不在 Phase 1-3 范围内，详见 [ROADMAP.md](./plan/ROADMAP.md) Phase 4-5。

| 功能领域         | 简述                                  | ROADMAP   |
| ---------------- | ------------------------------------- | --------- |
| SDK 多版本管理   | JDK/Python/Node.js 多版本共存与切换   | Phase 4   |
| 深度卸载         | 注册表扫描、残留文件清理、进程终止    | Phase 4   |
| 软件束 (Bundle)  | 一键安装多个软件，导出/导入配置       | Phase 4   |
| 沙盒环境 (Shadow)| 隔离运行时环境，独立 persist          | Phase 4   |
| 生命周期管理     | 归档、孤立文件清理、去重、自动清理    | Phase 4-5 |
| 运行时监控       | `hit top`/`hit ps`/`hit trace`        | Phase 4   |
| 插件系统         | Lua 脚本引擎，插件钩子                | Phase 5   |
| 配置同步         | 跨设备同步配置和已安装列表            | Phase 5   |
| 增量更新         | 差异下载，减少带宽                    | Phase 5   |
| 跨平台支持       | Linux / macOS                         | Phase 5   |

---

## 📁 目录结构对应

```
crates/
├── hit-cli/           # CLI 命令实现 + 进度渲染 + TUI
├── hit-core/          # 核心逻辑（manifest, bucket, download, compress, store, install, win）
├── hit-shim/          # Shim 代理（独立 bin）
├── hit-common/        # 基础类型（error, config, paths, session, event bus）
└── hit-test-utils/    # 共享测试 fixture（仅 dev-dependency）
```

---

## 🛠️ 技术栈清单

| 模块        | 依赖                 | 用途                    |
| ----------- | -------------------- | ----------------------- |
| Common      | thiserror            | 错误类型定义            |
| Common      | anyhow               | 错误传播                |
| Common      | serde + sonic-rs     | JSON 序列化             |
| Common      | tracing              | 日志系统                |
| Common      | flume                | EventBus 有界 channel   |
| Common      | once_cell            | 懒初始化                |
| Core        | git2                 | Bucket 仓库操作         |
| Core        | reqwest              | HTTP 下载（blocking）   |
| Core        | sha2 / blake3        | 哈希计算                |
| Core        | zip / sevenz-rust / tar / flate2 | 压缩解压  |
| Core        | petgraph             | 依赖图                  |
| Core        | rayon                | 并行索引构建            |
| Core        | tempfile             | 事务临时目录            |
| Core        | junction             | 目录连接（symlink 回退）|
| Core        | sysinfo              | 进程检测                |
| Core        | winreg               | 注册表操作              |
| Core        | windows              | Windows API（UAC 等）   |
| CLI         | clap                 | 参数解析（含 alias）    |
| CLI         | indicatif            | 进度条                  |
| CLI         | colored              | 彩色输出                |
| CLI         | ratatui              | TUI 交互界面            |
| CLI         | tracing-subscriber   | 日志订阅                |
| Store       | JSON 文件（db.json） | 数据存储                |

---

## 📅 时间线汇总

| 阶段    | 时长  | 开始   | 结束   | 核心交付物            |
| ------- | ----- | ------ | ------ | --------------------- |
| Phase 1 | 3个月 | 第1周  | 第12周 | 基础命令 + Scoop 兼容 |
| Phase 2 | 2个月 | 第13周 | 第20周 | 高级命令 + 依赖解析   |
| Phase 3 | 3个月 | 第21周 | 第32周 | Hit 增强功能          |

---

## ✅ 完成标准

### Phase 1 完成标准
- [ ] 成功安装任意 Scoop bucket 中的软件
- [ ] 成功卸载已安装软件
- [ ] 正确列出已安装软件
- [ ] 成功搜索软件
- [ ] 显示软件详情
- [ ] 成功更新 Bucket 和软件
- [ ] Bucket 管理命令正常工作
- [ ] Shim 代理正确转发命令
- [ ] Persist 配置文件正确持久化
- [ ] 安装失败时正确回滚
- [ ] 自动检测并请求提权
- [ ] junction fallback 在无开发者模式时正常创建目录连接

### Phase 2 完成标准
- [ ] 成功切换软件版本
- [ ] 成功清理旧版本
- [ ] 缓存管理正常工作
- [ ] 状态检查显示正确信息
- [ ] 依赖自动安装
- [ ] Bucket 索引加速搜索
- [ ] `hit hold` / `hit unhold` 正确锁定/解锁版本升级
- [ ] `hit config list/set` 可管理配置项

### Phase 3 完成标准
- [ ] 健康检查检测损坏文件
- [ ] 自动修复损坏安装
- [ ] 镜像源自动选择最快
- [ ] 交互式搜索安装（ratatui TUI）

---

## 📝 Rust 重构关键设计决策

### 1. Manifest 解析设计
- 使用 `serde` 进行 JSON 解析，支持 Scoop 的所有字段
- 变量替换使用自定义解析器，支持 `$version`, `$architecture`, `$url` 等变量
- 验证器使用 `schemars` 生成 JSON Schema 验证
- Hit 扩展字段（alias, bundle, shadow 等）在 Phase 1 仅声明为 `#[serde(default)]`，待对应功能阶段实现时再补充解析逻辑

### 2. 事务性安装设计
- 使用 RAII 模式管理事务状态
- 原子操作使用 Windows 的 `MoveFileEx` API
- 失败回滚通过删除临时目录实现

### 3. 并发设计
- 使用 `rayon` 进行并行下载与 Bucket 索引构建
- 使用 `flume` bounded channel 实现 core → CLI 的事件通信
- 避免引入 tokio：Hit 在 Phase 1-3 使用 blocking API 即可满足性能需求

### 4. 错误处理设计
- 使用 `thiserror` 定义统一错误类型（HitError 枚举）
- 使用 `anyhow` 进行错误传播
- 提供详细的错误信息和修复建议

### 5. 性能优化
- 使用 `bytes` crate 进行高效内存管理
- 使用 `lru` crate 实现缓存
- 使用 `memmap` 进行大文件操作
- 哈希校验与下载采用流式处理，避免大文件占用内存

### 6. Session/Context 模式（采纳 ref/Hok 设计）
- hit-common 中定义 `Session` 结构体，持有 `RefCell<Config>`、`OnceCell<EventBus>`、路径缓存
- 所有核心操作函数以 `session: &Session` 为首参数，通过 Session 访问配置和事件总线
- `Session::new()` 自动搜索配置文件路径，加载失败则使用默认配置
- 参考：`ref/Hok/crates/libscoop/src/session.rs`

### 7. Junction 回退策略（采纳 ref/Hok 设计）
- `symlink_dir` 先尝试 Windows 原生符号链接，失败后自动回退到 `junction::create`
- Junction 不需要管理员权限或开发者模式，适用于所有 Windows 版本
- `symlink_file` 失败后回退到 `std::fs::hard_link`
- 配置项 `no_junction: bool` 可禁用 junction 创建（兼容 Scoop 同名配置）
- 参考：`ref/Hok/crates/libscoop/src/internal/fs.rs:133-159`

### 8. EventBus 事件总线（采纳 ref/Hok 设计）
- 使用 `flume` crate 的 bounded channel（容量 20）实现双向事件传输
- hit-core 内部操作通过 `session.emitter()` 发送事件（下载进度、安装步骤、提示确认等）
- hit-cli 通过 `session.event_bus().receiver()` 接收事件并渲染 UI（进度条、彩色输出）
- `Event` 枚举使用 `#[non_exhaustive]` 保证向后兼容扩展
- 参考：`ref/Hok/crates/libscoop/src/event.rs`
