# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 实测验证（基于最新 binary，Bug 2 修复后重装）。

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
