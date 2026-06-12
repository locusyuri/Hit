# Hit

**一个用 Rust 编写的现代 Windows 软件包管理器**

## 特性

- **交互式安装**：集成 fzf 交互式搜索安装（`hit si`），告别手动翻找
- **命令简写**：内置子命令简写，`hit i` = `hit install`，`hit s` = `hit search`
- **零污染**：所有软件安装在用户目录，无需管理员权限
- **Shim 代理**：通过轻量级代理实现 PATH 不污染
- **版本管理**：支持 JDK、Python、Node.js 等 SDK 的多版本切换
- **深度卸载**：集成类似 Geek Uninstaller 的残留扫描清理功能
- **便携化**：解压即用，卸载干净

> 完整功能列表请参阅 [docs/FEATURES.md](docs/FEATURES.md)。

## 快速开始

### 安装

```bash
# 从源码构建（需要 Rust 工具链）
cargo build --release

# 安装软件
hit install git

# 列出已安装软件
hit list
```

### 基本使用

```bash
# 搜索软件
hit search python

# 安装指定版本
hit install python@3.11.0

# 切换版本
hit reset python 3.12.0

# 卸载软件
hit uninstall git

# 更新所有软件
hit update

# 清理旧版本
hit cleanup
```

## 项目结构

```
hit/
├── crates/           # Rust 工作区子模块
│   ├── hit-cli/      # 主命令行程序
│   ├── hit-core/     # 核心业务逻辑库
│   ├── hit-shim/     # Shim 代理可执行文件
│   ├── hit-uninstaller/  # 深度卸载模块
│   ├── hit-bucket/   # Bucket 仓库管理
│   └── hit-common/   # 公共工具库
├── buckets/          # 软件清单仓库
├── docs/             # 项目文档
└── scripts/          # 辅助脚本
```

详细文档请参阅 [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)。

## 开发

```bash
# 克隆项目
git clone <repository-url>
cd hit

# 开发模式运行
cargo run -- install git

# 运行测试
cargo test --workspace
```

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献

欢迎提交 Issue 和 Pull Request！