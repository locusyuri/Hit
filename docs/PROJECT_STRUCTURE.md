# Hit - Windows 软件包管理器项目结构

## 📁 项目概览

Hit 是一个用 Rust 编写的 Windows 软件包管理器，核心设计理念：
- **零污染**：所有软件安装在用户目录，无需管理员权限
- **Shim 代理**：通过轻量级代理实现 PATH 不污染
- **版本管理**：支持 JDK、Python、Node.js 等 SDK 的多版本切换
- **深度卸载**：集成类似 Geek Uninstaller 的残留扫描清理功能
- **便携化**：解压即用，卸载干净
- **安全性**：软件完整性校验，安全扫描集成
- **可扩展性**：插件系统支持社区扩展
- **跨平台**：未来计划支持 Linux 和 macOS
- **企业级**：支持批量部署、软件白名单/黑名单等企业功能

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
│   │       │   ├── bucket.rs     # Bucket 管理
│   │       │   ├── check.rs      # 健康检查
│   │       │   ├── repair.rs     # 修复损坏
│   │       │   ├── bundle.rs     # 软件束管理
│   │       │   ├── shadow.rs     # 沙盒环境
│   │       │   ├── mirror.rs     # 镜像源管理
│   │       │   ├── doctor.rs     # 环境诊断
│   │       │   ├── dev.rs        # 开发模式
│   │       │   └── ...           # 其他子命令
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
│   │       ├── sdk/              # SDK 版本管理
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # SDK 管理器
│   │       │   └── proxy.rs      # SDK 代理转发
│   │       ├── transaction/      # 事务管理（新增）
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 事务管理器
│   │       │   └── rollback.rs   # 回滚机制
│   │       ├── dependencies/     # 依赖解析（新增）
│   │       │   ├── mod.rs
│   │       │   ├── resolver.rs   # 依赖解析器
│   │       │   └── conflict.rs   # 冲突检测
│   │       ├── health/           # 健康检查（新增）
│   │       │   ├── mod.rs
│   │       │   ├── checker.rs    # 完整性检查
│   │       │   └── repairer.rs   # 自动修复
│   │       ├── bundle/           # 软件束（新增）
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 束管理器
│   │       │   └── manifest.rs   # 束清单格式
│   │       ├── shadow/           # 沙盒环境（新增）
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 沙盒管理器
│   │       │   └── isolate.rs    # 隔离机制
│   │       ├── mirror/           # 镜像源管理（新增）
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 镜像管理器
│   │       │   └── speedtest.rs  # 速度测试
│   │       ├── lifecycle/        # 生命周期管理（新增）
│   │       │   ├── mod.rs
│   │       │   ├── archive.rs    # 归档管理
│   │       │   ├── orphan.rs     # 孤立文件清理
│   │       │   └── dedup.rs      # 跨软件去重
│   │       ├── monitor/          # 运行时监控（新增）
│   │       │   ├── mod.rs
│   │       │   ├── tracker.rs    # 进程跟踪
│   │       │   └── stats.rs      # 资源统计
│   │       ├── sync/             # 配置同步（新增）
│   │       │   ├── mod.rs
│   │       │   ├── exporter.rs   # 配置导出
│   │       │   └── importer.rs   # 配置导入
│   │       └── dev/              # 开发模式（新增）
│   │           ├── mod.rs
│   │           ├── local.rs      # 本地目录安装
│   │           └── watcher.rs    # 文件监听
│   │
│   │       ├── backup/           # 备份与恢复（新增）
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 备份管理器
│   │       │   └── storage.rs    # 存储后端
│   │
│   │       └── delta/            # 增量更新（新增）
│   │           ├── mod.rs
│   │           ├── diff.rs       # 差异计算
│   │           └── patch.rs      # 补丁应用
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
│   └── hit-plugin/               # 插件系统
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # 库入口
│           ├── manager.rs        # 插件管理器
│           ├── lua_engine.rs     # Lua 脚本引擎
│           ├── api.rs            # 插件 API 定义
│           └── hooks.rs          # 插件钩子系统
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

├── plugins/                      # 插件目录
    ├── README.md                 # 插件开发指南
    ├── examples/                 # 示例插件
    └── lua/                      # Lua 插件

├── docs/                         # 项目文档
    ├── ARCHITECTURE.md           # 架构设计文档
    ├── PLUGIN_GUIDE.md           # 插件开发指南
    ├── CONTRIBUTING.md           # 贡献指南
    └── ROADMAP.md                # 项目路线图
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
  - `hit bucket update` - 更新所有 bucket
  - `hit bucket conflict list/resolve` - 冲突检测与解决
  - `hit bucket stats/outdated` - 统计与过时检测
  - `hit bucket verify <bucket>` - 验证 bucket 完整性
  - `hit check` - 健康检查
  - `hit repair <package>` - 修复损坏的安装
  - `hit bundle create/install` - 软件束管理
  - `hit shadow create/exec/list` - 沙盒环境管理
  - `hit mirror add/list/refresh` - 镜像源管理
  - `hit doctor` - 环境诊断与问题修复
  - `hit dev install/watch` - 开发模式
  - `hit archive <package>` - 归档旧版本
  - `hit orphan list/clean` - 孤立文件管理
  - `hit dedup` - 跨软件去重
  - `hit top/ps/trace` - 运行时监控
  - `hit config export/import` - 配置同步

### 2. **hit-core** - 核心业务逻辑
- **职责**：实现软件安装、卸载、版本管理等核心功能
- **关键组件**：
  - **Package Installer**：解析 Manifest → 下载 → 校验哈希 → 解压 → 创建 Shim → 配置 Persist
  - **Shim Generator**：在 `~/.hit/shims/` 生成轻量级代理 exe
  - **Persist Linker**：使用符号链接将配置文件重定向到 `~/.hit/persist/`
  - **SDK Manager**：管理多版本 SDK，维护 `current` 符号链接
  - **Path Manager**：修改用户级 PATH（`HKCU\Environment`），广播 `WM_SETTINGCHANGE`
  
  **新增核心模块**：
  - **Transaction Manager**：事务性安装，原子操作，失败回滚
  - **Dependency Resolver**：依赖解析，自动安装依赖包，冲突检测
  - **Health Checker**：定期检查安装完整性，自动修复损坏文件
  - **Bundle Manager**：软件束管理，一键安装多个软件
  - **Shadow Manager**：沙盒隔离环境，多版本独立运行
  - **Mirror Manager**：镜像源管理，自动选择最快源
  - **Lifecycle Manager**：软件生命周期（归档、孤立文件清理、去重）
  - **Monitor**：运行时监控（进程跟踪、资源统计）
  - **Sync**：配置同步（export/import/云同步）
  - **Dev Mode**：开发模式（本地目录安装、文件监听）

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
- **职责**：管理软件清单仓库，解决多 bucket 冲突，优化搜索安装体验
- **核心设计**：三层索引架构（全局索引 + Bucket 缓存 + 源仓库）
  - **全局索引**：内存中维护 `软件名 → [版本列表]`，自动合并所有 bucket
  - **Bucket 优先级**：main(100) > sdk(50) > extras(30) > personal(10)
  - **自动选择**：安装时选最高版本 + 最高优先级 bucket

- **关键功能**：
  1. **全局搜索**：`hit search <keyword>` 显示所有 bucket 的结果，标注来源
  2. **交互式安装**：搜索后提供 FuzzySelect 界面，上下箭头选择，Enter 直接安装
  3. **版本约束**：支持 `@latest`、`@stable`、`@^3.12`、`@3.12.0` 等语法
  4. **冲突检测**：`hit bucket conflict list/resolve` 管理同名软件冲突
  5. **元数据增强**：bucket.json 包含 `priority`、`maintainer`、`package_count` 等
  6. **过时检测**：`hit bucket outdated` 显示可更新的软件
  7. **软件别名**：Manifest 中定义 `alias` 字段，支持 `py` → `python`
  8. **快速安装**：`hit install`（无参数）直接弹出交互选择框
  9. **安装前预览**：显示版本、大小、依赖、bucket 来源，确认后再安装
  10. **Bucket 统计**：`hit bucket stats` 显示各 bucket 软件数、过时数、更新频率

- **子命令扩展**：
  - `hit bucket add/remove/list` - 基础管理
  - `hit bucket update` - 更新所有 bucket（并行）
  - `hit bucket conflict list/resolve` - 冲突管理
  - `hit bucket stats/outdated` - 统计与过时检测
  - `hit bucket verify <bucket>` - 验证 bucket 完整性

- **Manifest 扩展字段**：
```json
{
  "name": "python",
  "version": "3.12.0",
  "alias": ["py", "python3"],
  "bucket_priority": 50,
  "bucket_maintainer": "Python Official",
  "bucket_last_update": "2024-01-20T00:00:00Z"
}
```

- **Bucket 元数据（bucket.json）**：
```json
{
  "name": "main",
  "url": "https://github.com/hit-buckets/main.git",
  "priority": 100,
  "maintainer": "Hit Team",
  "package_count": 156,
  "last_update": "2024-01-20T00:00:00Z",
  "auto_update": true
}
```

### 6. **hit-common** - 公共工具库
- **职责**：提供跨模块复用的工具函数
- **组件**：
  - **Downloader**：支持断点续传、多线程加速（可选 aria2 集成）
  - **Hasher**：计算 SHA256/BLAKE3 校验和
  - **Extractor**：解压 ZIP/7z/TAR.GZ
  - **Logger**：结构化日志输出
  - **Error**：统一错误类型（`thiserror`）

### 7. **hit-plugin** - 插件系统
- **职责**：支持社区扩展功能
- **核心设计**：
  - **Lua 脚本引擎**：使用 `mlua` 集成 Lua 脚本
  - **插件钩子**：安装前/后、卸载前/后、命令执行前/后
  - **API 接口**：提供插件访问核心功能的接口
- **插件类型**：
  - **命令插件**：添加自定义子命令
  - **安装插件**：自定义安装逻辑
  - **卸载插件**：自定义卸载逻辑
  - **监控插件**：扩展运行时监控
- **插件目录**：`~/.hit/plugins/`
- **示例**：
  ```lua
  -- 插件示例：添加自定义命令
  function hit.command("hello")
      print("Hello from plugin!")
  end
  ```

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
  "version": "2"  // db 格式版本
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
    "crates/hit-plugin",
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
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-rustls"] }
rusqlite = "0.30"
tokio = { version = "1", features = ["full"] }

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

# hit-plugin/Cargo.toml
[dependencies]
hit-common = { path = "../hit-common" }
mlua = { version = "0.9", features = ["lua54"] }
serde = { version = "1", features = ["derive"] }

# hit-core/Cargo.toml 新增依赖
# 用于备份与恢复
zip = { version = "0.6", features = ["deflate"] }
# 用于增量更新
bsdiff = "0.1"
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
  },
  
  "bucket_priority": 100,
  "bucket_maintainer": "Git for Windows Team",
  "bucket_last_update": "2024-01-15T00:00:00Z",
  
  "alias": ["git", "git-cli"],
}
```

### 扩展字段（新增）

```json
{
  "name": "python",
  "version": "3.12.0",
  "description": "Python programming language",
  
  // 软件别名（便捷安装）
  "alias": ["py", "python3", "python3.12"],
  
  // Bucket 元数据（新增）
  "bucket_priority": 50,
  "bucket_maintainer": "Python Official",
  "bucket_last_update": "2024-01-20T00:00:00Z",
  
  // 依赖管理（自动安装依赖包）
  "dependencies": {
    "vc_redist": {"version": ">=14.0", "type": "runtime"},
    "openssl": {"version": "3.0", "type": "optional"}
  },
  
  // 健康检查配置（定期校验文件完整性）
  "health_check": {
    "enabled": true,
    "interval_days": 7,
    "critical_files": ["python.exe", "DLLs/"],
    "verify_hash": true
  },
  
  // 软件束定义（多个软件组合）
  "bundle": {
    "name": "dev-environment",
    "description": "Python development environment",
    "packages": [
      {"name": "python", "version": "3.12.0"},
      {"name": "git", "version": "2.40.0"},
      {"name": "vscode", "version": "1.85.0"}
    ]
  },
  
  // 沙盒配置（隔离环境）
  "shadow": {
    "enabled": false,
    "persist_isolated": true,
    "env_isolated": ["PATH", "PYTHONPATH"]
  },
  
  // 镜像源配置（多镜像支持）
  "mirrors": [
    {"name": "official", "url": "https://www.python.org/ftp/python/"},
    {"name": "tuna", "url": "https://mirrors.tuna.tsinghua.edu.cn/python/"},
    {"name": "aliyun", "url": "https://mirrors.aliyun.com/python/"}
  ],
  
  // 生命周期策略
  "lifecycle": {
    "auto_archive": true,
    "keep_versions": 2,
    "dedup_enabled": true
  },
  
  // 监控配置
  "monitor": {
    "track_processes": true,
    "track_file_access": false,
    "stats_retention_days": 30
  },
  
  // 开发模式配置
  "dev": {
    "watch_paths": ["src/", "tests/"],
    "auto_reload": true
  }
}
```

---

## ⚠️ 注意事项

### 1. Windows 符号链接权限
- **必须使用符号链接**：版本切换功能依赖符号链接，硬链接和复制不支持
- **安装时自动申请权限**：首次安装时检测权限，若无开发者模式则提示开启
- **权限检测与引导**：
```rust
use std::os::windows::fs::symlink_dir;

fn ensure_symlink_permission() -> Result<()> {
    let test_dir = tempdir().unwrap();
    let test_link = test_dir.path().join("test_link");
    
    match symlink_dir(test_dir.path(), &test_link) {
        Ok(_) => {
            let _ = std::fs::remove_dir(test_link);
            Ok(())
        },
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            anyhow!("检测到符号链接权限不足。\n\n"
                "请选择以下方案：\n"
                "1. 开启 Windows 开发者模式（推荐）\n"
                "2. 以管理员身份运行 hit（仅首次安装）\n"
                "3. 使用 'hit config set link_mode hardlink' 降级（无法版本切换）")
        },
        Err(e) => anyhow!(e),
    }
}
```
- **提权安装**：若检测到无权限，可自动请求 UAC 提权（仅首次）
```rust
// 检测是否以管理员身份运行
fn is_admin() -> bool {
    unsafe { windows::Win32::Security::IsUserAnAdmin() }.unwrap_or(false)
}

// 若未提权，重新以管理员启动
if !is_admin() {
    let mut cmd = Command::new("powershell");
    cmd.args([
        "-Command",
        &format!("Start-Process hit.exe -ArgumentList '{}' -Verb RunAs", args.join(" ")),
    ]);
    cmd.spawn()?;
    std::process::exit(0);
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
- 深度卸载功能需管理员权限，自动检测并请求 UAC
- 普通卸载（Hit 安装的软件）无需提权

### 4. 安全性注意事项
- **软件签名验证**：建议启用签名验证，确保软件来源可信
- **权限管理**：最小权限原则，仅在必要时请求管理员权限
- **隐私保护**：不收集用户使用数据，配置文件本地存储
- **安全扫描**：集成 VirusTotal 等安全扫描服务（可选）
- **插件安全**：插件执行前进行安全检查，限制敏感操作权限

---

## 🎯 里程碑规划

### Phase 1：基础框架（MVP）- 预计 2 个月
- [x] 项目工作区结构设计
- [x] 核心模块划分
- [ ] 实现 `hit install/uninstall/list` 基本命令
- [ ] Shim 代理机制（必须使用符号链接）
- [ ] Manifest 解析与验证
- [ ] 简单的 Bucket 支持
- [ ] 事务性安装（原子操作，失败回滚）
- [ ] 权限检测与自动提权机制
- [ ] 基础测试框架搭建

### Phase 2：核心功能完善 - 预计 3 个月
- [ ] Persist 持久化机制
- [ ] 版本切换（`hit reset`）
- [ ] 自动更新（`hit update`）
- [ ] 清理旧版本（`hit cleanup`）
- [ ] 依赖自动解析与安装
- [ ] 健康检查（`hit check`）
- [ ] 修复损坏（`hit repair`）
- [ ] 镜像源管理与速度测试
- [ ] Bucket 全局索引与自动选择（优先级系统）
- [ ] 交互式搜索安装（FuzzySelect）
- [ ] 软件别名（alias）支持
- [ ] Bucket 冲突检测与解决
- [ ] Bucket 统计与过时检测
- [ ] 安全扫描集成（VirusTotal）

### Phase 3：高级特性 - 预计 4 个月
- [ ] SDK 版本管理（JDK/Python/Node.js）
- [ ] 深度卸载模块
- [ ] Bucket 自动化更新流水线
- [ ] 多线程下载加速
- [ ] 软件束（Bundle）管理
- [ ] 沙盒环境（Shadow）
- [ ] 软件生命周期管理（归档、孤立文件清理、去重）
- [ ] 运行时监控（top/ps/trace）
- [ ] 备份与恢复功能
- [ ] 增量更新机制

### Phase 4：生态建设 - 预计 3 个月
- [ ] 官方 Bucket 仓库（GitHub 组织）
- [ ] Shell 补全脚本
- [ ] 配置同步（export/import）
- [ ] 开发模式（本地目录安装、文件监听）
- [ ] 环境诊断（`hit doctor`）
- [ ] 插件系统（Lua 脚本扩展）
- [ ] 社区贡献指南
- [ ] 详细文档与教程

### Phase 5：跨平台支持 - 预计 6 个月
- [ ] Linux 支持
- [ ] macOS 支持
- [ ] 跨平台测试
- [ ] 统一安装体验

### Phase 6：企业级功能 - 预计 4 个月
- [ ] 软件白名单/黑名单
- [ ] 批量部署（组策略）
- [ ] 审计日志
- [ ] 离线内网仓库
- [ ] 跨设备迁移
- [ ] 企业级安全扫描

### Phase 7：图形界面 - 预计 5 个月
- [ ] 基于 `tauri` 的 GUI 客户端
- [ ] 软件浏览、安装、卸载可视化
- [ ] 软件评分与评论系统
- [ ] 使用统计与推荐
- [ ] 系统托盘集成

---

---

## 🆕 新增功能详解

### 1. 事务性安装（Transaction）
- **模块**：`hit-core/transaction/`
- **职责**：保证安装/卸载的原子性，失败自动回滚
- **工作流程**：
  1. 创建临时事务目录
  2. 下载文件到临时目录
  3. 校验哈希
  4. 解压到临时目录
  5. 执行预安装脚本（验证）
  6. 原子移动：`rename` 临时目录 → 正式目录
  7. 更新 `db.json`
  8. 生成 Shim
  9. 提交事务
- **回滚机制**：任一阶段失败，清理临时文件，恢复 `db.json` 快照
- **实现**：使用 `tempfile` 创建临时目录，`std::fs::rename` 原子移动

### 2. 依赖解析（Dependencies）
- **模块**：`hit-core/dependencies/`
- **职责**：自动解析并安装软件依赖
- **Manifest 字段**：
```json
{
  "dependencies": {
    "vc_redist": {
      "version": ">=14.0",
      "type": "runtime",
      "optional": false
    },
    "openssl": {
      "version": "3.0",
      "type": "optional"
    }
  }
}
```
- **依赖图**：检测循环依赖、版本冲突
- **安装策略**：先安装依赖，再安装主包；依赖已满足则跳过

### 3. Bucket 优化（Bucket Optimizations）
- **模块**：`hit-core/`（bucket 相关子模块）
- **职责**：解决多 bucket 冲突，提升搜索安装体验
- **核心设计**：
  - **全局索引**：内存缓存所有 bucket 的 manifest，合并为 `软件名 → [版本列表]`
  - **优先级系统**：bucket 按优先级排序（main=100 > sdk=50 > extras=30）
  - **自动选择**：安装时自动选最高版本 + 最高优先级 bucket
  - **交互式选择**：搜索后提供 FuzzySelect 界面，上下箭头选择，Enter 直接安装

- **关键功能**：
  1. **全局搜索**：显示所有 bucket 结果，标注来源
  2. **版本约束语法**：`@latest`、`@stable`、`@^3.12`、`@3.12.0`
  3. **冲突检测**：`hit bucket conflict list/resolve`
  4. **软件别名**：Manifest `alias` 字段，支持 `py` → `python`
  5. **安装前预览**：显示版本、大小、依赖、bucket 来源
  6. **Bucket 统计**：`hit bucket stats/outdated`
  7. **快速安装**：`hit install` 无参数时直接弹出选择框

- **Manifest 扩展**：
```json
{
  "alias": ["py", "python3"],
  "bucket_priority": 50,
  "bucket_maintainer": "Python Official"
}
```

- **Bucket 元数据（bucket.json）**：
```json
{
  "priority": 100,
  "maintainer": "Hit Team",
  "package_count": 156,
  "auto_update": true
}
```

### 3. 健康检查（Health Check）
- **模块**：`hit-core/health/`
- **职责**：定期检查安装完整性，自动修复损坏
- **子命令**：
  - `hit check` - 检查所有软件
  - `hit check <package>` - 检查指定软件
  - `hit repair <package>` - 重新下载损坏文件
- **检查项**：
  - 文件是否存在
  - 哈希是否匹配
  - Shim 是否指向正确版本
  - 关键文件是否可执行
- **自动模式**：后台定时检查（默认 7 天），发现损坏自动修复

### 4. 软件束（Bundle）
- **模块**：`hit-core/bundle/`
- **职责**：一键安装多个软件，适合团队环境
- **Bundle 清单格式**：
```json
{
  "name": "dev-environment",
  "description": "Python 开发环境",
  "version": "1.0.0",
  "packages": [
    {"name": "python", "version": "3.12.0", "required": true},
    {"name": "git", "version": "latest", "required": true},
    {"name": "vscode", "version": "stable", "required": false}
  ],
  "post_install": [
    "git config --global user.name 'Your Name'",
    "pip install -U pip setuptools"
  ]
}
```
- **命令**：
  - `hit bundle create <name>` - 从当前环境创建束
  - `hit bundle install <bundle>` - 安装束
  - `hit bundle list` - 列出已安装束
  - `hit bundle export <name>` - 导出为 JSON


### 5. 沙盒环境（Shadow）
- **模块**：`hit-core/shadow/`
- **职责**：创建隔离环境，多版本软件互不干扰
- **应用场景**：
  - Python 多项目依赖隔离（类似 virtualenv）
  - 测试不同版本软件
  - 安全沙盒运行未知软件
- **实现**：
  - 独立 persist 目录：`~/.hit/persist/shadow/<name>/`
  - 独立环境变量：仅沙盒内可见
  - 通过 `hit shadow exec <name> <cmd>` 进入沙盒
- **命令**：
  - `hit shadow create <name> --base <package> --version <ver>`
  - `hit shadow list`
  - `hit shadow exec <name> <command>`
  - `hit shadow delete <name>`


### 6. 镜像源管理（Mirror）
- **模块**：`hit-core/mirror/`
- **职责**：多镜像支持，自动选择最快源
- **功能**：
  - 内置镜像：清华、阿里、UCloud、官方
  - 速度测试：`hit mirror speedtest`
  - 自动切换：下载失败时自动切换镜像
  - 区域感知：根据地理位置选择最近镜像
- **配置**：
```json
{
  "mirrors": {
    "python": [
      {"name": "tuna", "url": "https://mirrors.tuna.tsinghua.edu.cn/python/", "priority": 1},
      {"name": "aliyun", "url": "https://mirrors.aliyun.com/python/", "priority": 2}
    ]
  },
  "default_mirror": "tuna"
}
```

### 7. 生命周期管理（Lifecycle）
- **模块**：`hit-core/lifecycle/`
- **职责**：软件全生命周期自动化管理
- **功能**：
  - **归档**：`hit archive <package>` - 将旧版本移至外部存储
  - **孤立文件清理**：`hit orphan list/clean` - 扫描无主文件
  - **去重**：`hit dedup` - 跨软件重复文件硬链接化，节省空间
  - **自动清理**：配置 `auto_cleanup_days`，自动删除 N 天未使用的版本

### 8. 运行时监控（Monitor）
- **模块**：`hit-core/monitor/`
- **职责**：跟踪软件运行状态，收集资源统计
- **功能**：
  - `hit top` - 实时显示软件资源占用（类似 top）
  - `hit ps <package>` - 查看软件相关进程树
  - `hit trace <package>` - 跟踪软件文件访问（需管理员）
  - 统计信息：平均 CPU、内存、I/O
  - 识别冗余软件（长期未使用）

### 9. 配置同步（Sync）
- **模块**：`hit-core/sync/`
- **职责**：跨设备同步配置和已安装列表
- **功能**：
  - `hit config export` - 导出配置到文件
  - `hit config import` - 从文件导入配置
  - 云同步：GitHub Gist、OneDrive、Dropbox
  - 选择性同步：排除大型软件
  - 冲突处理：时间戳优先或手动合并

### 10. 开发模式（Dev）
- **模块**：`hit-core/dev/`
- **职责**：支持从本地目录安装，适合开发者
- **功能**：
  - `hit dev install <local-path>` - 从本地目录安装（不下载）
  - `hit dev watch <package>` - 监听文件变化，自动重载
  - 自动检测文件修改，更新 Shim 指向
  - 适合调试自己开发的软件

### 11. 备份与恢复（Backup）
- **模块**：`hit-core/backup/`
- **职责**：提供配置和已安装软件的备份与恢复功能
- **功能**：
  - `hit backup create` - 创建完整备份
  - `hit backup restore <backup-file>` - 从备份恢复
  - `hit backup list` - 列出所有备份
  - **备份内容**：配置文件、已安装软件列表、Shim 配置
  - **备份位置**：本地文件系统或云存储
  - **增量备份**：仅备份变化的文件，节省空间

### 12. 增量更新（Delta Update）
- **模块**：`hit-core/delta/`
- **职责**：实现软件的增量更新，提高更新速度
- **功能**：
  - **差异计算**：比较新旧版本，生成差异包
  - **增量下载**：仅下载变化的部分
  - **补丁应用**：将差异包应用到当前版本
  - **回滚机制**：更新失败时回滚到原版本
- **支持格式**：ZIP、7z、TAR.GZ
- **优势**：减少下载量，提高更新速度，节省带宽

---

## 📚 参考项目

- **Scoop**：设计灵感来源，学习其 Shim 和 Persist 机制
- **rustup**：代理转发机制的优秀实践
- **mise**：多语言 SDK 统一管理
- **uv**：Rust 编写的高性能包管理器
- **Geek Uninstaller**：深度卸载的扫描策略
- **Chocolatey**：Windows 包管理的企业级实践
- **Homebrew**：跨平台包管理的成功案例
- **Nix**：声明式包管理和环境隔离
- **Bazel**：构建系统的增量更新机制
- **Neovim**：插件系统的设计参考
- **VirusTotal**：安全扫描集成的参考

---

## 🔗 相关文档

- [CODING_GUIDELINES.md](./CODING_GUIDELINES.md) - 编码规范
- [AGENT.md](./AGENT.md) - AI 助手使用指南
- [对话.md](./对话.md) - 需求讨论与技术选型记录
