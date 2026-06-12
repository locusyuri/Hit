# 技术栈清单

## 核心依赖总览

| 模块 | 依赖 | 用途 |
|------|------|------|
| CLI | clap | 参数解析 |
| CLI | indicatif | 进度条 |
| CLI | colored | 彩色输出 |
| CLI | dialoguer | 交互式选择 |
| Core | serde + serde_json | JSON 序列化 |
| Core | thiserror | 错误处理 |
| Core | petgraph | 依赖图 |
| Bucket | git2 | Git 仓库操作 |
| Downloader | reqwest | HTTP 下载 |
| Downloader | blake3/sha2 | 哈希计算 |
| Compression | zip/sevenz-rust/tar | 压缩解压 |
| Store | rusqlite | SQLite 数据库 |
| Windows | windows | Windows API |
| Windows | winreg | 注册表操作 |

## 工作区配置

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
```

## 各模块 Cargo.toml

### hit-common

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "native-tls"] }
sha2 = "0.10"
blake3 = "1.5"
zip = "0.6"
sevenz-rust = "0.5"
tokio = { version = "1", features = ["full"] }
```

### hit-core

```toml
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
```

### hit-cli

```toml
[dependencies]
hit-core = { path = "../hit-core" }
hit-uninstaller = { path = "../hit-uninstaller" }
hit-bucket = { path = "../hit-bucket" }
clap = { version = "4", features = ["derive"] }
anyhow = "1"
indicatif = "0.17"
colored = "2"
dialoguer = "0.11"
```

### hit-uninstaller

```toml
[dependencies]
winreg = "0.50"
windows = { version = "0.52", features = ["Win32_System_Threading", "Win32_Foundation"] }
walkdir = "2"
rayon = "1"
regex = "1"
```

### hit-bucket

```toml
[dependencies]
hit-common = { path = "../hit-common" }
octocrab = "0.32"
git2 = "0.18"
serde_json = "1"
```

### hit-plugin

```toml
[dependencies]
hit-common = { path = "../hit-common" }
mlua = { version = "0.9", features = ["lua54"] }
serde = { version = "1", features = ["derive"] }
```

### 附加依赖

```toml
# hit-core 新增 — 备份与恢复
zip = { version = "0.6", features = ["deflate"] }

# hit-core 新增 — 增量更新
bsdiff = "0.1"
```

---

> 详见 [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) — 项目结构与模块说明
