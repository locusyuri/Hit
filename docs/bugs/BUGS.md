# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 第二轮自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。
> 上一轮（2026-06-27）发现的 10 类 bug 中，5 类已修复迁移至 SOLVED_BUGS，本文件保留仍未修或修复不完整的。

---

## `hit install` 完全不工作 ⭐⭐⭐⭐⭐

> **极为严重** —— 安装是包管理器的核心功能，现在完全不工作。第二轮实测仍复现。

### 现象

```
$ hit install curl
安装 curl ...
                        ← 然后什么都没发生，无下载、无解压、无 shim

$ hit list
没有已安装的软件         ← curl 没装上
```

`hit install curl` / `hit i jq` / `hit install curl --force` / `hit install main/git` / `hit install jq --arch 64bit` 全部同样：只输出 "安装 xxx ..." 然后立即结束，没有任何后续动作。`hit install nonexistent_pkg` 也只输出 "安装 nonexistent_pkg ..." 后无下文——应报错"未找到软件"。

### 根因

install 命令在打印 "安装 xxx ..." 之后中断。可能原因：
1. 解析 manifest 后静默返回（找不到包不报错）
2. 下载/解压逻辑 panic 被吞
3. control flow 在 "安装 xxx ..." 之后直接 return，未调用实际安装函数

### 修复方向

1. 在 install 实现里加 tracing 定位 "安装 xxx ..." 之后停在哪一步。
2. install 失败必须有错误输出——找不到包报"未找到软件"，下载失败报下载错误。
3. install 成功必须有 ✔ 完成提示 + shim 创建。
4. 回归：`hit install curl` 后 `hit list` 出现 curl，`hit which curl` 输出 shim 路径，`curl --version` 能运行。

### 证据

REPORT.md §5.1-§5.7、§6.1（"没有已安装的软件"）、§10-pre（"安装 curl ..." 后无下文，§10.1 cache list "缓存为空"）

---

## `hit info` 完全不工作 ⭐⭐⭐⭐⭐

> **极为严重** —— info 命令对任何输入都无业务输出。第二轮实测仍复现。

### 现象

```
$ hit info git              → 只有 WARN，无业务输出
$ hit info git --bucket main → 只有 WARN，无业务输出
$ hit info nonexistent       → 只有 WARN，无业务输出（应报"未找到软件"）
$ hit info curl              → 只有 WARN，无业务输出（curl 在 main+extras 都有，应报"多 bucket 请用 --bucket 指定"）
```

### 修复方向

1. 定位 info 实现，确认 manifest 查到后是否调用了打印函数。
2. info 至少输出：名称、版本、描述、主页、许可证、架构、依赖、Bucket（TEST_FLOW.md §4.1）。
3. `hit info nonexistent` 报错"未找到软件 'nonexistent'"。
4. `hit info curl`（多 bucket 同名）报错"在多个 bucket 中找到 ... 请使用 --bucket 指定"。

### 证据

REPORT.md §4.1-§4.4（全部输出为空，只有 WARN）

---

## clap 错误被吞掉 ⭐⭐⭐⭐⭐

> **极为严重** —— `hit`（无子命令）/ `hit wrongcmd` / `hit install`（无参数）应分别由 clap 报 "requires subcommand" / "unrecognized subcommand" / "requires argument"，实际全无输出。第二轮实测仍复现。

### 现象

```
$ hit                → 应显示 help（arg_required_else_help），实际输出为空
$ hit wrongcmd       → 应报 "unrecognized subcommand 'wrongcmd'"，实际输出为空
$ hit install        → 应报 "requires at least one package name"，实际输出为空
```

### 根因

上一轮 SOLVED_BUGS 记录 commit `f9cd803` 修复了 welcome 吞 clap 错误，但本轮实测验证仍未修——可能修复不完整，或构建未部署到测试环境。

### 修复方向

确认 `main.rs` 的 `Cli::parse()` 是否真正先于 welcome 执行，clap 解析失败应直接报错退出。验证：`hit wrongcmd` 应输出 clap 错误到 stderr 并非零退出。

### 证据

REPORT.md §19.1 / §19.2 / §19.3（全部输出为空）

---

## `hit reset` / `hit hold` / `hit unhold` 全部输出为空 ⭐⭐⭐⭐

> 第二轮实测仍复现。SOLVED_BUGS 记录"随 welcome 修复自动恢复"，但本轮 welcome 已修好、这些命令仍静默。

### 现象

```
$ hit reset python 3.11.0      → 无输出（应"✔ python 已切换到 3.11.0" 或报错）
$ hit reset python 9.9.9       → 无输出（应报错"版本 '9.9.9' 不存在"）
$ hit hold curl                → 无输出（应"🔒 'curl' 已锁定" 或报错"未安装"）
$ hit hold nonexistent         → 无输出（应报错"'nonexistent' 未安装"）
$ hit unhold curl              → 无输出
$ hit r nonexistent 1.0.0（alias）→ 无输出
```

### 根因

SOLVED_BUGS 说"代码逻辑完整，是 welcome 副作用"——但本轮 welcome 已修好，这些命令仍静默。说明根因不是 welcome，而是 reset/hold/unhold 本身的代码逻辑在找不到目标软件时静默返回（因为 install 没装上任何软件，reset/hold/unhold 找不到目标后无输出）。

### 修复方向

reset/hold/unhold 对未安装的软件必须报错（如"'curl' 未安装"），对成功操作必须输出 ✔ 提示。不能静默退出。

### 证据

REPORT.md §7.1.3 / §7.1.4 / §7.2.1 / §7.2.2 / §7.2.4 / §7.2.5 / §7.2.6 / §17-r

---

## `hit config set` 校验失效 ⭐⭐⭐⭐

> 第二轮实测：写入已修（§13.10 显示新值），但校验仍失效。

### 现象

```
$ hit config set aria2_enabled maybe      → 无输出（应报错"'maybe' 不是有效的布尔值"）
$ hit config set auto_cleanup_days abc    → 无输出（应报错"'abc' 不是有效的数字"）
$ hit config set unknown_key value        → 无输出（应报错"未知配置项"）
$ hit config set aria2_enabled true       → "✔ 配置 'aria2_enabled' 已更新为 'true'"  ✅
$ hit config list                         → aria2_enabled true / auto_cleanup_days 60    ✅ 写入生效
```

写入问题已修（§13.10 显示与 §13.3/§13.6 一致），但**校验仍失效**：maybe/abc/unknown_key 都不报错，静默接受。

### 修复方向

config set 必须校验：布尔项只接受 true/false/yes/no；数字项只接受有效数字；未知键报错。校验失败应输出错误信息到 stderr 并非零退出。

### 证据

REPORT.md §13.5 / §13.7 / §13.8（校验失效，无任何输出）

---

## `hit which` / `hit prefix <pkg>` / `hit home <pkg>` 输出为空 ⭐⭐⭐

> 第二轮新发现。

### 现象

```
$ hit which curl          → 无输出（应输出 Shim 路径和 Target 真实 exe 路径）
$ hit which nonexistent   → 无输出（应报错"未找到 'nonexistent' 的 shim 文件"）
$ hit prefix curl         → 无输出（应输出 …\apps\curl 路径）
$ hit prefix nonexistent  → 无输出（应报错"'nonexistent' 未安装"）
$ hit home nonexistent    → 只有 WARN 无业务输出（应报错"未找到软件 'nonexistent'"）
```

注：`hit prefix`（无参数）正常输出根目录，但 `hit prefix <pkg>` 静默。

### 根因

which/prefix/home 在找不到目标软件时静默返回，未报错。部分（which）可能是 install 未工作导致无 shim 可查，但 which nonexistent 应报错而非静默。

### 修复方向

which/prefix/home 对未安装/不存在的软件必须报错，不能静默退出。

### 证据

REPORT.md §12.1.1 / §12.1.2 / §12.2.2 / §12.2.3 / §12.3.2

---

## `hit bucket add` 已存在 / `hit bucket remove` 不存在 输出为空 ⭐⭐⭐

> 第二轮实测仍复现。

### 现象

```
$ hit bucket add main      （main 已存在）→ 无任何输出（应报错"Bucket 'main' 已存在"）
$ hit bucket add unknownbucket             → 无输出（应报错并提示用法）
$ hit bucket remove myrepo  （不存在）     → 无输出（应报错"Bucket 'myrepo' 不存在"）
$ hit bucket remove nonexistent            → 无输出（应报错"Bucket 'nonexistent' 不存在"）
```

注：`hit bucket rm main`（main 存在）正常输出"✔ bucket 'main' 已移除"，说明 remove 成功路径 OK，只是"不存在"路径静默。

### 修复方向

add 已存在应报错"Bucket '<name>' 已存在"；add 未知名称应报错并提示用法；remove 不存在应报错"Bucket '<name>' 不存在"。全部非零退出。

### 证据

REPORT.md §2.1.1 / §2.1.4 / §2.1.6 / §2.4.1 / §2.4.3

---

## `hit status` 统计与 `hit bucket list` 不一致 ⭐⭐⭐

> 第二轮实测：部分修复（不再是 0），但仍不一致。

### 现象

```
$ hit bucket list   → extras 2321 + main 1593 + versions 592 = 3 个 bucket, 4506 manifest
$ hit status        → Bucket 数量: 2 / 可用软件总数: 2908
```

status 显示 2 个 bucket（少了 main——因测试中 main 被某次 update 网络失败后从 list 消失？），可用软件总数 2908（=2321+extras+? 算法不明）。status 的统计与 bucket list 的数据源仍不统一。

### 修复方向

status 的 bucket 数应与 `hit bucket list` 的总数一致，可用软件数应与各 bucket manifest 总数一致。统一数据源。另外 bucket update 失败不应导致 bucket 从 list 消失。

### 证据

REPORT.md §16.1（Bucket 数量 2、可用软件总数 2908）vs §2.2.1（3 个 bucket、4506 manifest）

---

## `hit cleanup --cache` 输出为空 ⭐⭐

> 第二轮新发现。

### 现象

```
$ hit cleanup python   → "没有需要清理的内容"  ✅
$ hit cleanup --all    → "没有需要清理的内容"  ✅
$ hit cleanup --cache  → 无输出（应至少提示"已清理 N 个缓存文件"或"没有可清理的缓存"）
$ hit cleanup          → "没有需要清理的内容"  ✅
```

`cleanup --cache` 单独静默，其他 cleanup 子命令正常。

### 证据

REPORT.md §11.3（输出为空）vs §11.1/§11.2/§11.4（正常）

---

## Manifest 兼容性仍有少量遗漏 ⭐⭐

> 第二轮实测：大幅改善（从 ~1500 行 WARN 降到 6 条），但仍有遗漏。

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

1. 找到 megasync/bizhawk/filezilla/tablacus-explorer 第 5/6/8 行的具体字段（`invalid type: sequence, expected a string` 指向某个声明为 String 但实际是数组的字段），改为 StringOrVec。
2. irfanview/qrencode 的 `autoupdate.architecture.<arch>.hash` 仍是 `{url, jsonpath}` 对象形式，HashField::Fetch 未覆盖该路径（cf20905 修了顶层 hash，autoupdate 内可能漏了）。
3. 回归：对 main+extras+versions 全量解析，要求 0 WARN。

### 证据

REPORT_warn.log 全程（每条 search/info 命令都有这 6 条 WARN）

---

## `hit bucket update` 偶发网络失败 ⭐⭐

> 第二轮实测：大幅改善（§2.3.1 全 3/3 成功），但偶发失败。

### 现象

```
$ hit bucket update --all   → extras ✔ / main ✘ "An IO error occurred when talking to the server" / versions ✔
$ hit bucket update         → extras ✔ / versions ✔（main 未更新，因上次失败被跳过？）
```

bucket update 的 git clone 偶发失败，错误信息 "An IO error occurred when talking to the server" 模糊。同一 bucket 时成时败。

### 修复方向

1. 改进错误信息——附带 HTTP 状态码、URL，便于定位。
2. 考虑重试机制——网络失败时自动重试 2-3 次。
3. 确认 config 的 proxy 字段是否被 update 使用。

### 证据

REPORT.md §7.2.3（main 失败）、§8.2（main 被跳过）

---

## 已修复（迁移至 SOLVED_BUGS）✅

以下 bug 在本轮实测中已验证修复，详见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)：

- ✅ **welcome 错误触发污染所有命令输出** —— §2.3.3/§2.4.1/§2.4.3/§8/§9/§10/§11/§13/§14/§16/§17/§18 均无 ASCII 横幅
- ✅ **§2.3 bucket update 全成功** —— §2.3.1 显示 3/3 ✔（偶发失败见上）
- ✅ **§2.4.2 `hit bucket rm main`** —— 正确输出"✔ bucket 'main' 已移除"
- ✅ **§13 config set 写入生效** —— §13.10 显示 aria2_enabled=true / auto_cleanup_days=60，与 §13.3/§13.6 一致
- ✅ **§16 status 不再全 0** —— 显示 Bucket 数量 2 / 可用软件总数 2908（虽仍不一致，见上）
- ✅ **§3 search 大幅恢复** —— §3.1 search git 返回 162 个结果（含 git 本身），§3.5 未找到正确提示
- ✅ **§10 cache 全部正常** —— cache list/dir/clean 输出正确
- ✅ **§14 doctor 正常** —— "✔ 系统健康，无问题"
- ✅ **§17 alias 大部分正常** —— hit i/s/u/ls/st/b/c 触发正确（rm/r 静默见上）
- ✅ **§18 -v/-vv/-vvv 不再被 welcome 污染** —— 输出"没有已安装的软件"（因 install 未工作）
