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
│   ├── REFERENCE_PROJECTS.md     # 参考项目
│   └── 对话.md                   # 需求讨论与技术选型记录
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
│   │       ├── transaction/      # 事务管理
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 事务管理器
│   │       │   └── rollback.rs   # 回滚机制
│   │       ├── dependencies/     # 依赖解析
│   │       │   ├── mod.rs
│   │       │   ├── resolver.rs   # 依赖解析器
│   │       │   └── conflict.rs   # 冲突检测
│   │       ├── health/           # 健康检查
│   │       │   ├── mod.rs
│   │       │   ├── checker.rs    # 完整性检查
│   │       │   └── repairer.rs   # 自动修复
│   │       ├── bundle/           # 软件束
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 束管理器
│   │       │   └── manifest.rs   # 束清单格式
│   │       ├── shadow/           # 沙盒环境
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 沙盒管理器
│   │       │   └── isolate.rs    # 隔离机制
│   │       ├── mirror/           # 镜像源管理
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 镜像管理器
│   │       │   └── speedtest.rs  # 速度测试
│   │       ├── lifecycle/        # 生命周期管理
│   │       │   ├── mod.rs
│   │       │   ├── archive.rs    # 归档管理
│   │       │   ├── orphan.rs     # 孤立文件清理
│   │       │   └── dedup.rs      # 跨软件去重
│   │       ├── monitor/          # 运行时监控
│   │       │   ├── mod.rs
│   │       │   ├── tracker.rs    # 进程跟踪
│   │       │   └── stats.rs      # 资源统计
│   │       ├── sync/             # 配置同步
│   │       │   ├── mod.rs
│   │       │   ├── exporter.rs   # 配置导出
│   │       │   └── importer.rs   # 配置导入
│   │       ├── dev/              # 开发模式
│   │       │   ├── mod.rs
│   │       │   ├── local.rs      # 本地目录安装
│   │       │   └── watcher.rs    # 文件监听
│   │       ├── backup/           # 备份与恢复
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # 备份管理器
│   │       │   └── storage.rs    # 存储后端
│   │       └── delta/            # 增量更新
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
│   ├── hit-common/               # 公共工具库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── downloader.rs     # HTTP 下载器（reqwest）
│   │       ├── hasher.rs         # 哈希计算（SHA256）
│   │       ├── extractor.rs      # 压缩包解压（ZIP/7z）
│   │       ├── logger.rs         # 日志输出
│   │       └── error.rs          # 统一错误类型
│   │
│   └── hit-plugin/               # 插件系统
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # 库入口
│           ├── manager.rs        # 插件管理器
│           ├── lua_engine.rs     # Lua 脚本引擎
│           ├── api.rs            # 插件 API 定义
│           └── hooks.rs          # 插件钩子系统
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

### 1. **hit-cli** - 命令行界面
- **职责**：用户交互入口，解析子命令并调用核心模块
- **关键技术**：`clap`（参数解析）、`ratatui`（TUI 界面）、`indicatif`（进度条）、`colored`（彩色输出）
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
| [TODO.md](./TODO.md) | 实现任务清单 |
| [对话.md](./对话.md) | 需求讨论与技术选型记录 |
