# Hit - Windows 软件包管理器项目结构

## 📁 项目概览

Hit 是一个用 Rust 编写的 Windows 软件包管理器，**完全兼容 Scoop 的软件仓库（Bucket）**，核心设计理念：
- **Scoop 兼容**：直接复用 Scoop 的所有 bucket，无需重新维护软件清单
- **零污染**：所有软件安装在用户目录，无需管理员权限
- **Shim 代理**：通过轻量级代理实现 PATH 不污染
- **版本管理**：支持 JDK、Python、Node.js 等 SDK 的多版本切换
- **深度卸载**：集成类似 Geek Uninstaller 的残留扫描清理功能
- **便携化**：解压即用，卸载干净
- **高性能**：Rust 编写，启动快、内存占用低
- **安全性**：软件完整性校验，安全扫描集成

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
│   ├── PROJECT_STRUCTURE.md      # 项目结构说明（本文件）
│   ├── CODING_GUIDELINES.md      # 编码规范
│   ├── TECH_STACK.md             # 技术栈清单
│   ├── DEV_FLOW.md               # 开发流程
│   ├── MANIFEST_FORMAT.md        # Manifest 清单格式
│   ├── WINDOWS_NOTES.md          # Windows 注意事项
│   ├── ROADMAP.md                # 路线图与新增功能详解
│   ├── FEATURES.md               # 功能特性清单与设计理念
│   ├── REFERENCE_PROJECTS.md     # 参考项目
│   └── TODO.md                   # 实现任务清单（Phase 1-3 权威）
│
├── crates/                       # Rust 工作区子模块（5-crate 方案，详见 [TODO.md](./TODO.md)）
│   │
│   ├── hit-common/               # 基础类型库（lib）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # 库入口
│   │       ├── error.rs          # HitError 枚举（thiserror）
│   │       ├── config.rs         # Config 结构体（sonic-rs 序列化）
│   │       ├── paths.rs          # Scoop 兼容路径计算
│   │       ├── log.rs            # tracing 日志初始化
│   │       ├── session.rs        # Session/Context 模式（参考 ref/Hok/libscoop/session.rs）
│   │       └── event.rs          # EventBus + Event 枚举（flume bounded channel）
│   │
│   ├── hit-core/                 # 核心业务逻辑库（lib）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # 库入口
│   │       ├── manifest/         # Manifest 解析（schema.rs, parser.rs, validator.rs, variables.rs）
│   │       ├── bucket/           # Bucket 管理（git_client.rs, index.rs, registry.rs）
│   │       ├── download/         # 下载与缓存（http.rs, cache.rs）
│   │       ├── hash/             # 哈希校验（sha256/sha512/blake3，流式计算）
│   │       ├── compress/         # 解压（zip.rs, sevenz.rs, tar.rs, installer.rs）
│   │       ├── store/            # JSON 文件存储（db.json：load/save/migration/models）
│   │       ├── install/          # 安装流水线（controller.rs, transaction.rs, persist.rs, dependency.rs, hooks.rs）
│   │       ├── shim_mgmt/        # Shim 创建/移除/枚举
│   │       └── win/              # Windows 平台集成（#[cfg(windows)]）
│   │                             #   process.rs（sysinfo）, registry.rs（winreg）,
│   │                             #   fs.rs（symlink + junction fallback）,
│   │                             #   uac.rs（ShellExecuteW RunAs）, env.rs（WM_SETTINGCHANGE）
│   │
│   ├── hit-shim/                 # Shim 代理可执行文件（独立 bin，~200KB）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs           # 读 db.json → 解析真 exe → spawn → 转发 stdio
│   │
│   ├── hit-cli/                  # CLI 入口（bin）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # 程序入口
│   │       ├── cli.rs            # clap 命令树（含 alias：i/s/u/rm/ls/st/b/c）
│   │       ├── progress.rs       # EventBus 订阅 → indicatif / colored 渲染
│   │       ├── tui.rs            # ratatui 交互搜索（Phase 3）
│   │       └── commands/         # install.rs, uninstall.rs, list.rs, search.rs,
│   │                             # info.rs, update.rs, bucket.rs, hold.rs, ...
│   │
│   └── hit-test-utils/           # 共享测试 fixture（仅 [dev-dependencies]）
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs            # mock_config(), sample_manifest(), temp_scoop_root()
│
│   > **远期规划 crate**（不在 Phase 1-3 范围，详见 ROADMAP.md Phase 4-5）：
│   > - `hit-uninstaller` —— 深度卸载模块（注册表扫描、残留清理、进程终止）
│   > - `hit-plugin` —— Lua 脚本插件系统
│
├── ref/                          # 参考源码
│   ├── Scoop/                    # 原版 Scoop PowerShell 源码
│   ├── Main/                     # Scoop 官方 Main Bucket
│   └── Hok/                      # Rust 实现的 Scoop 替代品
│
├── buckets/                      # 默认 Bucket 仓库（Git 子模块或独立仓库）
│   ├── main/                     # 主仓库（命令行工具）
│   │   ├── bucket.json           # Bucket 元信息
│   │   └── manifest/             # 软件清单目录
│   │       ├── git.json
│   │       ├── curl.json
│   │       └── ...
│   ├── extras/                   # 扩展仓库（GUI 应用）
│   │   ├── bucket.json
│   │   └── manifest/
│   │       ├── vscode.json
│   │       ├── chrome.json
│   │       └── ...
│   ├── versions/                 # 历史版本仓库
│   ├── java/                     # JDK 专用仓库
│   └── sdk/                      # SDK 专用仓库（Python, Node.js 等）
│
├── scripts/                      # 辅助脚本
│   ├── build.ps1                 # Windows 构建脚本
│   ├── test.ps1                  # 测试脚本
│   ├── release.ps1               # 发布打包脚本
│   └── checkver.ps1              # 版本检查脚本（PowerShell）
│
├── tests/                        # 集成测试
│   ├── install_test.rs
│   ├── uninstall_test.rs
│   ├── shim_test.rs
│   └── sdk_version_test.rs
│
└── assets/                       # 静态资源
    ├── icons/                    # 图标文件
    ├── templates/                # 模板文件
    │   └── manifest_template.json # Manifest 生成模板
    └── completions/              # Shell 补全脚本
        ├── hit.bash
        ├── hit.zsh
        └── hit.ps1
```

---

## 🔑 核心模块说明

> 模块架构以 [TODO.md](./TODO.md) 为 Phase 1-3 权威清单；本节提供设计意图与职责概览。

### 1. **hit-common** - 基础类型库
- **职责**：跨 crate 共享的基础类型、配置、路径、日志、Session 与 EventBus
- **关键组件**：
  - **HitError**：`thiserror` 统一错误枚举，覆盖 IO / Manifest / Bucket / Download / Install / Config 等类别
  - **Config**：`sonic-rs` 序列化的用户配置（proxy, mirror, no_junction, link_mode 等）
  - **paths**：Scoop 兼容路径计算（root_path, cache_path, apps_path, shims_path, persist_path）
  - **Session/Context**：参考 `ref/Hok/libscoop/session.rs`，持有 `RefCell<Config>`、`OnceCell<EventBus>`、路径缓存；所有核心操作以 `&Session` 为首参数
  - **EventBus**：`flume` bounded channel（容量 20），定义 `Event` 枚举（DownloadProgress, ExtractStart, InstallStep, BucketUpdateProgress, PromptConfirm 等）
  - **log**：`tracing` 日志初始化

### 2. **hit-core** - 核心业务逻辑库
- **职责**：实现软件安装、卸载、版本管理等所有核心功能
- **内部模块**：
  - **manifest/**：Manifest 解析（schema / parser / validator / variables）
  - **bucket/**：Bucket git 仓库克隆/更新/索引（git2 + rayon 并行）
  - **download/**：HTTP 下载与缓存（reqwest blocking），进度通过 EventBus 上报
  - **hash/**：sha256 / sha512 / blake3 流式哈希校验
  - **compress/**：ZIP / 7z / TAR 解压 + NSIS / Inno Setup / MSI 静默安装
  - **store/**：JSON 文件存储（db.json 原子写入 + 版本迁移）
  - **install/**：安装流水线（controller / transaction / persist / dependency / hooks）
  - **shim_mgmt/**：Shim 创建/移除/枚举
  - **win/**（`#[cfg(windows)]`）：进程检测（sysinfo）、注册表（winreg）、文件系统（symlink + junction fallback）、UAC（ShellExecuteW RunAs）、环境变量（WM_SETTINGCHANGE）
- **关键设计**：
  - **事务性安装**：RAII 管理事务状态，`MoveFileEx` 原子操作，失败回滚通过删除临时目录实现
  - **Junction fallback**：symlink_dir 失败后回退到 `junction::create`（无需管理员或开发者模式），参考 `ref/Hok/libscoop/internal/fs.rs:133-159`
  - **依赖解析**：`petgraph` 构建依赖图，检测循环依赖

### 3. **hit-shim** - Shim 代理程序
- **职责**：独立的轻量级 exe，负责命令转发
- **工作流程**：
  1. 接收命令行参数
  2. 读取 `~/.hit/db.json` 获取当前激活版本
  3. 拼接真实路径：`~/.hit/apps/<package>/<version>/bin/<exe>`
  4. 启动真实进程并转发 stdin/stdout/stderr
  5. 返回退出码
- **特点**：体积 ~200KB（仅依赖 hit-common + sonic-rs，不引入 heavy deps），启动速度快

### 4. **hit-cli** - 命令行界面
- **职责**：用户交互入口，解析子命令并调用 hit-core
- **关键技术**：`clap`（参数解析，含 alias：`hit i/s/u/rm/ls/st/b/c`）、`ratatui`（TUI 界面，Phase 3）、`indicatif`（进度条）、`colored`（彩色输出）
- **子命令（Phase 1-3）**：
  - `hit install <package>` / `i` - 安装软件
  - `hit uninstall <package>` / `rm` - 卸载软件
  - `hit update [package]` / `u` - 更新软件
  - `hit search <keyword>` / `s` - 搜索软件
  - `hit list` / `ls` - 列出已安装软件
  - `hit info <package>` - 查看软件详情
  - `hit status` / `st` - 状态检查
  - `hit reset <package> <version>` - 切换版本
  - `hit cleanup` / `c` - 清理旧版本释放空间
  - `hit cache` - 缓存管理
  - `hit home <package>` - 打开主页
  - `hit which <command>` - 查找命令 shim 与真实 exe 路径
  - `hit prefix <package>` - 显示安装路径
  - `hit hold <pkg>` / `hit unhold <pkg>` - 版本锁定
  - `hit config list/set` - 配置管理
  - `hit bucket add/remove/list/update` - Bucket 管理（alias `b`）
  - `hit check` - 健康检查（Phase 3）
  - `hit repair <package>` - 修复损坏（Phase 3）
  - `hit mirror add/list/refresh` - 镜像源管理（Phase 3）

### 5. **hit-test-utils** - 共享测试 fixture
- **职责**：为 hit-core / hit-cli 的集成测试提供 dev-dependency 工具库
- **组件**：`mock_config()`、`sample_manifest()`、`temp_scoop_root()` 等辅助函数
- **注意**：仅作为 `[dev-dependencies]` 引入，不会进入 release 二进制

### 远期规划模块（不在 Phase 1-3 范围，详见 ROADMAP.md Phase 4-5）

- **hit-uninstaller** —— 深度卸载模块：注册表扫描、残留文件清理（`walkdir` + `rayon`）、进程强制终止；用于 `hit force-uninstall` 命令（针对非 Hit 安装的软件）
- **hit-plugin** —— 插件系统：`mlua` Lua 脚本引擎、插件钩子（安装前/后、卸载前/后）
- **hit-core 远期子模块**：`sdk/`（JDK/Python/Node.js 多版本切换）、`bundle/`（软件束）、`shadow/`（沙盒环境）、`lifecycle/`（归档/孤立/去重）、`monitor/`（hit top/ps/trace）、`sync/`（跨设备配置同步）、`dev/`（本地安装 + 文件监听）、`backup/`（备份恢复）、`delta/`（增量更新）

---

## 📦 数据存储结构

### 用户目录布局（默认 `~/.hit/`）

```
C:\Users\<username>\.hit\
├── apps/                     # 软件安装目录
│   ├── git/
│   │   ├── 2.40.0/          # 具体版本
│   │   │   ├── bin/
│   │   │   ├── etc/
│   │   │   └── ...
│   │   └── current -> 2.40.0 # 符号链接指向当前版本
│   ├── python/
│   │   ├── 3.11.0/
│   │   ├── 3.12.0/
│   │   └── current -> 3.12.0
│   └── ...
│
├── shims/                    # Shim 代理目录（加入 PATH）
│   ├── git.exe
│   ├── python.exe
│   └── ...
│
├── persist/                  # 持久化数据（配置文件）
│   ├── git/
│   │   └── config
│   └── python/
│       └── site-packages/
│
├── cache/                    # 下载缓存
│   └── <hash>.zip
│
├── db.json                   # 已安装软件清单
├── config.json               # 用户配置（镜像源、代理等）
├── logs/                     # 日志文件
└── plugins/                  # 插件目录
```

### db.json 结构示例

```json
{
  "installed_packages": {
    "git": {
      "version": "2.40.0",
      "bucket": "main",
      "install_date": "2024-01-15T10:30:00Z",
      "persist_files": ["etc/gitconfig"],
      "shims": ["git.exe", "git-lfs.exe"],
      "link_mode": "symlink",
      "health_status": "healthy",
      "last_check": "2024-01-15T10:30:00Z",
      "dependencies": ["vc_redist"],
      "size_bytes": 524288000,
      "shadow_enabled": false
    },
    "python": {
      "version": "3.12.0",
      "bucket": "sdk",
      "install_date": "2024-01-16T14:20:00Z",
      "available_versions": ["3.11.0", "3.12.0"],
      "current_version": "3.12.0",
      "sdk_proxies": ["python.exe", "pip.exe", "idle.exe"],
      "link_mode": "symlink",
      "health_status": "healthy"
    }
  },
  "buckets": [
    {
      "name": "main",
      "url": "https://github.com/hit-buckets/main.git",
      "last_update": "2024-01-15T08:00:00Z"
    }
  ],
  "config": {
    "proxy": null,
    "mirror": "https://mirror.nju.edu.cn/hit-main.git",
    "aria2_enabled": true,
    "link_mode": "symlink",
    "auto_cleanup_days": 30,
    "health_check_interval_days": 7,
    "default_mirror": "tuna",
    "sync_enabled": false,
    "sync_provider": "github_gist",
    "dev_mode": false
  },
  "bundles": {
    "dev-environment": {
      "packages": [
        {"name": "python", "version": "3.12.0"},
        {"name": "git", "version": "2.40.0"}
      ],
      "installed": true
    }
  },
  "shadows": {
    "python-3.9": {
      "base": "python",
      "version": "3.9.0",
      "persist_path": "~/.hit/persist/shadow/python-3.9/",
      "created_at": "2024-01-17T10:00:00Z"
    }
  },
  "mirrors": {
    "python": [
      {"name": "tuna", "url": "https://mirrors.tuna.tsinghua.edu.cn/python/", "priority": 1},
      {"name": "aliyun", "url": "https://mirrors.aliyun.com/python/", "priority": 2}
    ]
  },
  "lifecycle": {
    "archived_versions": {
      "git": ["2.38.0", "2.39.0"]
    },
    "orphan_files": [],
    "dedup_stats": {
      "files_deduped": 150,
      "space_saved_bytes": 1073741824
    }
  },
  "monitor": {
    "process_tracking": {},
    "resource_stats": {
      "git.exe": {"avg_cpu": 2.5, "avg_memory_mb": 50.0}
    }
  },
  "sync": {
    "last_sync": "2024-01-17T08:00:00Z",
    "config_hash": "sha256:..."
  },
  "plugins": {
    "enabled": true,
    "installed": [
      {
        "name": "hello",
        "version": "1.0.0",
        "path": "plugins/hello.lua",
        "enabled": true
      }
    ]
  },
  "version": "2"
}
```

---

## 📚 参考目录

项目根目录包含以下参考源码目录，用于开发时对照：

| 目录 | 来源 | 用途 |
|------|------|------|
| [`ref/Scoop/`](./ref/Scoop/) | [Scoop PowerShell](https://github.com/ScoopInstaller/Scoop) | 原版 Scoop 实现，核心参考 |
| [`ref/Main/`](./ref/Main/) | [Scoop Main Bucket](https://github.com/ScoopInstaller/Main) | 官方软件清单，兼容性测试 |
| [`ref/Hok/`](./ref/Hok/) | [hok](https://github.com/chawyehsu/hok) | Rust 实现的 Scoop 替代品（较久未更新） |

详情见 [REFERENCE_PROJECTS.md](./REFERENCE_PROJECTS.md)。

---

## 🔗 相关文档

| 文档 | 内容 |
|------|------|
| [TECH_STACK.md](./TECH_STACK.md) | 技术栈清单与各模块 Cargo.toml 依赖 |
| [DEV_FLOW.md](./DEV_FLOW.md) | 初始化、构建、测试、发布流程 |
| [MANIFEST_FORMAT.md](./MANIFEST_FORMAT.md) | Manifest 清单格式（Scoop 兼容 + Hit 扩展） |
| [WINDOWS_NOTES.md](./WINDOWS_NOTES.md) | Windows 符号链接、PATH、UAC 等注意事项 |
| [ROADMAP.md](./ROADMAP.md) | 里程碑规划与新增功能详解 |
| [REFERENCE_PROJECTS.md](./REFERENCE_PROJECTS.md) | 参考项目与学习资源 |
| [CODING_GUIDELINES.md](./CODING_GUIDELINES.md) | 编码规范 |
| [TODO.md](./TODO.md) | 实现任务清单（Phase 1-3 权威） |
| [FEATURES.md](./FEATURES.md) | 功能特性清单与设计理念 |
