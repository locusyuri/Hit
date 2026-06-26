# Hit

**一个用 Rust 编写的现代 Windows 软件包管理器，完全兼容 Scoop bucket/manifest 生态。**

## 特性

- **交互式搜索安装** — `hit si` 启动 TUI 界面，实时搜索 + 上下选择 + Enter 一键安装
- **命令简写** — `hit i` = `hit install`，`hit s` = `hit search`，所有常用命令都有 alias
- **零污染** — 安装到用户目录（`~/.hit`），无需管理员权限，不影响系统
- **Shim 代理** — 通过轻量级代理实现 PATH 整洁，卸载即删除，不留残留
- **版本管理** — `hit reset` 支持同一软件的多个版本间自由切换
- **版本锁定** — `hit hold` 锁住版本，`hit update` 时自动跳过
- **健康检查** — `hit doctor` 扫描安装完整性并自动修复（junction/shim/目录）
- **深度卸载** — `hit uninstall --purge` 同时删除 persist 持久化数据
- **Scoop 兼容** — 直接使用 Scoop bucket 的 manifest 格式，可添加任意 Scoop bucket
- **便携化** — 解压即用，删除 `~/.hit` 即完全卸载

## 快速开始

### 在线安装

```powershell
# 交互式安装（推荐）
.\scripts\install-hit.ps1

# 指定镜像和版本
.\scripts\install-hit.ps1 -Mirror github -Version 1.0.0
```

### 从源码构建

```bash
# 构建（需要 Rust 工具链）
cargo build --release -p hit-cli -p hit-shim

# 产物在 target/release/
#   hit.exe         ~11 MB  主程序
#   hit-shim.exe    ~214 KB  shim 代理
```

> 构建后可通过 `.\scripts\install-hit.ps1 -FromLocal .\target\release\hit.exe` 安装到本地。

## 常用命令

| 命令 | Alias | 说明 |
|------|-------|------|
| `hit install git` | `i` | 安装软件 |
| `hit search python` | `s` | 搜索软件 |
| `hit update` | `u` | 更新所有软件和 bucket |
| `hit uninstall git` | `rm` | 卸载软件 |
| `hit list` | `ls` | 列出已安装软件 |
| `hit info git` | — | 查看软件详情 |
| `hit status` | `st` | 查看系统状态 |
| `hit bucket list` | `b` | 管理 bucket 仓库 |
| `hit reset python 3.12.0` | `r` | 切换版本 |
| `hit hold python` | — | 锁定版本（禁止升级） |
| `hit unhold python` | — | 解除版本锁定 |
| `hit cleanup` | `c` | 清理旧版本和缓存 |
| `hit cache list` | — | 查看下载缓存 |
| `hit which git` | — | 查找 shim 对应的真实路径 |
| `hit prefix` | — | 显示安装根路径 |
| `hit home git` | — | 打开软件主页 |
| `hit config list` | — | 查看/修改配置 |
| `hit doctor` | — | 健康检查与自动修复 |
| `hit si` | — | TUI 交互式搜索并安装 |

## 下载镜像

| 镜像 | 地址 | 用法 |
|------|------|------|
| **GitHub** | `github.com/hit-buckets/hit` | `-Mirror github`（默认） |
| **CNB 云原生** | `cnb.cool/catmono/Hit` | `-Mirror cnb` |

## 项目结构

```
hit/
├── crates/
│   ├── hit-cli/         # 命令行入口（clap 命令树 + 进度渲染 + TUI）
│   ├── hit-core/        # 核心业务逻辑（manifest/install/store/bucket/health）
│   ├── hit-common/      # 公共工具库（config/session/event/error）
│   ├── hit-shim/        # Shim 代理（零外部依赖，~200KB）
│   └── hit-test-utils/  # 共享测试 fixture
├── docs/                # 项目文档与审查报告
├── scripts/
│   └── install-hit.ps1  # 安装脚本（交互式/静默）
├── ref/                 # Scoop / Hok 参考实现
└── .cnb.yml             # CNB 云原生构建配置
```

## 开发

```bash
# 运行测试
cargo test --workspace

# 代码检查
cargo clippy --workspace --all-targets

# 开发模式运行
cargo run -p hit-cli -- install git
```

## 许可证

MIT License
