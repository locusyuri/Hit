# EUREKA — 灵感速记

> 用于随手记录突发灵感与点子。每条只写**一句话核心想法**，详细的功能设计、方案论证、文档联动等后续再展开到对应文档（FEATURES.md / ROADMAP.md / TODO.md 等）。
>
> 格式建议：`- **关键词** — 一句话描述（可选：日期 / 触发场景）`

---

## 灵感池

- **i18n 国际化** — CLI 输出与错误信息按系统 locale 切换语言（中/英优先），manifest 描述字段也支持 `_zh` / `_en` 多语言 fallback。
- **代管其它包管理器** — hit list / upgrade / uninstall 等命令可列出并操作 bun、npm、pnpm、uv 等包管理器全局安装的包。通过子进程调用对应包管理器命令（如 `npm ls -g --json`）并解析 stdout 实现。由于各包管理器执行速度不确定，需使用异步/spawn 超时控制等方式防止拖慢 hit 自身响应。
