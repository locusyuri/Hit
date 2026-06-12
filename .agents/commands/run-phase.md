---
name: run-phase
description: 显示 TODO.md 中指定 Phase 的未完成任务列表
usage: /run-phase <phase-number>
---

从 `docs/TODO.md` 中提取指定 Phase（1-5）的任务状态。

## 用法

```
/run-phase 1     # 显示 Phase 1 任务
/run-phase 2     # 显示 Phase 2 任务
```

## 输出格式

对于每个任务显示：

- [ ] 任务名 — 未开始
- [x] 任务名 — 已完成
- 🔄 任务名 — 进行中
