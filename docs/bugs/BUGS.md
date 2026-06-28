# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 第五轮自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。

---

## 重装/升级/doctor--fix 时 Junction 创建失败 os error 183 ⭐⭐⭐⭐⭐

> **极为严重** —— 已安装软件重装（`--force`）、升级、`doctor --fix` 时，因旧 `current` junction 未删除，创建新 junction 时报 error 183。第五轮实测仍复现，开发 Agent 声称已修（`feb7c45`）但实测未生效。

### 现象

```
$ hit install curl --force（curl 已装）
▶ [提交] curl... → WARN 事务回滚 app=curl
错误: IO 错误：创建 Junction: ...\curl\current -> ...\curl\8.21.0_1：
Cannot create a file when that file already exists. (os error 183)

$ hit doctor --fix（jq current 链接损坏）
✗ jq 修复失败: Cannot create a file when that file already exists. (os error 183)
```

### 根因

创建 junction 前未检查并删除已有的旧 junction。`force` 重装和 `doctor --fix` 都走到创建 junction 路径，但旧 junction 未清理。

### 修复方向

创建 junction 前先 `Remove-Item` 或检查 `Exists` 再创建。三个路径统一修：`install --force`、`update`、`doctor --fix`。

### 证据

REPORT.md §5.4（curl --force 回滚）、§8.5（jq 升级 junction 失败）、§14.3（doctor --fix 修复失败）

---

## post_install 脚本缺少 Scoop 环境变量 ⭐⭐⭐⭐⭐

> **极为严重** —— post_install 改用 pwsh 执行（已修，不再是 cmd.exe），但 git 等含复杂 post_install 的软件引用 `$bucketsdir`、`$dir` 等 Scoop 变量，hit 未定义这些变量导致失败。

### 现象

```
$ hit install main/git
▶ [提交] git... ✔
C:\...\apps\git\2.54.0: 命令 "$bucketsdir\$bucket\scripts\git" 找不到...
  → $bucketsdir 展开为空字符串，路径变成 \\scripts\git
WARN 事务回滚 app=git
错误: 安装 'git' 失败：PostInstall 脚本退出码：1
```

### 根因

pwsh 执行方式已改对（不再是 `cmd.exe /C`），但 Scoop post_install 脚本依赖以下环境变量，hit 在执行前没设置：
- `$bucketsdir` — bucket 根目录
- `$dir` — 软件安装目录
- `$version` — 当前版本
- `$persist_dir` — 持久化目录
- `$cfg` — 配置对象

### 修复方向

在执行 post_install（pwsh -NoProfile -Command）前，先定义这些 Scoop 兼容变量。参考原版 Scoop `install.ps1` 中的 `$bucketsdir`、`$dir` 等变量定义。

### 证据

REPORT.md §5.5（git 安装失败详情）+ 用户此前关于 7zip 安装失败的反馈（`'$7zip_dir' is not recognized`）

---

## 已修复（迁移至 SOLVED_BUGS）✅

以下 bug 在本轮实测中已验证修复：

- ✅ **搜索含描述匹配** — `hit search git` = 82 结果（从 162 降，仅名称子串匹配），`-d` 标志可用
- ✅ **clap 错误被吞** — `hit`→帮助 / `hit wrongcmd`→`error: unrecognized subcommand` / `hit install`→"至少指定一个软件名"
- ✅ **install 已装检测** — `hit install curl`（已装）→`错误: 'curl' 已安装，如需重装请使用 --force`
- ✅ **install 不存在的包** — `hit install nonexistent_pkg`→`错误: 未找到软件 'nonexistent_pkg'`
- ✅ **静默家族（which/prefix/home/uninstall/reset 不存在时）** — 均正确报错
- ✅ **bucket add 已存在/remove 不存在** — 均正确报错
- ✅ **config set 校验** — maybe/abc/unknown_key 均正确报错
- ✅ **manifest 兼容性** — 0 WARN
