# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 第三轮自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。
> 上一轮（2026-06-28 第二轮）发现的 bug 中，6 类已修复迁移至 SOLVED_BUGS，本文件保留仍未修或修复不完整的。

---

## clap 错误被吞掉 ⭐⭐⭐⭐⭐

> **极为严重** —— `hit`（无子命令）/ `hit wrongcmd` / `hit install`（无参数）应分别由 clap 报 "requires subcommand" / "unrecognized subcommand" / "requires argument"，实际全无输出。第三轮实测仍复现。

### 现象

```
$ hit                → 应显示 help（arg_required_else_help），实际输出为空
$ hit wrongcmd       → 应报 "unrecognized subcommand 'wrongcmd'"，实际输出为空
$ hit install        → 应报 "requires at least one package name"，实际输出为空
```

### 根因

SOLVED_BUGS 记录 commit `f9cd803` 修复了 welcome 吞 clap 错误，但第二、三轮实测均未修。可能修复不完整，或 clap 的 `arg_required_else_help` / 错误子命令的处理路径仍被某处拦截。

### 修复方向

确认 `main.rs` 的 `Cli::parse()` 是否真正先于 welcome 执行，clap 解析失败应直接报错退出。验证：`hit wrongcmd` 应输出 clap 错误到 stderr 并非零退出。

### 证据

REPORT.md §19.1 / §19.2 / §19.3（全部输出为空）

---

## `hit install` 解压/同步阶段事务回滚 ⭐⭐⭐⭐⭐

> **极为严重** —— install 核心流程已部分修复（解析✔下载✔校验✔解压✔），但部分软件在解压/同步阶段事务回滚，install 失败。第三轮新发现。

### 现象

```
$ hit install curl   → 完整成功（解析✔下载✔校验✔解压✔同步✔提交✔），curl 8.21.0_1 装上 ✅
$ hit i jq           → 解析✔下载✔校验✔，解压时 "WARN 事务回滚 app=jq"，jq 未装上 ❌
$ hit install git    → 类似残留（§14 doctor 检测出 git 未跟踪应用目录）
$ hit install curl（第二次/升级）→ 同步时 junction 创建失败：
  "✘ 升级失败: IO 错误：创建 Junction: ...\curl\current -> ...\curl\8.21.0_1：Cannot create a file when that file already exists. (os error 183)"
```

### 根因

两类问题：
1. **jq 解压回滚** —— jq 是单 exe（`jq#1.8.2#abde28e.exe`），解压逻辑对 exe 类包可能处理不当，触发事务回滚。需查看回滚的具体错误（REPORT 只显示 "事务回滚 app=jq"，无具体原因）。
2. **重装/升级 junction 冲突** —— `hit install curl` 第二次执行时，`apps/curl/current` junction 已存在，install 试图重新创建 junction 但未先删除旧的，报 os error 183（Cannot create a file when that file already exists）。

### 修复方向

1. jq 解压回滚：定位事务回滚的具体错误原因，解压 exe 类包应正常处理。
2. junction 冲突：install 重装/升级时，应先删除旧 `current` junction 再创建新的（或用 replace junction 逻辑）。
3. 回归：`hit install jq` 后 `hit list` 出现 jq；`hit install curl --force` 升级不报 junction 错误。

### 证据

REPORT.md §5.2（jq 回滚）、§5.4（curl --force）、§8.3（update curl junction 失败）、§14.1（doctor 检测出 git/jq 未跟踪 + curl current 链接损坏）

---

## `hit uninstall` 大部分场景静默 ⭐⭐⭐⭐

> 第三轮实测仍复现。

### 现象

```
$ hit uninstall jq              → 无输出（jq 因回滚未装上，应报"'jq' 未安装"）
$ hit rm curl --purge           → "卸载 curl ..."后无 ✔ 完成提示，无 purge 确认
$ hit uninstall nonexistent     → 无输出（应报"'nonexistent' 未安装"）
$ hit uninstall                 → 无输出（应报"至少指定一个要卸载的软件名"）
$ hit uninstall jq curl         → 无输出（应依次处理或报错）
```

只有 `hit rm curl --purge` 输出了 "卸载 curl ..."，但无 ✔ 完成、无 purge 持久化数据删除提示，且后续 §10 cache list 仍显示 curl 缓存（purge 未生效？或缓存与 persist 是两回事）。

### 修复方向

1. uninstall 不存在的软件应报错"'<name>' 未安装"。
2. uninstall 无参数应报错"至少指定一个要卸载的软件名"。
3. uninstall 成功应输出 ✔ 完成提示。
4. `--purge` 应删除 persist 持久化数据并提示。

### 证据

REPORT.md §9.1-§9.5

---

## `hit reset` 仍静默 ⭐⭐⭐⭐

> 第三轮实测仍复现。hold/unhold 已修，但 reset 仍静默。

### 现象

```
$ hit reset python 3.11.0      → 无输出（python 未安装，应报错"'python' 未安装"或"版本 '3.11.0' 不存在"）
$ hit reset python 9.9.9       → 无输出（应报错"版本 '9.9.9' 不存在（python）"）
$ hit r nonexistent 1.0.0（alias）→ 无输出（应报错"'nonexistent' 未安装"）
```

### 根因

reset 在找不到目标软件/版本时静默返回，未报错。可能因为 install python@x 失败无目标，但 reset 对不存在的软件/版本必须报错。

### 修复方向

reset 对未安装的软件报错"'<name>' 未安装"；对不存在的版本报错"版本 '<ver>' 不存在（<name>）"；成功输出"✔ <name> 已切换到 <ver>"。

### 证据

REPORT.md §7.1.3 / §7.1.4 / §17-r

---

## `hit which` / `hit prefix <不存在>` / `hit home <不存在>` 静默 ⭐⭐⭐

> 第三轮实测：prefix <已安装> 已修，但 which 全静默、prefix <不存在> 静默、home <不存在> 静默。

### 现象

```
$ hit which curl          → 无输出（curl 已安装，应输出 Shim 路径和 Target 真实 exe 路径）
$ hit which nonexistent   → 无输出（应报错"未找到 'nonexistent' 的 shim 文件"）
$ hit prefix curl         → 输出 apps\curl 路径  ✅ 已修
$ hit prefix nonexistent  → 无输出（应报错"'nonexistent' 未安装"）
$ hit home nonexistent    → 只有 WARN 无业务输出（应报错"未找到软件 'nonexistent'"）
```

### 修复方向

which 对已安装软件输出 shim 路径 + target exe 路径；对不存在软件报错。prefix/home 对不存在软件报错。

### 证据

REPORT.md §12.1.1 / §12.1.2 / §12.2.3 / §12.3.2

---

## `hit config set` 校验失效 ⭐⭐⭐⭐

> 第三轮实测仍复现。写入已修（§13.10 显示新值），但校验仍失效。

### 现象

```
$ hit config set aria2_enabled maybe      → 无输出（应报错"'maybe' 不是有效的布尔值"）
$ hit config set auto_cleanup_days abc    → 无输出（应报错"'abc' 不是有效的数字"）
$ hit config set unknown_key value        → 无输出（应报错"未知配置项"）
```

校验失败时无任何错误输出，静默接受。

### 修复方向

config set 必须校验：布尔项只接受 true/false/yes/no；数字项只接受有效数字；未知键报错。校验失败应输出错误信息到 stderr 并非零退出。

### 证据

REPORT.md §13.5 / §13.7 / §13.8（校验失效，无任何输出）

---

## `hit bucket add` 已存在 / `hit bucket remove` 不存在 输出为空 ⭐⭐⭐

> 第三轮实测仍复现。

### 现象

```
$ hit bucket add main      （main 已存在）→ 无任何输出（应报错"Bucket 'main' 已存在"）
$ hit bucket add unknownbucket             → 无输出（应报错并提示用法）
$ hit bucket remove myrepo  （不存在）     → 无输出（应报错"Bucket 'myrepo' 不存在"）
$ hit bucket remove nonexistent            → 无输出（应报错"Bucket 'nonexistent' 不存在"）
```

### 修复方向

add 已存在应报错"Bucket '<name>' 已存在"；add 未知名称应报错并提示用法；remove 不存在应报错"Bucket '<name>' 不存在"。全部非零退出。

### 证据

REPORT.md §2.1.1 / §2.1.4 / §2.1.6 / §2.4.1 / §2.4.3

---

## Manifest 兼容性仍有 6 条遗漏 ⭐⭐

> 第三轮实测：仍 6 条 WARN，与第二轮相同。

### 现象

每条 search/info 命令仍输出 6 条 WARN：

| manifest | 错误 | 根因 |
|---|---|---|
| `extras/bucket/megasync.json` L6 | invalid type: sequence, expected a string | 字段多态未支持 |
| `extras/bucket/bizhawk.json` L5 | 同上 | 字段多态未支持 |
| `extras/bucket/filezilla.json` L8 | 同上 | 字段多态未支持 |
| `extras/bucket/tablacus-explorer.json` L5 | 同上 | 字段多态未支持 |
| `extras/bucket/irfanview.json` L92 | HashField 不支持对象形式 | autoupdate.hash 对象未支持 |
| `main/bucket/qrencode.json` L89 | 同上 | autoupdate.hash 对象未支持 |

### 修复方向

1. megasync/bizhawk/filezilla/tablacus-explorer 第 5/6/8 行的具体字段改为 StringOrVec。
2. irfanview/qrencode 的 `autoupdate.architecture.<arch>.hash` 对象形式（`{url, jsonpath}`）未覆盖。
3. 回归：对 main+extras+versions 全量解析，要求 0 WARN。

### 证据

REPORT_warn.log 全程

---

## 已修复（迁移至 SOLVED_BUGS）✅

以下 bug 在本轮实测中已验证修复，详见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)：

- ✅ **§4 info 完全不工作** —— 现输出名称/版本/描述/主页/许可证/架构/依赖/Bucket 全字段
- ✅ **§5 install 完全不工作（核心流程）** —— curl 完整装上（解析✔下载✔校验✔解压✔同步✔提交✔），list 显示 curl 8.21.0_1（但 jq 回滚/junction 冲突见上）
- ✅ **§7.2 hold/unhold 静默** —— 输出 🔒 'curl' 已锁定 / ⏭ 已经是锁定状态 / 🔓 已解除锁定
- ✅ **§16 status 统计与 bucket list 不一致** —— Bucket 数量 3 / 可用软件总数 4500 / 已安装软件 1，与 bucket list 一致
- ✅ **§17-b `hit b ls` 与 `bucket list` 不一致** —— 都显示 3 个 bucket
- ✅ **§10 cache list 正常** —— 显示 curl/git/jq 3 个缓存文件及大小
- ✅ **§12.2.2 `hit prefix curl`** —— 输出 apps\curl 路径
- ✅ **§14.1 doctor 正常** —— 检测出 3 个问题（curl current 链接损坏 / git jq 未跟踪应用目录），提示用 --fix
- ✅ **§18 -v/-vv/-vvv 日志级别** —— 不再被 welcome 污染，正常输出 list
- ✅ **welcome 错误触发（第二轮已修）** —— 第三轮确认所有命令无 ASCII 横幅
- ✅ **config set 写入（第二轮已修）** —— §13.10 显示 aria2_enabled=true / auto_cleanup_days=60，与 set 一致
