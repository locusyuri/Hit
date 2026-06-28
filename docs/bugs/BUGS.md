# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 第六轮自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。

---

## 重装/升级/doctor--fix 时 Junction 创建失败 os error 183 ⭐⭐⭐⭐⭐

> **极为严重** —— 已安装软件重装（`--force`）时，因旧 `current` junction 未删除，创建新 junction 时报 error 183。第六轮实测仍复现。开发 Agent 已尝试修复两次（`feb7c45` + `remove_dir_all→remove_dir`），均未生效。

### 现象

```
$ hit install curl（首次）→ 成功 ✅
$ hit install curl --force（重装）→ 回滚
WARN 事务回滚 app=curl
错误: IO 错误：创建 Junction: ...\curl\current -> ...\curl\8.21.0_1：
Cannot create a file when that file already exists. (os error 183)
```

### 根因推测

`remove_dir` 对 junction 可能静默失败（junction 是 reparse point，`remove_dir` 可能不处理符号链接目录）。建议直接用 `junction::delete()` 删除旧 junction，再 `junction::create()` 创建新的。

### 修复方向

创建 junction 前先调用 `junction::delete()` 或 `std::fs::remove_dir()` + 检查返回值。三种路径统一修：`install --force`、`update`、`doctor --fix`。

### 证据

REPORT.md §5.4（curl --force 回滚）

---

## 已修复（迁移至 SOLVED_BUGS）✅

以下 bug 在本轮实测中已验证修复（确认清单，共 11 项）：

- 搜索含描述匹配
- clap 错误被吞
- install 已装检测
- install 不存在的包报错
- 静默家族（which/prefix/home/uninstall/reset/bucket rm 不存在时均报错）
- bucket add 已存在 / remove 不存在 报错
- config set 校验（maybe/abc/unknown_key 报错）
- manifest 兼容性 0 WARN
- post_install 用 pwsh 执行（不再是 cmd.exe）
- post_install Scoop 变量定义（$bucketsdir/$dir/$version 等）
- info/hold/unhold/status/cache/list/prefix 全部正常
