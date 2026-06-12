# Hit 功能特性

Hit 项目分为两层特性：**已完成** 的 PowerShell 包装函数（当前基于 Scoop 运行）和 **计划中** 的 Rust 原生实现。

---

## ✨ 已完成特性（PowerShell 包装层）

以下功能已通过 `ref/Microsoft.PowerShell_profile.ps1` 实现，当前对 Scoop 的增强。

### 1. 命令简写系统

通过 `hit` 包装函数，大幅缩短 Scoop 命令输入：

| 简写 | 展开 | 说明 |
|------|------|------|
| `hit i xxx` | `scoop install xxx` | 安装软件包 |
| `hit s xxx` | `scoop search xxx` | 搜索软件包 |
| `hit st xxx` | `scoop status xxx` | 查看软件状态 |
| `hit u xxx` | `scoop update xxx` | 更新软件包 |
| `hit rm xxx` | `scoop uninstall xxx` | 卸载软件包 |
| `hit ls` | `scoop list` | 列出已安装软件 |
| `hit b` | `scoop bucket` | 管理软件源 |
| `hit c` | `scoop cleanup` | 清理缓存 |

### 2. 交互式搜索安装 — `si`

基于 **fzf** 的交互式搜索安装工具，替代传统的命令行搜索。

**用法**：`si <关键词>`

**工作流程**：
1. 调用 `scoop search` 搜索关键词
2. 按包名分组聚合，显示名称、版本、来源、可执行文件
3. 通过 fzf 呈现表格界面（自动列宽对齐、截断处理）
4. Enter 选择 → 自动安装；Esc → 取消
5. 同名软件多源时，二次 fzf 选择来源

**示例界面**：
```
> fzf 搜索窗口
  git       2.40.0  [main,extras]  bin/git.exe
  python    3.12.0  [sdk]         bin/python.exe
```

### 3. 智能目录跳转 — `z` / `zi`

基于 **zoxide + fzf + eza** 的目录导航系统。

| 命令 | 功能 | 说明 |
|------|------|------|
| `z` | 返回用户目录 | 无参数时跳转到 `~` |
| `z <path>` | 直接跳转 | 路径是目录则直接 cd |
| `z <keyword>` | 模糊匹配 | 由 zoxide 根据历史匹配 |
| `zi` / `cdi` | 交互式跳转 | fzf 列表 + eza 树形预览 |
| `cd` | 别名到 `z` | 保留原有的 cd 习惯 |

`zi` 的交互界面特性：
- 右侧 **eza 实时预览** 目标目录结构（2 层深度）
- `Ctrl+Space` 切换预览面板开关
- zoxide 的 Frecency（频率+最近）排序

### 4. 文件列表美化 — `ls` / `tree`

基于 **eza**（Rust 实现的 ls 替代品）的命令增强：

| 命令 | 功能 |
|------|------|
| `ls` | 带图标的彩色文件列表 |
| `tree` | 带图标的树形目录展示 |

### 5. 一键加载 MSVC 编译环境 — `vcvars`

快速配置 Visual Studio Build Tools 环境变量，无需启动 Developer Command Prompt：

```powershell
vcvars
# MSVC 14.50.35717 / SDK 10.0.26100.0 (x64) environment loaded
```

设置以下环境变量：
- `PATH` — 添加 MSVC 工具链路径
- `INCLUDE` — 包含 MSVC、UCRT、Windows SDK 头文件
- `LIB` — 包含 MSVC、UCRT、Windows SDK 库文件

### 6. Neovide 启动 — `vi`

```powershell
vi <file>     # 用 Neovide 打开文件
vi            # 直接启动 Neovide
```

### 7. 提示符美化 — oh-my-posh

使用 Catppuccin 主题的 oh-my-posh 提示符，提供 Git 状态、执行时间等上下文信息。

### 8. PSReadLine 智能历史

- **预测源**：HistoryAndPlugin（历史和插件双预测）
- **展示方式**：ListView（列表视图）

### 9. 启动横幅

每次打开 PowerShell 时显示功能速查横幅：

```
🐱  ~  Meow, welcome <user>  ~  🐱

╭─ PowerShell 7 ready ─────────────────────╮
│  hit i   → scoop install                 │
│  hit s   → scoop search                  │
│  si      → search & install (fzf)        │
│  z       → smart cd (zoxide)             │
│  zi/cdi  → interactive jump (fzf+eza)    │
│  ls/tree → eza with icons                │
│  vi      → neovide                       │
│  vcvars  → load MSVC/SDK env             │
╰──────────────────────────────────────────╯
```

---

## 🚧 计划特性（Rust 原生 — Hit CLI）

以下特性是 Hit 项目（Rust 实现）的目标功能。

### 1. 零污染安装

- 所有软件安装在用户目录 `~/.hit/apps/`，无需管理员权限
- 不写注册表、不修改系统文件
- **便携化**：解压即用，卸载即删

### 2. Shim 代理机制

- 在 `~/.hit/shims/` 生成轻量级代理 exe（~200KB）
- 加入 PATH 后自动转发命令到实际程序
- 版本切换时只需更新符号链接指向

### 3. SDK 多版本管理

- 支持 JDK、Python、Node.js 等 SDK 多版本共存
- `current` 符号链接切换当前激活版本
- 版本约束语法：`@latest`、`@stable`、`@^3.12`、`@3.12.0`

### 4. 深度卸载

- 扫描注册表卸载信息（`winreg`）
- 并行残留文件扫描（`walkdir` + `rayon`）
- 清理注册表键值、服务、计划任务
- 强制进程终止（`windows-rs` API）

### 5. 事务性安装

- 原子操作：任意阶段失败自动回滚
- 下载 → 校验哈希 → 解压 → 安装 → 生成 Shim 全链路保护
- 使用临时目录 + `std::fs::rename` 原子移动

### 6. 依赖解析

- Manifest 中声明依赖关系（版本约束、可选/必需）
- 依赖图检测循环依赖和版本冲突
- 先安装依赖，再安装主包

### 7. Bucket 优化

- **三层索引**：全局索引 → Bucket 缓存 → 源仓库
- **优先级系统**：main(100) > sdk(50) > extras(30)
- **交互式选择**：搜索后 FuzzySelect 界面 + Enter 直接安装
- **软件别名**：`py` → `python`
- **安装前预览**：版本、大小、依赖、来源

### 8. 健康检查

- 定期校验文件完整性和哈希匹配
- 自动修复损坏（`hit repair`）
- 检查 Shim 指向是否正确

### 9. 镜像源管理

- 内置中国镜像（清华、阿里、UCloud）
- 速度测试与自动切换
- 区域感知：选择最近镜像

### 10. 软件束（Bundle）

- 一键安装多个软件（如 `dev-environment` 束）
- 支持导出/导入束配置
- 适合团队标准化开发环境

### 11. 沙盒环境（Shadow）

- 隔离运行时环境（独立 persist、独立环境变量）
- 类似 Python virtualenv 但通用
- 安全运行未知软件

### 12. 生命周期管理

- **归档**：将旧版本移至外部存储
- **孤立文件清理**：扫描无主残留文件
- **去重**：跨软件重复文件硬链接化
- **自动清理**：N 天未使用的版本自动删除

### 13. 运行时监控

- `hit top` — 实时软件资源占用
- `hit ps` — 进程树查看
- `hit trace` — 文件访问跟踪

### 14. 插件系统

- Lua 脚本引擎（`mlua`）
- 插件钩子：安装前/后、卸载前/后
- 自定义子命令

### 15. 其他

- **配置同步**：跨设备同步配置和已安装列表
- **开发模式**：本地目录安装，文件变化自动重载
- **备份与恢复**：完整配置备份
- **增量更新**：仅下载差异部分，减少带宽
- **环境诊断**：`hit doctor` 一键排查问题

---

> 参见 [ROADMAP.md](./ROADMAP.md) — 特性路线图与实现时间线
> 参见 [TODO.md](./TODO.md) — 具体实现任务清单
