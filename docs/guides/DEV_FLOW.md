# 开发流程

## 1. 初始化项目

```bash
cargo new hit --vcs git
cd hit
# 创建工作区结构（5-crate 方案，详见 TODO.md）
mkdir -p crates/{hit-common,hit-core,hit-shim,hit-cli,hit-test-utils}/src
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

## 5. 一键安装脚本

`scripts/install-hit.ps1` 为最终用户提供 `irm get.hit.sh | iex` 风格的安装体验。

```powershell
# 默认安装到 $HOME\.hit（从 GitHub Releases 下载）
.\scripts\install-hit.ps1

# 自定义路径 + 镜像
.\scripts\install-hit.ps1 -Path D:\hit -Mirror tuna

# 本地预编译二进制安装（开发调试用）
.\scripts\install-hit.ps1 -FromLocal .\target\release\hit.exe

# 强制覆盖
.\scripts\install-hit.ps1 -Force
```

**脚本职责**：
1. 检查 PowerShell 5+ 与执行策略（自动 `Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`）
2. 回退链确定安装路径：`-Path` > `$env:HIT_ROOT` > `$env:SCOOP` > `$HOME\.hit`
3. 从 GitHub Releases / 镜像下载 zip 并解压
4. 创建 Scoop 兼容目录布局（apps/ shims/ cache/ persist/ buckets/ logs/）
5. 写入默认 `config.json`
6. 注册 `shims/` 到 `HKCU\Environment\Path` 并广播 `WM_SETTINGCHANGE`

---

> 详见 [PROJECT_STRUCTURE.md](../plan/PROJECT_STRUCTURE.md) — 项目结构与模块说明
