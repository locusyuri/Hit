# Hit - Windows 软件包管理器项目结构

## 📁 项目概览

Hit 是一个用 Rust 编写的 Windows 软件包管理器，核心设计理念：
- **零污染**：所有软件安装在用户目录，无需管理员权限
- **Shim 代理**：通过轻量级代理实现 PATH 不污染
- **版本管理**：支持 JDK、Python、Node.js 等 SDK 的多版本切换
- **深度卸载**：集成类似 Geek Uninstaller 的残留扫描清理功能
- **便携化**：解压即用，卸载干净

---

## 🗂️ 完整目录结构

```
hit/
├── Cargo.toml                    # 主工作区配置
├── Cargo.lock                    # 依赖锁定文件
├── README.md                     # 项目说明文档
├── LICENSE                       # 开源许可证
├── .gitignore                    # Git 忽略配置
├── docs/                         # 项目文档
│   ├── PROJECT_STRUCTURE.md      # 项目结构说明（本文件）
│   ├── CODING_GUIDELINES.md      # 编码规范
│   ├── AGENT.md                  # AI 助手指南
│   └── 对话.md                   # 需求讨论记录
│
├── crates/                       # Rust 工作区子模块
│   │
│   ├── hit-cli/                  # 主命令行程序
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # 程序入口
│   │       ├── cli.rs            # CLI 参数解析（clap）
│   │       ├── commands/         # 子命令实现
│   │       │   ├── mod.rs
│   │       │   ├── install.rs    # 安装命令
│   │       │   ├── uninstall.rs  # 卸载命令
│   │       │   ├── update.rs     # 更新命令
│   │       │   ├── search.rs     # 搜索命令
│   │       │   ├── list.rs       # 列出已安装软件
│   │       │   ├── info.rs       # 查看软件信息
│   │       │   ├── reset.rs      # 版本切换
│   │       │   ├── cleanup.rs    # 清理旧版本
│   │       │   └── bucket.rs     # Bucket 管理
│   │       ├── config.rs         # 配置管理
│   │       └── utils.rs          # 通用工具函数
│   │
│   ├── hit-core/                 # 核心业务逻辑库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # 库入口
│   │       ├── package/          # 软件包管理
│   │       │   ├── mod.rs
│   │       │   ├── installer.rs  # 安装器接口与实现
│   │       │   ├── uninstaller.rs # 卸载器接口与实现
│   │       │   ├── manifest.rs   # Manifest 清单解析
│   │       │   └── version.rs    # 版本管理
│   │       ├── shim/             # Shim 代理机制
│   │       │   ├── mod.rs
│   │       │   ├── generator.rs  # Shim 生成器
│   │       │   └── resolver.rs   # Shim 路径解析
│   │       ├── persist/          # 持久化数据管理
│   │       │   ├── mod.rs
│   │       │   └── linker.rs     # 符号链接管理
│   │       ├── database/         # 状态数据库
│   │       │   ├── mod.rs
│   │       │   └── store.rs      # JSON/SQLite 存储
│   │       ├── environment/      # 环境变量管理
│   │       │   ├── mod.rs
│   │       │   └── path_manager.rs # PATH 操作
│   │       └── sdk/              # SDK 版本管理
│   │           ├── mod.rs
│   │           ├── manager.rs    # SDK 管理器
│   │           └── proxy.rs      # SDK 代理转发
│   │
│   ├── hit-shim/                 # Shim 代理可执行文件
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs           # Shim 启动器（转发命令到真实程序）
│   │
│   ├── hit-uninstaller/          # 深度卸载模块
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── scanner.rs        # 残留扫描器
│   │       ├── registry_cleaner.rs # 注册表清理
│   │       ├── file_scanner.rs   # 文件系统扫描
│   │       └── process_killer.rs # 进程强制终止
│   │
│   ├── hit-bucket/               # Bucket 仓库管理
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── repository.rs     # 仓库管理
│   │       ├── manifest_validator.rs # 清单验证
│   │       ├── checkver.rs       # 版本检查自动化
│   │       └── autoupdate.rs     # 自动更新流水线
│   │
│   └── hit-common/               # 公共工具库
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── downloader.rs     # HTTP 下载器（reqwest）
│           ├── hasher.rs         # 哈希计算（SHA256）
│           ├── extractor.rs      # 压缩包解压（ZIP/7z）
│           ├── logger.rs         # 日志输出
│           └── error.rs          # 统一错误类型
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

### 1. **hit-cli** - 命令行界面
- **职责**：用户交互入口，解析子命令并调用核心模块
- **关键技术**：`clap`（参数解析）、`indicatif`（进度条）、`colored`（彩色输出）
- **子命令**：
  - `hit install <package>` - 安装软件
  - `hit uninstall <package>` - 卸载软件
  - `hit update [package]` - 更新软件
  - `hit search <keyword>` - 搜索软件
  - `hit list` - 列出已安装软件
  - `hit info <package>` - 查看软件详情
  - `hit reset <package> <version>` - 切换版本
  - `hit cleanup` - 清理旧版本释放空间
  - `hit bucket add/remove/list` - Bucket 管理

### 2. **hit-core** - 核心业务逻辑
- **职责**：实现软件安装、卸载、版本管理等核心功能
- **关键组件**：
  - **Package Installer**：解析 Manifest → 下载 → 校验哈希 → 解压 → 创建 Shim → 配置 Persist
  - **Shim Generator**：在 `~/.hit/shims/` 生成轻量级代理 exe
  - **Persist Linker**：使用符号链接将配置文件重定向到 `~/.hit/persist/`
  - **SDK Manager**：管理多版本 SDK，维护 `current` 符号链接
  - **Path Manager**：修改用户级 PATH（`HKCU\Environment`），广播 `WM_SETTINGCHANGE`

### 3. **hit-shim** - Shim 代理程序
- **职责**：独立的轻量级 exe，负责命令转发
- **工作流程**：
  1. 接收命令行参数
  2. 读取 `~/.hit/db.json` 获取当前激活版本
  3. 拼接真实路径：`~/.hit/apps/<package>/<version>/bin/<exe>`
  4. 启动真实进程并转发 stdin/stdout/stderr
  5. 返回退出码
- **特点**：体积极小（~200KB），无外部依赖，启动速度快

### 4. **hit-uninstaller** - 深度卸载模块
- **职责**：清理传统安装软件的残留（需管理员权限）
- **功能**：
  - 读取注册表卸载信息（`winreg` crate）
  - 执行软件自带卸载程序
  - 并行扫描残留文件（`walkdir` + `rayon`）
  - 清理注册表键值
  - 强制终止进程（`windows-rs` API）
  - 删除服务/计划任务
- **使用场景**：`hit force-uninstall <package>`（针对非 Hit 安装的软件）

### 5. **hit-bucket** - Bucket 仓库管理
- **职责**：管理软件清单仓库，支持自动化更新
- **功能**：
  - 添加/移除 Bucket（Git 仓库）
  - Manifest 验证（URL 有效性、哈希匹配）
  - 版本检查自动化（`checkver` 机制）
  - 自动生成 PR（GitHub API + `octocrab`）

### 6. **hit-common** - 公共工具库
- **职责**：提供跨模块复用的工具函数
- **组件**：
  - **Downloader**：支持断点续传、多线程加速（可选 aria2 集成）
  - **Hasher**：计算 SHA256/BLAKE3 校验和
  - **Extractor**：解压 ZIP/7z/TAR.GZ
  - **Logger**：结构化日志输出
  - **Error**：统一错误类型（`thiserror`）

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
└── logs/                     # 日志文件
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
      "shims": ["git.exe", "git-lfs.exe"]
    },
    "python": {
      "version": "3.12.0",
      "bucket": "sdk",
      "install_date": "2024-01-16T14:20:00Z",
      "available_versions": ["3.11.0", "3.12.0"],
      "current_version": "3.12.0"
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
    "aria2_enabled": true
  }
}
```

---

## 🛠️ 技术栈清单

### 核心依赖（Cargo.toml）

```toml
[workspace]
members = [
    "crates/hit-cli",
    "crates/hit-core",
    "crates/hit-shim",
    "crates/hit-uninstaller",
    "crates/hit-bucket",
    "crates/hit-common",
]

# hit-common/Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "native-tls"] }
sha2 = "0.10"
blake3 = "1.5"
zip = "0.6"
sevenz-rust = "0.5"
tokio = { version = "1", features = ["full"] }

# hit-core/Cargo.toml
[dependencies]
hit-common = { path = "../hit-common" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
winreg = "0.50"
dirs = "5"
walkdir = "2"
rayon = "1"
thiserror = "1"

# hit-cli/Cargo.toml
[dependencies]
hit-core = { path = "../hit-core" }
hit-uninstaller = { path = "../hit-uninstaller" }
hit-bucket = { path = "../hit-bucket" }
clap = { version = "4", features = ["derive"] }
anyhow = "1"
indicatif = "0.17"
colored = "2"
dialoguer = "0.11"

# hit-uninstaller/Cargo.toml
[dependencies]
winreg = "0.50"
windows = { version = "0.52", features = ["Win32_System_Threading", "Win32_Foundation"] }
walkdir = "2"
rayon = "1"
regex = "1"

# hit-bucket/Cargo.toml
[dependencies]
hit-common = { path = "../hit-common" }
octocrab = "0.32"
git2 = "0.18"
serde_json = "1"
```

---

## 🚀 开发流程

### 1. 初始化项目
```bash
cargo new hit --vcs git
cd hit
# 创建工作区结构
mkdir -p crates/{hit-cli,hit-core,hit-shim,hit-uninstaller,hit-bucket,hit-common}/src
touch Cargo.toml
```

### 2. 构建与运行
```bash
# 开发模式
cargo run -- install git

# 发布模式（优化体积）
cargo build --release
strip target/release/hit.exe  # 去掉符号信息
```

### 3. 测试
```bash
cargo test --workspace
cargo test --package hit-core
```

### 4. 发布
```powershell
.\scripts\release.ps1 --version 0.1.0
```

---

## 📝 Manifest 清单格式

### 基础结构（JSON）

```json
{
  "name": "git",
  "version": "2.40.0",
  "description": "Distributed version control system",
  "homepage": "https://git-scm.com",
  "license": "GPL-2.0",
  
  "architecture": {
    "64bit": {
      "url": "https://github.com/git-for-windows/git/releases/download/v2.40.0.windows.1/PortableGit-2.40.0-64-bit.7z.exe",
      "hash": "sha256:abc123..."
    }
  },
  
  "bin": [
    "bin/git.exe",
    "bin/git-lfs.exe"
  ],
  
  "env_set": {
    "GIT_INSTALL_ROOT": "$dir"
  },
  
  "persist": [
    "etc/gitconfig",
    "share/git-core/templates"
  ],
  
  "checkver": {
    "github": "https://github.com/git-for-windows/git",
    "regex": "v([\\d.]+)\\.windows\\.1"
  },
  
  "autoupdate": {
    "architecture": {
      "64bit": {
        "url": "https://github.com/git-for-windows/git/releases/download/v$version.windows.1/PortableGit-$version-64-bit.7z.exe"
      }
    }
  }
}
```

---

## ⚠️ 注意事项

### 1. Windows 符号链接权限
- Windows 10+ 需开启**开发者模式**才能非管理员创建符号链接
- 降级方案：使用硬链接或复制文件（失去版本切换能力）
- 检测代码：
```rust
use std::os::windows::fs::symlink_dir;

fn create_link(target: &Path, link: &Path) -> Result<()> {
    symlink_dir(target, link).map_err(|e| {
        if e.kind() == ErrorKind::PermissionDenied {
            anyhow!("请开启开发者模式或以管理员运行")
        } else {
            anyhow!(e)
        }
    })
}
```

### 2. PATH 环境变量刷新
- 修改 `HKCU\Environment` 后需广播消息：
```rust
use windows::Win32::UI::WindowsAndMessaging::{SendMessageTimeoutW, HWND_BROADCAST, WM_SETTINGCHANGE};

unsafe {
    SendMessageTimeoutW(
        HWND_BROADCAST,
        WM_SETTINGCHANGE,
        0,
        LPARAM("Environment\0".as_ptr() as isize),
        SMTO_ABORTIFHUNG,
        5000,
        &mut result,
    );
}
```

### 3. 深度卸载需要提权
- 检测是否需要管理员权限
- 使用 `runas` 重新以管理员身份启动：
```rust
use std::process::Command;

Command::new("powershell")
    .arg("-Command")
    .arg(&format!("Start-Process hit.exe -ArgumentList 'force-uninstall {}' -Verb RunAs", package))
    .spawn()?;
```

---

## 🎯 里程碑规划

### Phase 1：基础框架（MVP）
- [ ] 实现 `hit install/uninstall/list` 基本命令
- [ ] Shim 代理机制
- [ ] Manifest 解析与验证
- [ ] 简单的 Bucket 支持

### Phase 2：核心功能完善
- [ ] Persist 持久化机制
- [ ] 版本切换（`hit reset`）
- [ ] 自动更新（`hit update`）
- [ ] 清理旧版本（`hit cleanup`）

### Phase 3：高级特性
- [ ] SDK 版本管理（JDK/Python/Node.js）
- [ ] 深度卸载模块
- [ ] Bucket 自动化更新流水线
- [ ] 多线程下载加速

### Phase 4：生态建设
- [ ] 官方 Bucket 仓库（GitHub 组织）
- [ ] Shell 补全脚本
- [ ] 图形界面（可选，使用 `egui` 或 `tauri`）
- [ ] 插件系统（Lua 脚本扩展）

---

## 📚 参考项目

- **Scoop**：设计灵感来源，学习其 Shim 和 Persist 机制
- **rustup**：代理转发机制的优秀实践
- **mise**：多语言 SDK 统一管理
- **uv**：Rust 编写的高性能包管理器
- **Geek Uninstaller**：深度卸载的扫描策略

---

## 🔗 相关文档

- [CODING_GUIDELINES.md](./CODING_GUIDELINES.md) - 编码规范
- [AGENT.md](./AGENT.md) - AI 助手使用指南
- [对话.md](./对话.md) - 需求讨论与技术选型记录
