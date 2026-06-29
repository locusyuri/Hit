# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

`hit si` 被错误映射到 `i`（install），直接安装第一个搜索结果而非启动交互式 TUI。锁定期间不测不修。

---

## 全量回归测试通过（16 项迁入 SOLVED_BUGS）✅

详情见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。仅剩 `hit si` [已锁定]。
