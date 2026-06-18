# 参考项目

## 项目内参考源码

项目包含以下参考目录，用于开发时对照学习：

| 目录 | 来源 | 说明 |
|------|------|------|
| [`ref/Scoop/`](../ref/Scoop/) | [Scoop](https://github.com/ScoopInstaller/Scoop) | 原版 Scoop PowerShell 实现，核心参考 |
| [`ref/Main/`](../ref/Main/) | [Scoop 官方 Main Bucket](https://github.com/ScoopInstaller/Main) | Scoop 主仓库的软件清单，用于兼容性测试 |
| [`ref/Hok/`](../ref/Hok/) | [hok](https://github.com/chawyehsu/hok) | 用 Rust 实现的 Scoop 替代品（较久未更新），参考其架构设计 |

> **Hok 说明**：Hok 由 chawyehsu 开发，是一个用 Rust 实现 Scoop 的 CLI 工具。
> - 版本：v0.1.0-beta.7（较久未更新）
> - 架构与 Hit 类似（libscoop + CLI 分层）
> - 提供了 `scoop_hash` 和 `libscoop` 两个子 crate
> - 性能测试显示比原版 Scoop 快 35-73 倍
> - 值得参考：其 Shim 实现、Bucket 管理、Manifest 解析方式

## 外部参考项目

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

> 详见 [PROJECT.md](../plan/PROJECT.md) — 项目描述与模块说明
