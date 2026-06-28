# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-27 自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。

---

## `hit install` 完全不工作 ⭐⭐⭐⭐⭐

> **极为严重** —— 安装是包管理器的核心功能，现在完全不工作。

### 现象

```
$ hit install curl
安装 curl ...
                        ← 然后什么都没发生，无下载、无解压、无 shim

$ hit list
没有已安装的软件         ← curl 没装上
```

`hit install curl` / `hit i jq` / `hit install curl --force` / `hit install main/git` / `hit install jq --arch 64bit` 全部同样：只输出 "安装 xxx ..." 然后立即结束，没有任何后续动作（无下载进度、无哈希校验、无解压、无创建 shim、无 ✔ 完成提示）。`hit list` 显示"没有已安装的软件"。

`hit install nonexistent_pkg` 也只输出 "安装 nonexistent_pkg ..." 然后无下文——应报错"未找到软件"。

### 根因

install 命令的执行链在打印 "安装 xxx ..." 之后中断。可能原因：
1. 解析 manifest 失败后静默返回（无错误输出）—— 因为 manifest 解析有 bug（见已修复的兼容性条目，但仍可能有个别 manifest 解析失败导致 install 找不到包后静默退出）
2. install 的下载/解压逻辑 panic 后被吞掉
3. install 命令的 control flow 在 "安装 xxx ..." 之后直接 return，未调用实际的安装函数

### 修复方向

1. 在 install 命令的实现里加 tracing 日志（或临时 eprintln），定位 "安装 xxx ..." 之后执行到哪一步停了。
2. install 失败必须有错误输出 —— 找不到包报"未找到软件 'xxx'"，下载失败报下载错误，不能静默退出。
3. install 成功必须有 ✔ 完成提示 + shim 创建。
4. 回归：`hit install curl` 后 `hit list` 应出现 curl，`hit which curl` 应输出 shim 路径，`curl --version` 应能运行。

### 证据

REPORT.md §5.1-§5.7、§6.1（"没有已安装的软件"）、§10-pre（"安装 curl ..." 后无下文，§10.1 cache list "缓存为空"）

---

## `hit info` 完全不工作 ⭐⭐⭐⭐⭐

> **极为严重** —— info 命令对任何输入都无业务输出。

### 现象

```
$ hit info git              → 只有 WARN，无业务输出
$ hit info git --bucket main → 只有 WARN，无业务输出
$ hit info nonexistent       → 只有 WARN，无业务输出（应报"未找到软件"）
$ hit info curl              → 只有 WARN，无业务输出（curl 在 main+extras 都有，应报"多 bucket 请用 --bucket 指定"）
```

### 根因

info 命令查到 manifest 后没有打印任何字段（名称/版本/描述/主页/许可证/架构/依赖/Bucket）。可能是：
1. info 的输出函数被误删或被 welcome 吞掉（但 info 在 welcome 触发之前就无输出，不像 welcome 问题）
2. info 从 manifest 提取字段的逻辑返回了空结构，打印空内容
3. info 的打印逻辑依赖某个被 welcome 改动影响的全局状态

### 修复方向

1. 定位 info 命令实现，确认 manifest 查到后是否调用了打印函数。
2. info 至少应输出：名称、版本、描述、主页、许可证、架构、依赖、Bucket（TEST_FLOW.md §4.1）。
3. `hit info nonexistent` 应报错"未找到软件 'nonexistent'"。
4. `hit info curl`（多 bucket 同名）应报错"在多个 bucket 中找到 ... 请使用 --bucket 指定"。

### 证据

REPORT.md §4.1-§4.4（全部输出为空，只有 WARN）

---

## `hit bucket add main`（已存在时）输出为空 ⭐⭐⭐

### 现象

```
$ hit bucket add main      （main 已存在）
                        ← 无任何输出，无错误
```

应报错"Bucket 'main' 已存在"（之前版本是有这个错误的，见 REPORT §2.1.4 之前会话的输出）。

### 根因

可能是 welcome 改动副作用 —— welcome 触发后"已跳过"退出，没走到 add 的"已存在"检查。或者 add 的"已存在"检查被删了。

### 修复方向

`hit bucket add <已存在的 bucket>` 应报错 "Bucket '<name>' 已存在" 并以非零退出码退出。

### 证据

REPORT.md §2.1.1（输出完全为空）、§2.1.4（输出完全为空）

---

## `hit reset` / `hit hold` / `hit unhold` 全部输出为空 ⭐⭐⭐⭐

> **已解决** — 代码逻辑完整,是 Bug A(welcome 污染 stdout)的副作用。修完 A+B 后应恢复,待回归验证。

---

## `hit config set` 校验失效 + 声称成功但不写入 ⭐⭐⭐⭐

> **已解决** — 校验逻辑代码本身完整,校验失效是 Bug A(welcome 污染)的副作用;写入不一致由 `default_path()` 路径解析修复 + 放弃 hit 自身 shim 解决。

---

## `hit status` 显示 Bucket 数量为 0（实际有 3 个）⭐⭐⭐

### 现象

```
$ hit status
Hit 0.1.0

  已安装软件:    0
  Bucket 数量:   0      ← 实际有 3 个 bucket（main/extras/versions）
  可用软件总数:  0      ← 实际有 4502 个 manifest
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit
```

`hit bucket list` 同环境显示 3 个 bucket、共 4502 manifest，但 `hit status` 全显示 0。

### 根因

status 统计 bucket 数 / 可用软件数时，读的数据源与 bucket list 不一致。可能 status 读的是 db.json（为空），bucket list 读的是 buckets/ 目录扫描。两边数据源不统一。

### 修复方向

status 的 bucket 数应与 `hit bucket list` 的总数一致，可用软件数应与各 bucket manifest 总数一致。统一数据源。

### 证据

REPORT.md §16.1 / §16.2（Bucket 数量 0、可用软件总数 0）vs §2.2.1（3 个 bucket、extras 2319 + main 1591 + versions 592 = 4502）

---

## `hit b ls`（alias）显示"没有已添加的 Bucket"但 `hit bucket list` 正常 ⭐⭐⭐

### 现象

```
$ hit bucket list          → 显示 extras/main/versions 3 个 bucket
$ hit b ls                 → "没有已添加的 Bucket"
```

同一环境，alias `b ls` 与完整命令 `bucket list` 结果不一致。

### 根因

alias `b` 解析后可能没正确转发到 `bucket list`，而是转发到了别的子命令（如 `bucket` 无参数，显示空）。或 alias 解析后子命令参数丢失。

### 修复方向

`hit b ls` 必须与 `hit bucket list` 输出完全一致。检查 alias 解析是否保留了子命令参数。

### 证据

REPORT.md §17-b（"没有已添加的 Bucket"）vs §2.2.2（`hit b ls` 之前会话正常显示 3 个）—— 注意本会话 §2.2.2 没跑（脚本里在 §2.2.2 之前 welcome 就开始污染了，但 §2.2.1 正常）。对比 §2.2.1 与 §17-b。

---

## `hit bucket update` 频繁网络失败 ⭐⭐

### 现象

```
$ hit bucket update
正在更新 bucket 'extras'...
  ✘ extras 失败: Bucket 'extras' 错误：克隆失败：Could not decode server reply
正在更新 bucket 'main'...
  ✔ main
正在更新 bucket 'versions'...
  ✘ versions 失败: Bucket 'extras' 错误：克隆失败：Could not decode server reply
✔ Bucket 更新完成（1/3）

$ hit bucket update main
  ✘ main 失败: Bucket 'main' 错误：克隆失败：An IO error occurred when talking to the server
```

bucket update 的 git clone 频繁失败，错误信息 "Could not decode server reply" / "An IO error occurred when talking to the server" 模糊。同一 bucket 时成时败（main 第一次成、第二次败）。

### 根因

可能是 git 克隆对 GitHub 的网络不稳定（GFW？），但错误信息太模糊无法定位。也可能是 hit 的 git 客户端（gix）对某些 GitHub 响应处理有 bug。

### 修复方向

1. 改进错误信息 —— "Could not decode server reply" 应附带 HTTP 状态码、响应头、URL，便于定位。
2. 考虑重试机制 —— 网络失败时自动重试 2-3 次。
3. 考虑支持配置代理（config 已有 proxy 字段，但 update 是否用上了？）。

### 证据

REPORT.md §2.3.1（extras/versions 失败）、§2.3.2（main 失败）、§8.2（main 失败）

---

## Manifest 兼容性仍有少量遗漏 ⭐⭐

> 已修复大部分（cf20905），但仍有少量 manifest 被跳过。

### 现象

单次 `hit search git` 仍输出 1 条 WARN：

```
WARN 跳过无效 manifest '...\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12
```

extras bucket 的 `megasync.json` / `bizhawk.json` / `filezilla.json` / `irfanview.json` / `tablacus-explorer.json` 也仍被跳过（invalid type: sequence, expected a string at line N column 4 —— 仍是字段多态问题）。

### 根因

cf20905 修了 `suggest` 改 Vec，但 `suggest` 在某些 manifest 里是对象形式（`{"JDK": ["java/opendk", "java/oraclejdk"]}`），对象内的数组仍可能未被正确处理。qrencode.json 的 `autoupdate.architecture.64bit.hash` 仍是 `{url, jsonpath}` 对象形式未支持。

### 修复方向

1. `qrencode.json` 第 89 行的 HashField 仍需扩展 —— autoupdate 块里的 hash 对象形式（`{url, jsonpath}`）还没支持全。
2. megasync/bizhawk/filezilla/irfanview/tablacus-explorer 的 `invalid type: sequence, expected a string at line N column 4` —— 找到具体是哪个字段（第 N 行第 4 列），把它改成 StringOrVec。
3. 回归：对 main+extras+versions 全量解析，要求 0 WARN。

### 证据

REPORT_warn.log 全程（每条 search/info 命令都有 qrencode.json 的 WARN）；REPORT.md §3.1 开头的 WARN

---

## 设计问题：bucket add 未知 bucket 提示用法示例 ⭐

（之前已记录，commit `e79afb0` 声称已修复，但本次测试 §2.1.6 输出为空，无法验证是否真的修了 —— 因为输出被 welcome 吞了。等 welcome 修好后重新验证。）

## 格式问题：所有输出未对齐 ⭐

（之前已记录，commit `e79afb0` 声称已修复 status/bucket 对齐。本次测试 §16 status 仍显示未对齐的空格缩进，§2.2.1 bucket list 看起来对齐了。需等 welcome 修好后重新评估。）
