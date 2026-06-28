# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
> 本批 bug 来自 2026-06-28 第五轮自动批量实测（见 [REPORT.md](./REPORT.md) + [REPORT_warn.log](./REPORT_warn.log)），由 `scripts/run-tests.ps1` 执行。

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

### 现象

```
$ hit si 7
安装 7zip ...
```

`si` 命令在无终端环境（如测试脚本中）TUI 初始化失败后回退安装第一个结果，完全背离设计目标。

### 根因

`tui::run_app()` 在非交互环境/无 TTY 时无法初始化，但未提示用户"请使用 `hit search` 或 `hit install`"，而是静默安装第一个匹配结果。

### 修复方向

需重新设计 TUI 的 fallback 行为：检测到无可用终端时提示用户改用标准命令，而非静默走安装流程。

### 证据

REPORT.md §15.x
