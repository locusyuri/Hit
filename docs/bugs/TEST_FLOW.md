# Hit 实测流程

> 本文档描述如何在本地安装 hit 后，对所有命令进行**实际手动测试**的完整流程。
> 测试环境：Windows + PowerShell 5+。所有命令在普通用户权限下运行（无需管理员）。

---

## 0. 测试准备

### 0.1 构建发布版二进制

```powershell
# 在项目根目录 C:\Repos\Hit 下
cargo build --release -p hit-cli -p hit-shim
```

验证产物存在：

```powershell
Test-Path .\target\release\hit.exe        # True
Test-Path .\target\release\hit-shim.exe   # True
```

### 0.2 安装到本地（使用安装脚本）

#### 0.2.1 交互式安装（首次体验流程）

```powershell
# 交互式安装，选择"使用本地编译的 exe"
.\scripts\install-hit.ps1
```

按提示输入：

- 安装路径：`C:\Users\<你>\Downloads\test\hit`（或任意独立测试目录，便于清理）
- 安装方式：输入 `2`
- hit.exe 路径：`C:\Repos\Hit\target\release\hit.exe`

安装完成后**重新打开终端**让 PATH 生效。

#### 0.2.2 一行命令快速安装（反复测试用）

跳过所有交互提示，直接用参数指定，适合在修复 Bug 后快速回归测试：

```powershell
.\scripts\install-hit.ps1 -Path C:\Users\Violet\Downloads\test\hit -FromLocal C:\Repos\Hit\target\release\hit.exe -NonInteractive -Force
```

参数说明：

| 参数 | 含义 |
|------|------|
| `-Path` | 安装目录（覆盖默认 `~/.hit`） |
| `-FromLocal` | 使用本地预编译 exe，跳过网络下载 |
| `-NonInteractive` | 跳过所有交互提示 |
| `-Force` | 覆盖已有安装 |

#### 0.2.3 清理环境变量（卸载辅助）

测试结束后清理 Hit 注册的 `HIT_ROOT` 和 PATH 中的 shims 条目（需两次确认）：

```powershell
.\scripts\uninstall-env.ps1
```

自动化场景可加 `-Force` 跳过确认。

### 0.3 确认安装成功

```powershell
hit --version
hit --help
```

### 0.4 记录测试根目录

后续所有命令默认在该安装目录下操作。用以下命令确认：

```powershell
hit prefix           # 应输出安装根目录
hit config list      # 查看当前配置
```

---

## 1. 首次启动引导

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 1.1 | 删除 `config.json` 后运行 `hit bucket list` | 显示 Hit ASCII 横幅 + 三选项菜单（1 快速开始 / 2 自定义 / 3 跳过） |
| 1.2 | 选择 `1` | 自动克隆 main、extras、versions 三个官方 Bucket，逐个显示 ✔ |
| 1.3 | 重新删除 config.json，选择 `3` | 提示"已跳过"，生成默认 `config.json` |
| 1.4 | 重新删除 config.json，选择 `2` | 列出已知 Bucket，提示逐个输入名称；输入 `main` 回车 → 添加成功；空行结束 |

---

## 2. Bucket 管理（`hit bucket` / `hit b`）

### 2.1 添加 Bucket

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 2.1.1 | `hit bucket add main` | 克隆 ScoopInstaller/Main，输出"✔ bucket 'main' 添加完成" |
| 2.1.2 | `hit bucket add extras` | 同上，添加 extras |
| 2.1.3 | `hit bucket add versions` | 同上，添加 versions |
| 2.1.4 | `hit bucket add main`（重复） | 报错"Bucket 'main' 已存在" |
| 2.1.5 | `hit bucket add myrepo https://github.com/user/repo.git` | 用自定义 URL 添加 |
| 2.1.6 | `hit bucket add unknownbucket`（未知名称且无 URL） | 报错"未知 bucket '...'，请提供 Git 仓库 URL" |

### 2.2 列出 Bucket

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 2.2.1 | `hit bucket list` | 表格输出：名称 / Manifest 数量 / 描述，底部显示总数 |
| 2.2.2 | `hit b ls` | alias 同样生效 |

### 2.3 更新 Bucket

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 2.3.1 | `hit bucket update` | 更新所有已添加 Bucket，逐个显示 ✔，最后汇总 |
| 2.3.2 | `hit bucket update main` | 仅更新指定的 main Bucket |
| 2.3.3 | `hit bucket update nonexistent` | 输出"没有可更新的 Bucket" |

### 2.4 移除 Bucket

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 2.4.1 | `hit bucket remove myrepo` | 移除 myrepo 目录并从 db.json 删除记录 |
| 2.4.2 | `hit bucket rm main`（alias） | 同上 |
| 2.4.3 | `hit bucket remove nonexistent` | 报错"Bucket 'nonexistent' 不存在" |

---

## 3. 搜索软件（`hit search` / `hit s`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 3.1 | `hit search git` | 表格输出含 git 的软件（名称 / 版本 / 描述），底部显示结果数 |
| 3.2 | `hit s python` | alias 生效，返回 python 相关结果 |
| 3.3 | `hit search GIT`（大写） | 大小写不敏感，仍能找到 |
| 3.4 | `hit search git --bucket main` | 仅返回 main bucket 中的结果 |
| 3.5 | `hit search nonexistent_xyz` | 输出"未找到匹配 'nonexistent_xyz' 的软件" |

---

## 4. 查看详情（`hit info`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 4.1 | `hit info git` | 输出：名称、版本、描述、主页、许可证、架构、依赖、Bucket |
| 4.2 | `hit info git --bucket main` | 指定 bucket 查询，避免歧义 |
| 4.3 | `hit info nonexistent` | 报错"未找到软件 'nonexistent'" |
| 4.4 | 在多个 bucket 都含同名软件时执行 `hit info git` | 报错提示"在多个 bucket 中找到 ... 请使用 --bucket 指定" |

---

## 5. 安装软件（`hit install` / `hit i`）

> 选用小体积、无依赖的软件测试，推荐 `hit`（本仓库自身已发布的）或 `curl`、`jq` 等。

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 5.1 | `hit install curl` | 显示下载进度 → 校验哈希 → 解压 → 安装 → 创建 shim，最后"✔ curl <version> 安装完成" |
| 5.2 | `hit i jq`（alias） | 同上，jq 安装成功 |
| 5.3 | `hit install curl curl`（重复安装） | 已安装时报错或提示（视实现）；加 `--force` 可强制重装 |
| 5.4 | `hit install curl --force` | 强制重装，覆盖现有版本 |
| 5.5 | `hit install main/git` | 指定 bucket 安装 |
| 5.6 | `hit install nonexistent_pkg` | 报错"未找到软件 'nonexistent_pkg'" |
| 5.7 | `hit install curl --arch 64bit` | 指定架构安装 |

安装后验证：

```powershell
hit list               # 应出现 curl / jq
hit which curl         # 输出 shim 路径和真实 exe 路径
curl --version         # 直接运行（PATH 已含 shims）
```

---

## 6. 列出已安装（`hit list` / `hit ls`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 6.1 | `hit list` | 表格输出：名称 / 版本 / 架构 / Bucket / 安装时间，底部总数 |
| 6.2 | `hit ls` | alias 生效 |
| 6.3 | `hit list curl`（过滤） | 仅显示含 "curl" 的行 |
| 6.4 | `hit list nonexistent` | "没有匹配 'nonexistent' 的已安装软件" |

---

## 7. 版本管理

### 7.1 切换版本（`hit reset` / `hit r`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 7.1.1 | `hit install python@3.11.0`（若 versions bucket 提供） | 安装指定旧版本 |
| 7.1.2 | `hit install python@3.12.0` | 安装另一版本，此时存在两个版本 |
| 7.1.3 | `hit reset python 3.11.0` | 切换 current junction 到 3.11.0，输出"✔ python 已切换到 3.11.0" |
| 7.1.4 | `hit reset python 9.9.9`（不存在） | 报错"版本 '9.9.9' 不存在（python）" |

### 7.2 锁定版本（`hit hold` / `hit unhold`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 7.2.1 | `hit hold curl` | 输出"🔒 'curl' 已锁定"，`hit list` 中 curl 行显示 `[held]` |
| 7.2.2 | `hit hold curl`（重复） | 输出"⏭ 'curl' 已经是锁定状态" |
| 7.2.3 | `hit update --all` | 跳过被锁定的 curl |
| 7.2.4 | `hit unhold curl` | 输出"🔓 'curl' 已解除锁定" |
| 7.2.5 | `hit unhold curl`（重复） | 输出"⏭ 'curl' 未处于锁定状态" |
| 7.2.6 | `hit hold nonexistent` | 报错"'nonexistent' 未安装" |

---

## 8. 更新软件（`hit update` / `hit u`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 8.1 | `hit update` | 先更新所有 Bucket → 检查新版本 → 升级，最后汇总"升级完成（x/y）" |
| 8.2 | `hit update --all` | 显式更新所有已安装软件 |
| 8.3 | `hit update curl` | 仅更新 curl |
| 8.4 | `hit update nonexistent` | 输出"未安装，跳过" |
| 8.5 | `hit update --force` | 忽略版本比较强制重装 |

---

## 9. 卸载软件（`hit uninstall` / `hit rm`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 9.1 | `hit uninstall jq` | 删除 apps 目录、shim、junction，输出"✔ jq 已卸载" |
| 9.2 | `hit rm curl --purge` | 卸载 curl 并删除 persist 持久化数据 |
| 9.3 | `hit uninstall nonexistent` | 报错"'nonexistent' 未安装" |
| 9.4 | `hit uninstall`（无参数） | 报错"至少指定一个要卸载的软件名" |
| 9.5 | `hit uninstall jq curl`（多个） | 依次卸载多个软件 |

---

## 10. 缓存管理（`hit cache`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 10.1 | `hit cache list` | 表格输出：软件 / 版本 / 大小 / 路径，底部汇总总数和总大小 |
| 10.2 | `hit cache dir` | 输出缓存目录绝对路径 |
| 10.3 | `hit cache clean` | 清理所有缓存，输出"✔ 已清理 N 个缓存文件" |
| 10.4 | `hit cache clean curl` | 仅清理 curl 相关缓存 |
| 10.5 | `hit cache list`（清理后） | "缓存为空" |

---

## 11. 清理旧版本（`hit cleanup` / `hit c`）

> 前提：安装过某软件的多个版本（见 7.1）。

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 11.1 | `hit cleanup python` | 删除非当前版本目录，输出删除列表和"✔ 已清理 N 个旧版本" |
| 11.2 | `hit cleanup --all` | 清理所有软件的旧版本 |
| 11.3 | `hit cleanup --cache` | 同时清理下载缓存 |
| 11.4 | `hit cleanup`（无旧版本） | "没有需要清理的内容" |

---

## 12. 路径查询

### 12.1 which

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 12.1.1 | `hit which curl`（已安装时） | 输出 Shim 路径和 Target 真实 exe 路径 |
| 12.1.2 | `hit which nonexistent` | 报错"未找到 'nonexistent' 的 shim 文件" |

### 12.2 prefix

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 12.2.1 | `hit prefix` | 输出安装根目录 |
| 12.2.2 | `hit prefix curl`（已安装） | 输出 `…\apps\curl` 路径 |
| 12.2.3 | `hit prefix nonexistent` | 报错"'nonexistent' 未安装" |

### 12.3 home

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 12.3.1 | `hit home git` | 打开浏览器访问 git 的 homepage URL |
| 12.3.2 | `hit home nonexistent` | 报错"未找到软件 'nonexistent'" |

---

## 13. 配置管理（`hit config`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 13.1 | `hit config list` | 列出所有配置项及当前值（proxy / mirror / aria2_enabled / no_junction / root_path / auto_cleanup_days / health_check_interval_days） |
| 13.2 | `hit config set proxy http://127.0.0.1:7890` | 设置代理，输出"✔ 配置 'proxy' 已更新" |
| 13.3 | `hit config set aria2_enabled true` | 设置布尔项 |
| 13.4 | `hit config set aria2_enabled yes` | 等价 true |
| 13.5 | `hit config set aria2_enabled maybe` | 报错"'maybe' 不是有效的布尔值" |
| 13.6 | `hit config set auto_cleanup_days 60` | 设置数字项 |
| 13.7 | `hit config set auto_cleanup_days abc` | 报错"'abc' 不是有效的数字" |
| 13.8 | `hit config set unknown_key value` | 报错"未知配置项" |
| 13.9 | `hit config set proxy ""`（空值） | 清空 proxy（设为 null） |
| 13.10 | `hit config list` | 验证上述修改已写入 |

---

## 14. 健康检查（`hit doctor`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 14.1 | `hit doctor`（系统正常） | "✔ 系统健康，无问题" |
| 14.2 | 手动删除某 app 的 `current` junction 后运行 `hit doctor` | 列出问题（MissingCurrent / BrokenJunction 等），提示用 `--fix` |
| 14.3 | `hit doctor --fix` | 自动重建 junction、删除损坏 shim，输出"✔ 已修复 x/y 个问题" |
| 14.4 | 手动创建一个指向不存在路径的 `.shim` 文件后运行 `hit doctor --fix` | 检测到 BrokenShim 并自动删除 |

---

## 15. 交互式搜索安装（`hit si`）

> 需要在真实终端中运行（非管道/重定向环境）。

| 步骤 | 操作 | 预期结果 |
|------|------|----------|
| 15.1 | `hit si` | 启动 TUI，显示搜索框和软件表格 |
| 15.2 | 输入 `git` | 实时过滤显示含 git 的软件 |
| 15.3 | `↑`/`↓` 移动选择 | 高亮行随之变化 |
| 15.4 | 按 `Enter` | 安装选中软件，退出 TUI 后显示安装结果 |
| 15.5 | `hit si python` 然后 `Esc` | 以 python 为初始关键词，Esc 取消返回无输出 |
| 15.6 | 同名软件在多 bucket 中存在时选中 | 弹出子窗口选择来源 bucket |

---

## 16. 系统状态（`hit status` / `hit st`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 16.1 | `hit status` | 显示 Hit 版本、已安装软件数、Bucket 数、可用软件总数、缓存文件数及大小、根目录 |
| 16.2 | `hit st` | alias 生效 |

---

## 17. 命令简写（alias）汇总验证

逐一验证所有 alias 解析正确：

| 完整命令 | 简写 | 验证方式 |
|----------|------|----------|
| `hit install x` | `hit i x` | 触发安装流程 |
| `hit search x` | `hit s x` | 触发搜索 |
| `hit update` | `hit u` | 触发更新 |
| `hit uninstall x` | `hit rm x` | 触发卸载 |
| `hit list` | `hit ls` | 列出已安装 |
| `hit status` | `hit st` | 显示状态 |
| `hit bucket …` | `hit b …` | bucket 管理 |
| `hit cleanup` | `hit c` | 清理 |
| `hit reset x v` | `hit r x v` | 版本切换 |

---

## 18. 日志级别（`-v`）

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 18.1 | `hit -v list` | 输出 INFO 级别 tracing 日志 |
| 18.2 | `hit -vv list` | 输出 DEBUG 级别 |
| 18.3 | `hit -vvv list` | 输出 TRACE 级别 |

---

## 19. 错误处理与边界情况

| 步骤 | 命令 | 预期结果 |
|------|------|----------|
| 19.1 | `hit`（无子命令） | 显示帮助信息（`arg_required_else_help`） |
| 19.2 | `hit wrongcmd` | clap 报错"unrecognized subcommand" |
| 19.3 | `hit install`（无参数） | 报错提示需要软件名 |
| 19.4 | 在无网络环境下 `hit bucket add main` | 报错提示克隆失败 |
| 19.5 | `hit bucket remove` 后 `hit list`（依赖该 bucket 的软件仍存在） | 验证 db.json 一致性 |

---

## 20. 完全卸载验证

| 步骤 | 操作 | 预期结果 |
|------|------|----------|
| 20.1 | `hit uninstall <所有已安装软件> --purge` | 全部卸载干净 |
| 20.2 | `hit bucket remove <所有 bucket>` | 全部移除 |
| 20.3 | 删除安装目录（如 `C:\Users\<你>\Downloads\test\hit`） | 完全清除 |
| 20.4 | 从 `HKCU\Environment\Path` 中移除 shims 路径 | PATH 不再包含 hit 相关路径 |
| 20.5 | 重开终端运行 `hit --version` | "hit 不是内部或外部命令" |

---

## 测试清单速查

| 模块 | 测试项数 | 对应章节 |
|------|---------|----------|
| 首次引导 | 4 | §1 |
| Bucket 管理 | 11 | §2 |
| 搜索 | 5 | §3 |
| 详情 | 4 | §4 |
| 安装 | 7 | §5 |
| 列表 | 4 | §6 |
| 版本管理 | 10 | §7 |
| 更新 | 5 | §8 |
| 卸载 | 5 | §9 |
| 缓存 | 5 | §10 |
| 清理 | 4 | §11 |
| 路径查询 | 5 | §12 |
| 配置 | 10 | §13 |
| 健康检查 | 4 | §14 |
| 交互搜索 | 6 | §15 |
| 系统状态 | 2 | §16 |
| 命令简写 | 9 | §17 |
| 日志级别 | 3 | §18 |
| 错误处理 | 5 | §19 |
| 完全卸载 | 5 | §20 |
| **合计** | **108** | |

> 测试通过标准：每个命令的输出与"预期结果"一致，无 panic、无异常退出码。
