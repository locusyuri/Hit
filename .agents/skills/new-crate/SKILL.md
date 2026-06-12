---
name: new-crate
description: 创建 Hit 项目的新 workspace crate 子模块。用于用户需要添加新 crate 时自动搭建 Cargo.toml 和 src/lib.rs 骨架并注册到 workspace。
---

# new-crate — 创建新的 workspace crate

当用户要求添加一个新的 crate（例如"创建一个 hit-downloader crate"）时使用此 skill。

## 输入

- **crate 名称**：遵循 `hit-<name>` 命名规范（例如 `hit-cli`、`hit-downloader`）。如果用户未提供 `hit-` 前缀，自动补全。
- **描述**（可选）：crate 的用途说明。

## 执行步骤

1. **检查 crates/ 目录**：如果 `crates/` 目录不存在，先创建它。
2. **创建目录结构**：在 `crates/hit-<name>/` 下创建：
   - `Cargo.toml`：包含 `[package]` 表（name、version 继承 workspace、edition 2024）
   - `src/lib.rs`：空的 lib 入口文件
3. **注册到 workspace**：在根 `Cargo.toml` 的 `[workspace]` 成员的 `members` 数组中添加 `"crates/hit-<name>"`。如果 `[workspace]` 不存在，创建它。
4. **验证**：运行 `cargo check --workspace` 确认新 crate 可编译。

## Cargo.toml 模板

```toml
[package]
name = "hit-<name>"
version.workspace = true
edition = "2024"
description = "<用途说明>"

[dependencies]
```

## 注意事项

- 不要添加未经用户确认的依赖
- 不要生成 `main.rs`，新 crate 默认是 library
- 如果用户明确要求 binary crate，则创建 `src/main.rs` 并在 Cargo.toml 中添加 `[[bin]]` 配置
