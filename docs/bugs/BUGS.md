# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## 首次启动引导功能彻底不可用 ⭐⭐⭐⭐⭐

> **严重** —— 首次运行 `hit` 时不再显示欢迎引导界面，用户无法通过引导快速添加官方 bucket。

### 现象

```
$ hit  # 首次安装后运行
# 预期：显示欢迎横幅 + 三选项菜单（1 快速开始 / 2 自定义 / 3 跳过）
# 实际：直接显示帮助信息，或执行命令无引导
```

### 根因

安装脚本 [install-hit.ps1](file:///C:/Repos/Hit/scripts/install-hit.ps1#L270-L285) 在安装时预先创建了 `config.json` 文件，而 `is_first_run()` 的判定条件是"config.json 不存在 **且** buckets 目录为空"。由于 config.json 总是存在，条件永远不满足，引导永远不会触发。

### 修复

修改 [welcome.rs](file:///C:/Repos/Hit/crates/hit-cli/src/welcome.rs#L21-L30) 中的 `is_first_run()`，移除 config.json 存在性检查，改为仅检查 buckets 目录是否为空。

### 证据

用户报告：`docs/plan/PROJECT.md#L89-98` 描述的首次启动引导功能不可用。

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。

---

## 已修复（第十二轮确认）✅

| Bug | 结果 |
|-----|------|
| 升级时 Junction 创建失败 os error 183 | ✅ `hit update --force` 成功升级 |
| `hit search/info` 指定 bucket 时返回未找到 | ✅ `hit search git --bucket main` 返回正确结果 |
| Bucket 更新时出现"无法打开 git 仓库"错误 | ✅ `hit bucket update` 正确处理无效 git 仓库 |
| `hit cleanup --cache` 输出为空 | ✅ 空缓存时输出"没有缓存文件需要清理" |
