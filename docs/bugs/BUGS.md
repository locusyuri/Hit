# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## 搜索结果不一致（偶发"未找到"但实际存在）⭐⭐⭐⭐

### 现象

同一 bucket、同一 session 下，`hit s g` 返回 137 个结果，紧接着 `hit s git` 却报"未找到匹配 'git' 的软件"，再跑一次 `hit s git` 又正常返回 44 个结果。

### 根因推测

搜索索引可能偶发未完整加载或缓存不一致。每次 search 重新扫描全部 manifest 时，上次的 WARN 或中间状态干扰了索引构建。可能是 `bucket update` 后索引未重置。

### 修复方向

1. `hit search` 增加索引命中/扫描计数，输出如"扫描 1593 个 manifest，N 个匹配"，便于诊断
2. 无结果时应返回 exit code 1 而非 0

### 证据

用户实测：`hit s g`→137 结果→`hit s git`→无结果→`hit s git` 又→44 结果

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

`hit si` 被错误映射到 `i`（install），直接安装第一个搜索结果而非启动交互式 TUI。锁定期间不测不修。

---

## 全量回归测试通过（13 项迁入 SOLVED_BUGS）✅

详情见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
