# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 第六轮自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。

---

## 重装/升级/doctor--fix 时 Junction 创建失败 os error 183 ⭐⭐⭐⭐⭐

> **极为严重** —— 已安装软件重装（`--force`）时，因旧 `current` junction 未删除，创建新 junction 时报 error 183。第六轮实测仍复现。开发 Agent 已尝试修复三次（`feb7c45` / `remove_dir` / `cmd.rmdir`），前两次未生效。

### 现象

```
$ hit install curl（首次）→ 成功 ✅
$ hit install curl --force（重装）→ 回滚
WARN 事务回滚 app=curl
错误: IO 错误：创建 Junction: ...\curl\current -> ...\curl\8.21.0_1：
Cannot create a file when that file already exists. (os error 183)
```

### 根因（第三次修复分析）

`create_junction` 删除旧 junction 的流程中，`remove_readonly()` 使用 Rust 的 `fs::metadata()` 获取 junction 属性——但 `fs::metadata` 在 junction 上会**跟随到目标目录**（`apps/curl/8.21.0_1/`），不会操作 `current` junction 点自身。因此 junction 点的 readonly 属性未被清除，`junction::delete` 静默失败，最终 `junction::create` 报 183。

**三个修复的演进**：
| 修复 | 方案 | 失效原因 |
|------|------|----------|
| `feb7c45` | `junction::delete.ok()` → 报错并 `remove_dir_all` fallback | `remove_dir_all` 跟随 junction 删了目标目录 |
| `f75bd6b` | `remove_dir_all` → `remove_dir` | `remove_dir` 对 readonly junction 静默失败 |
| `eb4e657` | **当前**：`cmd.exe /c attrib -R` 清除 readonly + `cmd.exe /c rmdir` 删除 junction | 待验证 |

### 修复方向（第三次，commit `eb4e657`）

创建 junction 前改用 Windows 原生命令（`attrib -R` / `rmdir`），避免 Rust 标准库的 `fs::metadata` 跟随 junction 到目标目录的问题。三级 fallback：
1. `junction::delete()`
2. `cmd.exe /c rmdir`（Windows 原生，正确删除 reparse point）
3. `fs::remove_dir()`（兜底）

### 证据

REPORT.md §5.4（curl --force 回滚）

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

`hit si` 被错误映射到 `i`（install），直接安装第一个搜索结果而非启动交互式 TUI。锁定期间不测不修。

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
