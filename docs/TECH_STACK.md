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
