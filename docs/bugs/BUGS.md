# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 第四轮自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。

---

## 搜索含描述匹配导致大量无关结果 ⭐⭐⭐⭐⭐

> **极为严重** —— `hit search git` 返回 162 个结果，其中 caesium-image-compressor、cdex、digital 等大量软件完全不包含 "git" 名称，仅描述中含 "digital" 等被模糊匹配。用户反馈。

### 现象

```
$ hit search git
# 结果包含：
# caesium-image-compressor - "...digital pictures..."
# cdex - "Digital Audio CD Extractor"
# digital - "A digital logic designer..."
# ...
共 162 个结果
```

而仅匹配名称本应只有 ~20 个结果（git、git-absorb、git-annex 等）。

### 根因

搜索逻辑改为同时匹配名称和描述，但未做优先级排序或精准匹配。描述中任何一个词匹配关键词就会被返回，导致搜索 `git` 返回描述含 `digital`、`digitally`、`digitial` 等无关结果。

### 修复方向

1. 匹配优先级：名称匹配 > 描述含关键词作为关键词匹配（而非子串匹配）
2. 或默认仅匹配名称，`--include-description` 参数才搜索描述
3. 参考原版 Scoop：`search` 默认仅搜索名称，`search -d` 才搜描述

### 证据

REPORT.md §3.1（162 个结果，大量 "digital" 类无关软件）、REPORT.md §3.4（--bucket main 同样 91 个）

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐

> **极为严重** —— 用户反馈 `hit si 7` 直接安装 7zip，而非启动交互式 TUI。`hit si` 完全背离设计目标。

### 现象

```
$ hit si 7
安装 7zip ...
▶ [解析] 7zip...
✔ [解析] 7zip 完成
...
```

`si` 命令被当作 `i`（install）的 alias 执行，直接把参数作为软件名安装，而非启动交互式搜索 TUI。

### 根因

`si` 别名解析到 `i`（install）而非独立的 `si`（interactive search）命令。可能在 clap 的 alias 配置中 `si` 被错误映射到 `i`。

### 修复方向

确保 `hit si` 命令解析到独立的交互式搜索实现（TUI），而非 `hit install` 的别名。`hit si` 应启动 TUI 界面（如 §15 所述），而非静默安装第一个结果。

### 证据

用户描述的 `hit si 7` 输出；测试证实 `hit i nonexistent_alias_test`（§17-i）也触发 install 路径（走 install 流程）。

---

## 安装 post_install 脚本被 cmd.exe 执行而非 PowerShell ⭐⭐⭐⭐⭐

> **极为严重** —— 包含 `post_install` 的软件（如 7zip、git）在安装时，PowerShell 脚本被传给 cmd.exe 执行，`$7zip_dir` 变量无法识别导致失败。用户反馈+实测证实。

### 现象

```
$ hit install 7zip
▶ [解析] 7zip... ✔
▶ [下载] 7zip... ✔
▶ [校验] 7zip... ✔
解压 7zip (7zip#26.02#63f4002.msi)
▶ [解压] 7zip... ✔
▶ [同步] 7zip...
▶ [提交] 7zip... ✔
'$7zip_dir' is not recognized as an internal or external command,
operable program or batch file.       ← cmd.exe 在执行 PowerShell 脚本！
WARN 事务回滚 app=7zip
错误: 安装 '7zip' 失败：PostInstall 脚本退出码：1
```

git 同样：
```
▶ [解压] git... ✔
WARN 事务回滚 app=git
```

这是因为 `post_install` 脚本包含 PowerShell 语法（`$7zip_dir`、`Set-Content` 等），但 Hit 用 `cmd.exe` 调用它们而非 `pwsh/powershell`。

### 修复方向

1. post_install 脚本必须通过 `pwsh -NoProfile -File` 执行，而非直接 `cmd /c`。
2. 也可通过 `powershell -NoProfile -File` 执行（兼容 PowerShell 5.1）。
3. 回归：`hit install 7zip` 应成功，post_install 正常执行。

### 证据

用户反馈的报告 + REPORT.md §10.1（cache 有 7zip 已下载）+ §14 doctor 显示 "✗ 7zip: 未跟踪的应用目录"

---

## Junction 创建失败（重装/升级/doctor --fix） ⭐⭐⭐⭐⭐

> **极为严重** —— 已安装软件重装、升级或 `doctor --fix` 修复时，因旧 `current` junction 未删除，创建新 junction 时报 os error 183（文件已存在）。

### 现象

```
$ hit install curl --force（curl 已装）
▶ [提交] curl... → WARN 事务回滚 app=curl

$ hit update --force
升级 curl...
▶ [提交] curl... ✔
▶ [同步] jq...
✘ 升级失败: IO 错误：创建 Junction: ...\jq\current -> ...\jq\1.8.2：
Cannot create a file when that file already exists. (os error 183)

$ hit doctor --fix
✗ jq 修复失败: Cannot create a file when that file already exists. (os error 183)
```

### 根因

install/update/doctor --fix 尝试创建 `apps/<name>/current` junction 时，未先删除已存在的旧 junction（或 SymbolicLink 替代方案），导致 `CreateJunction` API 返回 error 183。

### 修复方向

创建 junction 前应先检查是否存在，若存在则删除后再创建（或使用 `replace` 模式）。所有创建 junction 的代码路径统一处理。

### 证据

REPORT.md §5.4（curl --force 回滚）、§8.5（jq 升级 junction 失败）、§14.3（doctor --fix jq 修复失败）

---

## clap 错误仍被吞掉 ⭐⭐⭐⭐⭐

> **极为严重** —— §19.1-§19.3 输出全部为空。仅 clap 的 `--help` 正常。

### 现象

```
$ hit                → 输出为空（应显示 help）
$ hit wrongcmd       → 输出为空（应报 "unrecognized subcommand"）
$ hit install        → 输出为空（应报 "requires at least one package name"）
```

### 证据

REPORT.md §19.1 / §19.2 / §19.3

---

## install/update 遇到已安装软件时静默退出 ⭐⭐⭐

### 现象

```
$ hit install curl          （curl 已装）→ "安装 curl ..." 后无任何后续
$ hit install jq --arch 64bit（jq 已装）→ "安装 jq ..." 后无任何后续
$ hit install nonexistent_pkg            → "安装 nonexistent_pkg ..." 后无任何后续
$ hit install python@3.11.0              → 完全无输出
```

重复安装时应提示"已安装，如需重装请用 --force" 或直接报错。不存在的包应报"未找到软件"。

### 证据

REPORT.md §5.3 / §5.7 / §5.6 / §7.1.1 / §7.1.2

---

## `hit which` / `hit prefix <不存在>` / `hit home <不存在>` / `hit uninstall <不存在>` 仍静默 ⭐⭐⭐

### 现象

```
$ hit which curl          → 无输出（curl 已装，应输出 shim 路径）
$ hit which nonexistent   → 无输出
$ hit prefix nonexistent  → 无输出
$ hit home nonexistent    → 只有 WARN 无业务输出
$ hit uninstall jq        → "卸载 jq ..." 无 ✔ 完成提示（上次已修）
$ hit uninstall nonexistent → 无输出
$ hit reset python 3.11.0 → 无输出
$ hit r nonexistent 1.0.0 → 无输出
```

注：`hit rm curl --purge` 已修（输出"✔ curl 已卸载" ✅）
注：`hit prefix curl` 已修（输出 apps\curl 路径 ✅）

### 证据

REPORT.md §12.1 / §12.2.3 / §12.3.2 / §9.3 / §7.1.3 / §17-r

---

## `hit config set` 校验仍失效 ⭐⭐⭐

### 现象

```
$ hit config set aria2_enabled maybe      → 无输出（应报错）
$ hit config set auto_cleanup_days abc    → 无输出（应报错）
$ hit config set unknown_key value        → 无输出（应报错）
```

注：写入路径已修，§13.10 与 §13.3/§13.6 值一致 ✅

### 证据

REPORT.md §13.5 / §13.7 / §13.8

---

## `hit bucket add` 已存在 / `hit bucket remove` 不存在 仍静默 ⭐⭐⭐

### 现象

```
$ hit bucket add main           → 无输出（main 已存在，应报错）
$ hit bucket add unknownbucket  → 无输出（应报错提示用法）
$ hit bucket remove myrepo      → 无输出（应报错"不存在"）
$ hit bucket remove nonexistent → 无输出
```

注：`hit bucket rm main`（main 存在）正常输出"✔ bucket 'main' 已移除" ✅

### 证据

REPORT.md §2.1.1 / §2.1.4 / §2.1.6 / §2.4.1 / §2.4.3

---

## Manifest 兼容性仍有 6 条 WARN ⭐⭐

### 现象

每条 search/info 命令仍输出 6 条 WARN：
- `megasync.json` / `bizhawk.json` / `filezilla.json` / `tablacus-explorer.json` — invalid type: sequence
- `irfanview.json` / `qrencode.json` — HashField 对象形式未支持

### 证据

REPORT_warn.log 全程

---

## 已修复（迁移至 SOLVED_BUGS）✅

- ✅ **§5 install jq 回滚** — jq 1.8.2 完整安装成功 {"✔ jq 1.8.2 安装完成（1）"}
- ✅ **§9.2 `hit rm curl --purge`** — 输出 "✔ curl 已卸载"
- ✅ **§10.3 `hit cache clean`** — "✔ 已清理 4 个缓存文件"
- ✅ **§14 doctor 正常检测问题** — 检测到 4 个问题
- ✅ **§16 status 正确** — 已安装 1 / Bucket 3 / 可用软件 4506
- ✅ **§17-b `hit b ls`** — 3 个 bucket
- ✅ **§18 日志级别** — 正常
- ✅ **config 写入** — 正常
- ✅ **info** — 全字段正常输出
- ✅ **bucket update** — 3/3 ✔
- ✅ **hold/unhold** — 🔒/⏭/🔓
