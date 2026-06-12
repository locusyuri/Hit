# 开发流程

## 1. 初始化项目

```bash
cargo new hit --vcs git
cd hit
# 创建工作区结构
mkdir -p crates/{hit-cli,hit-core,hit-shim,hit-uninstaller,hit-bucket,hit-common}/src
touch Cargo.toml
```

## 2. 构建与运行

```bash
# 开发模式
cargo run -- install git

# 发布模式（优化体积）
cargo build --release
strip target/release/hit.exe  # 去掉符号信息
```

## 3. 测试

```bash
cargo test --workspace
cargo test --package hit-core
```

## 4. 发布

```powershell
.\scripts\release.ps1 --version 0.1.0
```

---

> 详见 [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) — 项目结构与模块说明
