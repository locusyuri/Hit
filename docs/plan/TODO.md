# Hit 项目实现 TODO 清单

## 🎯 目标：将 Scoop 重构为 Rust，完全兼容 Scoop Bucket

> **文档定位**：本文档是 Phase 1-3 实现的 **权威任务清单**。
> Phase 4-5 的远期规划请参阅 [PROJECT.md](./PROJECT.md) 远期功能章节。
> 项目结构与模块划分请参阅 [PROJECT.md](./PROJECT.md)。
> 当其他文档与本文档在 Phase 1-3 范围内存在冲突时，以本文档为准。

---

## 架构设计概述

### Scoop PowerShell 模块 → Rust 模块映射

| Scoop 模块           | 功能描述                     | 对应 Rust 位置           |
|----------------------|------------------------------|--------------------------|
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
│       ├── compress/     # 解压（zip.rs, sevenz_rust2.rs, tar.rs, installer.rs）
│       ├── store/        # JSON 文件存储（db.json：load/save/migration/models）
│       ├── install/      # 安装流水线（controller.rs, transaction.rs, persist.rs, dependency.rs, hooks.rs）
│       ├── shim_mgmt/    # Shim 创建/移除/枚举
│       └── win/          # Windows 平台集成（#[cfg(windows)]）
│                         #   process.rs（sysinfo）, registry.rs（winreg）,
│                         #   fs.rs（junction + hard_link）,
│                         #   uac.rs（ShellExecuteW RunAs）, env.rs（WM_SETTINGCHANGE）
├── hit-shim/             # Shim 代理（独立 bin，~200KB）
│   └── src/
│       └── main.rs       # 读 db.json → 解析真 exe → spawn → 转发 stdio
├── hit-cli/              # CLI 入口（bin）
│   └── src/
│       ├── main.rs
│       ├── cli.rs        # clap 命令树（含 alias：i/s/u/rm/ls/st/b/c）
│       ├── progress.rs   # EventBus 订阅 → indicatif / colored 渲染
│       ├── tables.rs     # tabled 表格渲染（search/list/cache/bucket）
│       └── commands/     # install.rs, uninstall.rs, list.rs, search.rs,
│                         # info.rs, update.rs, bucket.rs, hold.rs, ...
└── hit-test-utils/       # 共享测试 fixture（仅 [dev-dependencies]）
    └── src/
        └── lib.rs        # mock_config(), sample_manifest(), temp_scoop_root()
```

#### 合并/拆分理由

| 原 10-crate 方案 | 5-crate 方案处置 | 理由 |
|-----------------|------------------|------|
| `hit-manifest`   | 并入 `hit-core::manifest` | 解析与 install 强耦合，独立 crate 会反复协商 API |
| `hit-bucket`     | 并入 `hit-core::bucket` | 操作仅被 install/search/update 调用，无需 pub 边界 |
| `hit-downloader` | 并入 `hit-core::download` + `hit-core::hash` | 单一调用点 |
| `hit-compression`| 并入 `hit-core::compress` | 单一调用点 |
| `hit-store`      | 并入 `hit-core::store` | JSON 文件 I/O 约 200 行，无需独立 crate |
| `hit-windows`    | 并入 `hit-core::win`（`#[cfg(windows)]`） | 被多处 hit-core 模块使用，避免循环依赖 |
| `hit-shim`       | **保留独立 bin** | 每个安装二进制对应一个 shim，必须控制体积 ~200KB |

### Rust 优势利用

| Rust 特性  | 应用场景             | 带来的好处                 |
| :--- | --- | --- |
| 类型安全   | Manifest 数据结构    | 编译期检查，避免运行时错误 |
| 所有权系统 | 文件操作、资源管理   | 自动资源清理，无内存泄漏   |
| 并发支持   | 并行下载、索引构建   | 提升性能                   |
| 零成本抽象 | 模块化设计           | 代码复用，无运行时开销     |
| 宏系统     | 命令行解析、错误处理 | 减少样板代码               |
| Cargo 生态 | 依赖管理、构建系统   | 简化开发流程               |

---

## Phase 1：Scoop 基础能力实现（3个月）

### 1.1 项目初始化与基础架构

| 序号  | 任务                                                                                              | 状态 | 依赖  |
| :--- | --- | :--: | --- |
| 1.1.1 | 创建 Cargo workspace 结构：根 Cargo.toml 添加 `[workspace]`，创建 `crates/`，按 5-crate 方案初始化子 crate 骨架（空 lib.rs / main.rs） | ✅ | -     |
| 1.1.2 | 配置 workspace 级 Cargo.toml：`[workspace.dependencies]`（serde, sonic-rs, thiserror, anyhow, tracing, flume）；`[profile.release]`（LTO, strip, opt-level=3, codegen-units=1） | ✅ | 1.1.1 |
| 1.1.3 | 完善 .gitignore：在现有 `/target`、`.codegraph/`、`.agents/graph.bin` 基础上追加 `Cargo.lock`（库 crate 不锁）、`*.swp`、`.vs/`、`*.pdb` | ✅ | -     |
| 1.1.4 | 创建项目文档结构（已完成：docs/ 含 10 个 .md 文件）                                               | ✅ | -     |
| 1.1.5 | 初始化 hit-common crate：定义 error.rs / config.rs / paths.rs / log.rs                            | ✅ | 1.1.1 |
| 1.1.6 | 定义 HitError 枚举（hit-common/src/error.rs）：thiserror derive，覆盖 IO、Manifest、Bucket、Download、Install、Config 等错误类别；对外暴露 `type Result<T> = std::result::Result<T, HitError>` | ✅ | 1.1.5 |
| 1.1.7 | 实现 Session/Context 模式（hit-common/src/session.rs）：Session 结构体持有 `RefCell<Config>`、`OnceCell<EventBus>`、路径缓存；所有核心操作以 `&Session` 为首参数；参考 `ref/Hok/crates/libscoop/src/session.rs` | ✅ | 1.1.5 |
| 1.1.8 | 实现 EventBus 事件总线（hit-common/src/event.rs）：flume bounded channel（容量 20）；定义 `Event` 枚举（DownloadProgress, ExtractStart, InstallStep, BucketUpdateProgress, PromptConfirm 等）；Session 通过 `event_bus()` 暴露 sender/receiver | ✅ | 1.1.5 |
| 1.1.9 | 初始化 hit-test-utils crate 骨架：dev-dependency 库，含 mock_config()、sample_manifest()、temp_scoop_root() 辅助函数 | ✅ | 1.1.1 |

### 1.2 hit-core/manifest：Scoop Manifest 格式兼容解析

| 序号  | 任务                                                                    | 状态 | 依赖  |
| :--- | --- | :--: | --- |
| 1.2.1 | 分析 Scoop Manifest JSON Schema（参考 `ref/Main/` 真实 manifest）       | ✅ | -     |
| 1.2.2 | 定义 Manifest 数据结构（hit-core/src/manifest/schema.rs，serde derive）：完整反序列化 Scoop 字段（architecture/bin/env_set/persist/depends/pre_install/post_install/pre_uninstall/shortcuts/checkver/autoupdate 等）；**Hit 扩展字段**（alias, dependencies, health_check, mirrors 等）在此阶段声明但标记 `#[serde(default, skip_serializing_if)]` 跳过解析 | ✅ | 1.2.1 |
| 1.2.3 | 实现变量替换引擎（hit-core/src/manifest/variables.rs）：支持 `$version`, `$architecture`, `$url`, `$dir`, `$appdir`, `$scoopdir`, `$persist_dir` 等 Scoop 内置变量；递归替换 url、hash、bin、env_set 中的变量引用 | ✅ | 1.2.2 |
| 1.2.4 | 实现 Manifest 解析器（hit-core/src/manifest/parser.rs）                 | ✅ | 1.2.2 |
| 1.2.5 | 实现 Manifest 验证器（hit-core/src/manifest/validator.rs）              | ✅ | 1.2.2 |
| 1.2.6 | 支持 Scoop 特殊字段（architecture、depends、persist、pre/post_install） | ✅ | 1.2.2 |
| 1.2.7 | 编写 Manifest 解析单元测试（hit-core/tests/manifest_test.rs）：使用 `ref/Main/` 中的真实 manifest JSON 作为测试 fixtures，覆盖 git.json、python.json、7zip.json 等代表性清单 | ✅ | 1.2.2-1.2.5 |

### 1.3 hit-core/bucket：Scoop Bucket 仓库支持

| 序号  | 任务                                                                                   | 状态 | 依赖  |
| :--- | --- | :--: | --- |
| 1.3.1 | 实现 Git 仓库克隆（hit-core/src/bucket/git_client.rs，gix crate）：clone with progress；支持 proxy 配置（从 Session config 读取）；进度通过 EventBus 发送 `BucketUpdateProgress` 事件；**默认浅克隆（depth=1）**，支持 `--full-clone` 切换 | ✅ | 1.1.7, 1.1.8 |
| 1.3.2 | 实现 Bucket 更新（git pull）                                                           | ✅ | 1.3.1 |
| 1.3.3 | 实现 Bucket 列表管理                                                                   | ✅ | 1.3.1 |
| 1.3.4 | 解析 bucket.json 元数据                                                                | ✅ | 1.2.2 |
| 1.3.5 | 默认添加 Scoop 官方 bucket（main, extras）                                             | ✅ | 1.3.1 |
| 1.3.6 | 构建软件索引（hit-core/src/bucket/index.rs）：遍历 bucket 目录下所有 .json 文件（rayon par_bridge 并行），解析为 Manifest 摘要（name + version + description），存入内存 `HashMap<String, Vec<PackageSummary>>` | ✅ | 1.2.2 |

### 1.4 hit-core/download：下载与哈希校验

| 序号  | 任务                                                                                   | 状态 | 依赖  |
| :--- | --- | :--: | --- |
| 1.4.1 | 实现 HTTP 下载器（hit-core/src/download/http.rs，reqwest blocking client）：支持 proxy 配置（Session config）；下载进度通过 EventBus 发送 `DownloadProgress` 事件（已下载字节/总字节/速率） | ✅ | 1.1.7, 1.1.8 |
| 1.4.2 | 实现缓存管理（hit-core/src/download/cache.rs）                                         | ✅ | 1.4.1 |
| 1.4.3 | 实现哈希校验（hit-core/src/hash/mod.rs）：支持 sha256、sha512、blake3；流式计算（避免大文件内存问题）；校验失败返回 `HashMismatch` 错误（含 expected/actual/path 上下文） | ✅ | -     |

> 1.4.x 说明：断点续传、GitHub API 下载延后到 Phase 2+；MVP 阶段失败即重下（参考 Hok）。

### 1.5 hit-core/compress：解压模块

| 序号  | 任务                                                                                   | 状态 | 依赖 |
| :--- | --- | :--: | --- |
| 1.5.1 | 实现 ZIP 解压（hit-core/src/compress/zip.rs，zip crate）                               | ✅ | -    |
| 1.5.2 | 实现 7z 解压（hit-core/src/compress/sevenz.rs，sevenz-rust2）                           | ✅ | -    |
| 1.5.3 | 实现 TAR 解压（hit-core/src/compress/tar.rs，tar + flate2）                            | ✅ | -    |
| 1.5.4 | 支持安装程序处理（hit-core/src/compress/installer.rs）：NSIS 静默安装（/S flag）、Inno Setup 静默安装（/VERYSILENT）、MSI 安装（msiexec /qn）；通过 Session config 控制是否使用 lessmsi 提取 MSI | ✅ | -    |

### 1.6 hit-core/win：Windows 平台集成

| 序号  | 任务                                                                                   | 状态 | 依赖  |
| :--- | --- | :--: | --- |
| 1.6.1 | 实现进程管理（hit-core/src/win/process.rs，sysinfo crate）：检测运行中的进程；安装前检查目标进程是否在运行；支持优雅终止和强制终止 | ✅ | -     |
| 1.6.2 | 实现注册表操作（hit-core/src/win/registry.rs，winreg crate）：读写 `HKCU\Environment`（PATH 管理）；读写 `HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall`（已安装软件检测） | ✅ | -     |
| 1.6.3 | 实现文件系统操作（hit-core/src/win/fs.rs）：**Junction + HardLink 单一策略**（与 Scoop 原版一致）—— 创建 `current` 目录链接时使用 `junction::create`（无需管理员或开发者模式）；`persist/` 下文件链接使用 `std::fs::hard_link`；当 `no_junction=true` 时跳过 `current` 链接创建，shim 直接指向具体版本路径；实现 remove_junction / remove_hardlink 函数（按链接类型清理） | ✅ | -     |
| 1.6.4 | 实现 UAC 提权（hit-core/src/win/uac.rs）：`is_admin()` 检测当前是否管理员；`elevate_self()` 使用 ShellExecuteW + RunAs verb 重新以管理员启动自身；仅在写系统级路径（如 `Program Files`）或注册表 `HKLM` 时触发，链接创建本身不触发提权 | ✅ | -     |
| 1.6.5 | 实现环境变量管理（hit-core/src/win/env.rs）：修改用户级 PATH（添加/移除 shims 目录）；广播 `WM_SETTINGCHANGE` 消息通知其他进程刷新环境变量（使用 SendMessageTimeoutW） | ✅ | -     |
| 1.6.6 | 实现 `no_junction` 配置支持：Config 中添加 `no_junction: Option<bool>` 字段；当 `no_junction=true` 时，跳过 `current` 目录 junction 链接创建，shim 直接指向具体版本路径（兼容 Scoop 同名配置项） | ✅ | 1.1.5, 1.6.3 |

### 1.7 hit-shim：Shim 代理机制（独立 bin）

| 序号  | 任务                                                                                   | 状态 | 依赖        |
| :--- | --- | :--: | --- |
| 1.7.1 | 创建 hit-shim 独立 binary crate：`crates/hit-shim/Cargo.toml` 零外部依赖（仅 std）；`[profile.release]` 单独优化体积 | ✅ | 1.1.1       |
| 1.7.2 | 实现命令转发逻辑：读取同名 `.shim` sidecar 文件获取目标路径（兼容 Scoop 格式）；使用 `std::process::Command` 启动真实进程；完整转发 stdin/stdout/stderr 和所有命令行参数 | ✅ | 1.6.3       |
| 1.7.3 | 读取 `.shim` 文件解析目标路径（hit-shim/src/parse.rs）：解析 `path = "..."` 和 `args = ...` 格式；根据 shim 自身路径推导 `.shim` 文件位置（`exe.with_extension("shim")`） | ✅ | 1.1.5       |
| 1.7.4 | 启动真实进程并转发 stdin/stdout/stderr：Windows 下使用 `CREATE_NEW_PROCESS_GROUP` 标志；正确处理 Ctrl+C 信号传播；返回子进程 exit code | ✅ | 1.6.1       |
| 1.7.5 | 最小化 shim 体积（~209KB release binary，零外部依赖 + LTO + strip + opt-level "s"） | ✅ | -           |

### 1.8 hit-core/install：核心安装逻辑

| 序号  | 任务                                                                                   | 状态 | 依赖                |
| :--- | --- | :--: | --- |
| 1.8.0 | 集成 Session 与 install 流程：所有安装/卸载函数签名以 `session: &Session` 为首参数；通过 `session.event_bus()` 发送安装步骤进度事件（PackageResolveStart, PackageDownloadStart, PackageExtractStart, PackageCommitStart, PackageSyncDone 等） | ✅ | 1.1.7, 1.1.8        |
| 1.8.1 | 实现事务管理器（hit-core/src/install/transaction.rs）：RAII 模式管理事务状态            | ✅ | -                   |
| 1.8.2 | 创建临时事务目录（tempfile crate）                                                     | ✅ | 1.6.3               |
| 1.8.3 | 实现原子移动（rename，使用 Windows `MoveFileEx` API）                                  | ✅ | 1.6.3               |
| 1.8.4 | 实现失败回滚机制：删除临时目录，保留已安装状态不变                                     | ✅ | 1.8.1               |
| 1.8.5 | 实现安装流程控制器（hit-core/src/install/controller.rs）：编排完整安装流水线：解析 manifest → 解析依赖 → 下载 → 校验哈希 → 解压 → 创建 shim → 设置 persist → 更新 db.json → 执行 post_install 脚本；每步通过 EventBus 发送进度事件 | ✅ | 所有上游模块        |
| 1.8.6 | 实现 Persist 持久化机制（hit-core/src/install/persist.rs）：使用 junction（目录）+ hard_link（文件）将 app 目录中的配置文件/目录链接到 `~/.hit/persist/<app>/`（依赖 1.6.3，与 Scoop 原版一致）；卸载时保留 persist 目录；版本切换时更新链接指向 | ✅ | 1.6.3               |
| 1.8.7 | 实现依赖解析器（hit-core/src/install/dependency.rs）：解析 Manifest 的 depends 字段    | ✅ | 1.2.2               |

### 1.9 hit-core/store：数据存储

| 序号  | 任务                                                                                   | 状态 | 依赖     |
| :--- | --- | :--: | --- |
| 1.9.1 | 实现 JSON 文件存储（hit-core/src/store/mod.rs）：定义 `Db` 结构体（对应 db.json），使用 sonic-rs 序列化/反序列化；实现 `Db::load()` / `Db::save()` 原子写入（写临时文件后 rename） | ✅ | sonic-rs |
| 1.9.2 | 定义数据模型（hit-core/src/store/models.rs）：`InstalledPackage`（version, bucket, install_date, shims, persist_files, held）、`BucketInfo`（name, url, last_update）、`HitConfig`（proxy, mirror, aria2_enabled, no_junction, root_path） | ✅ | -        |
| 1.9.3 | 实现数据库迁移（hit-core/src/store/migration.rs）：db.json 包含 `version` 字段；加载时检查版本号，自动执行迁移逻辑（字段重命名、默认值填充） | ✅ | -        |
| 1.9.4 | 实现安装记录管理                                                                       | ✅ | 1.9.1    |

### 1.10 hit-cli：命令行接口

#### 1.10.1 CLI 框架搭建

| 序号     | 任务                                                                                   | 状态 | 依赖               |
| :--- | --- | :--: | --- |
| 1.10.1.1 | 使用 clap 定义命令结构（hit-cli/src/cli.rs）：`#[derive(Parser)]` 与 `#[derive(Subcommand)]` 定义子命令枚举；**添加命令简写别名**：`install` 加 `#[clap(alias = "i")]`、`search` 加 `alias = "s"`、`update` 加 `alias = "u"`、`uninstall` 加 `alias = "rm"`、`list` 加 `alias = "ls"`、`status` 加 `alias = "st"`、`bucket` 加 `alias = "b"`、`cleanup` 加 `alias = "c"`（参考 `ref/Hok/src/cmd/mod.rs`） | ✅ | -                  |
| 1.10.1.2 | 实现命令路由分发：各子命令模块接收 `&Session` 参数                                      | ✅ | 1.10.1.1, 1.1.7    |
| 1.10.1.3 | 添加进度条和彩色输出（indicatif, colored）                                             | ✅ | indicatif, colored |
| 1.10.1.4 | 集成 EventBus 进度渲染（hit-cli/src/progress.rs）：从 Session event_bus receiver 接收 Event；根据事件类型更新 indicatif ProgressBar（下载进度条、解压状态、安装步骤）；PromptConfirm 事件触发用户确认对话框 | ✅ | 1.1.8, 1.10.1.3    |

#### 1.10.2 install 命令

| 序号     | 任务                      | 状态 | 依赖              |
| :--- | --- | :--: | --- |
| 1.10.2.1 | 解析软件名和版本约束      | ✅ | hit-core/bucket   |
| 1.10.2.2 | 搜索 Bucket 获取 Manifest | ✅ | hit-core/bucket   |
| 1.10.2.3 | 调用 hit-core 执行安装    | ✅ | hit-core/install  |

#### 1.10.3 uninstall 命令

| 序号     | 任务                   | 状态 | 依赖             |
| :--- | --- | :--: | --- |
| 1.10.3.1 | 查找已安装软件         | ✅ | hit-core/store   |
| 1.10.3.2 | 调用 hit-core 执行卸载 | ✅ | hit-core/install |

#### 1.10.4 list 命令

| 序号     | 任务                     | 状态 | 依赖             |
| :--- | --- | :--: | --- |
| 1.10.4.1 | 读取数据库中的已安装列表 | ✅ | hit-core/store   |
| 1.10.4.2 | 格式化输出（表格形式）   | ✅ | -                |

#### 1.10.5 search 命令

| 序号     | 任务               | 状态 | 依赖            |
| :--- | --- | :--: | --- |
| 1.10.5.1 | 遍历 Bucket 索引   | ✅ | hit-core/bucket |
| 1.10.5.2 | 实现关键词模糊匹配 | ✅ | -               |
| 1.10.5.3 | 显示匹配结果       | ✅ | -               |

#### 1.10.6 info 命令

| 序号     | 任务               | 状态 | 依赖            |
| :--- | --- | :--: | --- |
| 1.10.6.1 | 查找软件 Manifest  | ✅ | hit-core/bucket |
| 1.10.6.2 | 格式化显示软件详情 | ✅ | -               |

#### 1.10.7 update 命令

| 序号     | 任务                 | 状态 | 依赖                            |
| :--- | --- | :--: | --- |
| 1.10.7.1 | 更新所有 Bucket      | ✅ | hit-core/bucket                 |
| 1.10.7.2 | 检查已安装软件新版本 | ✅ | hit-core/store, hit-core/bucket |
| 1.10.7.3 | 执行软件升级         | ✅ | hit-core/install                |

#### 1.10.8 bucket 命令

| 序号     | 任务                            | 状态 | 依赖            |
| :--- | --- | :--: | --- |
| 1.10.8.1 | bucket add - 添加新 Bucket      | ✅ | hit-core/bucket |
| 1.10.8.2 | bucket remove - 移除 Bucket     | ✅ | hit-core/bucket |
| 1.10.8.3 | bucket list - 列出所有 Bucket   | ✅ | hit-core/bucket |
| 1.10.8.4 | bucket update - 更新指定 Bucket | ✅ | hit-core/bucket |
| 1.10.8.5 | bucket create - 交互式创建 Bucket：初始化目录结构、生成 bucket.json、可选配合 `gh` CLI 推送至 GitHub | 📋 | hit-core/bucket |

### 1.11 首次启动引导

| 序号   | 任务                                                                                   | 状态 | 依赖          |
| ------ | -------------------------------------------------------------------------------------- | ---- | ------------- |
| 1.11.1 | 检测首次运行：`config.json` 不存在时标记为首次启动                                      | ✅ | hit-common/config |
| 1.11.2 | 实现欢迎界面：提供快速开始（导入 main + extras + versions）、自定义选择、跳过三选项     | ✅ | hit-cli |
| 1.11.3 | 快速开始模式：自动添加 Scoop 官方 bucket（main, extras, versions）                     | ✅ | 1.3.5 |

### 1.12 基础测试框架

| 序号   | 任务                                                                                   | 状态 | 依赖                 |
| :--- | --- | :--: | --- |
| 1.12.1 | 设置单元测试框架                                                                       | ✅ | -                    |
| 1.12.2 | 编写 Manifest 解析测试                                                                 | ✅ | hit-core/manifest    |
| 1.12.3 | 编写 Bucket 管理测试                                                                   | ✅ | hit-core/bucket      |
| 1.12.4 | 编写安装卸载集成测试：使用 hit-test-utils 创建临时 Scoop root；测试完整安装流水线（manifest → download → extract → shim → db.json 更新）；测试卸载清理；测试安装失败回滚（模拟下载中断、哈希不匹配） | ✅ | hit-core/install     |
| 1.12.5 | 编写 EventBus 事件流测试：验证安装流程中事件按正确顺序发送（ResolveStart → DownloadStart → DownloadProgress... → ExtractStart → CommitStart → SyncDone） | ✅ | 1.1.8                |
| 1.12.6 | 编写 junction / hard_link 测试：验证 `current` 目录通过 `junction::create` 正确创建；验证 persist 文件通过 `std::fs::hard_link` 正确创建；验证 `no_junction` 配置生效时跳过 `current` 链接创建、shim 直接指向版本路径 | ✅ | 1.6.3, 1.6.6         |

---

## Phase 2：Scoop 高级功能实现（2个月）

### 2.1 高级命令实现

| 序号  | 任务                                                                                   | 状态 | 依赖                            |
| :--- | --- | :--: | --- |
| 2.1.1 | `hit reset` - 版本切换                                                                 | ✅ | hit-core/install                |
| 2.1.2 | `hit cleanup` - 清理旧版本                                                             | ✅ | hit-core/store                  |
| 2.1.3 | `hit cache` - 缓存管理                                                                 | ✅ | hit-core/download               |
| 2.1.4 | `hit status` - 状态检查                                                                | ✅ | hit-core/store, hit-core/bucket |
| 2.1.5 | `hit home` - 打开主页                                                                  | ✅ | hit-core/manifest               |
| 2.1.6 | `hit uninstall --purge` - 彻底卸载                                                     | ✅ | hit-core/install                |
| 2.1.13| 卸载 Hit 自身（脚本方案）— 提供 `scripts\uninstall-env.ps1`（模式1：仅清理环境变量，保留已安装软件）和 `scripts\uninstall-hit.ps1`（模式2：彻底删除全部内容），与安装脚本 `install-hit.ps1` 对称，无需 CLI 子命令 | ✅ | hit-common/config, hit-core/win/env |
| 2.1.7 | `hit which` - 查找命令对应的 shim 路径和真实 exe 路径                                  | ✅ | hit-shim                        |
| 2.1.8 | `hit prefix` - 显示安装路径                                                            | ✅ | hit-core/store                  |
| 2.1.9 | `hit hold <pkg>` - 版本锁定：在 db.json 的 InstalledPackage 中设置 `held: true` 字段；被 hold 的包在 `hit update` 时跳过升级；参考 Hok 的 `operation::package_hold` 实现 | ✅ | hit-core/store                  |
| 2.1.10| `hit unhold <pkg>` - 解除版本锁定：将 `held` 字段设回 `false`                          | ✅ | 2.1.9                           |
| 2.1.11| `hit list` 增加 held 标记：已 hold 的包在 list 输出中显示 `[held]` 标记                | ✅ | 2.1.9                           |
| 2.1.12| `hit config` 子命令：`hit config list` 显示当前配置；`hit config set <key> <value>` 修改配置（proxy, no_junction, mirror 等）；参考 Hok 的 config.rs `set()` 方法 | ✅ | hit-common/config               |

### 2.2 依赖解析增强

| 序号  | 任务                          | 状态 | 依赖              |
| :--- | --- | :--: | --- |
| 2.2.1 | 解析 Manifest 的 depends 字段 | ✅ | hit-core/manifest |
| 2.2.2 | 构建依赖图                    | ✅ | petgraph          |
| 2.2.3 | 检测循环依赖                  | ✅ | 2.2.2             |
| 2.2.4 | 实现依赖安装顺序              | ✅ | hit-core/install  |

### 2.3 Bucket 全局索引

| 序号  | 任务                              | 状态 | 依赖              |
| :--- | --- | :--: | --- |
| 2.3.1 | 构建内存索引（软件名 → 版本列表） | ✅ | hit-core/bucket   |
| 2.3.2 | 实现优先级系统                    | ✅ | hit-core/bucket   |
| 2.3.3 | 安装时自动选择最佳版本            | ✅ | hit-core/install  |

---

## Phase 3：Hit 增强功能（3个月）

### 3.1 健康检查

| 序号  | 任务                   | 状态 | 依赖              |
| :--- | --- | :--: | --- |
| 3.1.1 | 实现文件完整性检查     | ✅ | hit-core/download |
| 3.1.2 | 检查 Shim 指向是否正确 | ✅ | hit-shim          |
| 3.1.3 | 实现自动修复功能       | ✅ | hit-core/install  |

### 3.2 镜像源管理

| 序号  | 任务           | 状态 | 依赖              |
| :--- | --- | :--: | --- |
| 3.2.1 | 配置多镜像源   | 📋 | hit-common        |
| 3.2.2 | 实现速度测试   | 📋 | hit-core/download |
| 3.2.3 | 自动选择最快源 | 📋 | 3.2.2             |

### 3.3 CLI 表格渲染（tabled）

> 原 ratatui 交互式搜索（si 命令）已移除，改用 tabled 为 search/list/cache/bucket 命令提供自动表格渲染。

| 序号  | 任务                                                                                   | 状态 | 依赖    |
| :--- | --- | :--: | --- |
| 3.3.1 | 新增 `tables.rs` 模块：使用 `#[derive(Tabled, Clone)]` 声明式定义 SearchRow、ListRow、CacheRow、BucketRow 4 种表格行类型 | ✅ | tabled |
| 3.3.2 | 统一 search/list/cache/bucket 命令的表格输出：调用 `tables::print_*_table()` 函数渲染表格，移除手写 `println!` 格式化逻辑 | ✅ | 3.3.1  |
| 3.3.3 | 移除 ratatui/crossterm/comfy-table 依赖，删除 `tui.rs` 和 `si` 命令                    | ✅ | -      |

### 3.4 统一色彩输出（owo-colors）

> 使用 owo-colors 替代 colored，配合 anstyle/supports-color/is-terminal/terminal-size 实现统一的 CLI 色彩输出系统。保留 tabled（表格）和 indicatif（进度条）专用库。

| 序号  | 任务                                                                                   | 状态 | 依赖        |
| :--- | --- | :--: | --- |
| 3.4.1 | 添加 owo-colors/anstyle/supports-color/is-terminal/terminal-size 依赖到 hit-cli Cargo.toml | ✅ | owo-colors |
| 3.4.2 | 定义统一色彩主题：成功绿色 ✔、错误红色 ✘、警告黄色 ⚠、步骤青色 ▶、表格表头青色粗体 | ✅ | 3.4.1      |
| 3.4.3 | 新增 `output.rs` 模块：封装语义化输出函数（success/error/warn/info/step），统一替代散落的 `.green()`/`.red()` 调用 | ✅ | 3.4.1      |
| 3.4.4 | 修改 `progress.rs`：使用 owo-colors 替代 colored 渲染彩色文本，保留 indicatif 用于进度条 | ✅ | 3.4.1      |
| 3.4.5 | 修改 `main.rs`：使用 `output::error()` 统一错误输出样式 | ✅ | 3.4.3      |
| 3.4.6 | 修改各命令文件（search.rs, list.rs, info.rs, status.rs）：补充空状态消息和标签的颜色渲染 | ✅ | 3.4.3      |
| 3.4.7 | 修改剩余命令文件（install.rs, update.rs, doctor.rs, cleanup.rs, hold.rs, unhold.rs, cache.rs, config.rs, home.rs, which.rs, reset.rs, uninstall.rs, welcome.rs）：将 colored 替换为 owo-colors | ✅ | 3.4.3      |
| 3.4.8 | 移除 colored 依赖，清理相关代码 | ✅ | 3.4.4-3.4.7 |

---

## 远期展望（Phase 4-5）

> 以下功能不在 Phase 1-3 范围内，详见 [PROJECT.md](./PROJECT.md) 远期功能章节。

| 功能领域         | 简述                                  | ROADMAP   |
| :--- | --- | --- |
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

### 7. 链接策略：Junction + HardLink（与 Scoop 原版一致）
- 目录链接（如 `apps/<app>/current`、`persist/` 下目录）使用 `junction::create`，无需管理员权限或开发者模式
- 文件链接（如 `persist/` 下文件）使用 `std::fs::hard_link`
- 不引入符号链接（symlink）：Scoop 全代码库零 symlink 使用，且 symlink 需管理员或开发者模式，普通用户体验差
- 配置项 `no_junction: bool` 可禁用 `current` 目录链接创建（兼容 Scoop 同名配置，此时 shim 直接指向版本路径）
- 不提供"符号链接/自动回退/目录交接点"多模式可选配置，避免复杂度与兼容性风险

### 8. EventBus 事件总线（采纳 ref/Hok 设计）
- 使用 `flume` crate 的 bounded channel（容量 20）实现双向事件传输
- hit-core 内部操作通过 `session.emitter()` 发送事件（下载进度、安装步骤、提示确认等）
- hit-cli 通过 `session.event_bus().receiver()` 接收事件并渲染 UI（进度条、彩色输出）
- `Event` 枚举使用 `#[non_exhaustive]` 保证向后兼容扩展
- 参考：`ref/Hok/crates/libscoop/src/event.rs`
