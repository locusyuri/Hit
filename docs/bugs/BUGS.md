# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

`hit si` 被错误映射到 `i`（install），直接安装第一个搜索结果而非启动交互式 TUI。锁定期间不测不修。

---

## 全量回归测试通过（11 项已修，迁入 SOLVED_BUGS）✅

第六/七轮实测验证，以下 bug 全部修复：

- 搜索含描述匹配
- clap 错误被吞
- install 已装检测 / 不存在的包报错
- 静默家族（which/prefix/home/uninstall/reset/bucket rm 不存在时均报错）
- bucket add 已存在 / remove 不存在 报错
- config set 校验（maybe/abc/unknown_key 报错）
- manifest 兼容性 0 WARN
- post_install 改用 pwsh 执行
- post_install Scoop 变量定义（$bucketsdir/$dir/$version 等）
- Junction 创建失败 os error 183（重装/升级/doctor--fix 三种场景）
- info/hold/unhold/status/cache/list/prefix 全部正常

详情见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
