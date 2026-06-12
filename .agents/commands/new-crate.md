---
name: new-crate
description: 创建 Hit 项目的新 crate 子模块（Cargo.toml + src/lib.rs）
usage: /new-crate <crate-name>
---

按照 Hit 项目约定创建一个新的 crate 子模块。

## 步骤

1. 在 `crates/<crate-name>/` 下创建目录结构
2. 生成 `Cargo.toml`，使用 `{ name = "hit-<name>", version = "0.1.0", edition = "2024" }`
3. 生成 `src/lib.rs`（库 crate）或 `src/main.rs`（二进制 crate）
4. 将 crate 添加到工作区根 `Cargo.toml` 的 `[workspace.members]`

## 命名规则

- 目录名：`crates/hit-<name>/`
- 包名：`hit-<name>`
- 例：`hit-cli`, `hit-core`, `hit-shim`, `hit-bucket`, `hit-common`, `hit-uninstaller`, `hit-plugin`

## 默认依赖（可选，按需添加）

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
```
