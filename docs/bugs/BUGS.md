# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 实测验证（基于最新 binary，Round 4 manifest 路径修复后重装）。

---

## `hit install` 解压/同步阶段事务回滚 ⭐⭐⭐⭐⭐

> **极为严重** —— install 核心流程已部分修复（解析✔下载✔校验✔解压✔），但部分软件在解压/同步阶段事务回滚，install 失败。

### 现象

```
$ hit install curl   → 完整成功（解析✔下载✔校验✔解压✔同步✔提交✔），curl 8.21.0_1 装上 ✅
$ hit i jq           → 解析✔下载✔校验✔，解压时 "WARN 事务回滚 app=jq"，jq 未装上 ❌
$ hit install git    → 类似残留（doctor 检测出 git 未跟踪应用目录）
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

## Manifest 兼容性仍有 6 条遗漏 ⭐⭐

> 实测验证：仍 6 条 WARN，与之前相同。

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
