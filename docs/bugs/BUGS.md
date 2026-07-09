# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## 重装/升级时 Junction 创建失败 os error 183 ⭐⭐⭐⭐⭐

> **极为严重** —— `hit install curl --force` 和 `hit update --force`（7zip）重装/升级时旧 junction 无法删除。开发 Agent 已修四次，仍未生效。

### 现象

```
$ hit install curl --force
WARN 事务回滚 app=curl
错误: IO 错误：创建 Junction: ...\current -> ...\8.21.0_2：
Cannot create a file when that file already exists. (os error 183)

$ hit update --force
升级 7zip → 26.02
WARN 事务回滚 app=7zip
✘ 升级失败: 同上 junction os error 183
```

### 注意

- ✅ `hit i 7zip`（首次安装）→ 成功
- ✅ `hit rm 7zip`（卸载）→ `✔ 7zip 已卸载`
- ✅ `hit i 7zip`（**卸载后**重装）→ 成功（clean install）
- ❌ `hit install curl --force`（**不卸载**直接 `--force` 重装）→ 报 os error 183
- ❌ `hit update --force` 中 7zip 升级 → 报 os error 183（curl/git 升级成功）

**关键发现**：卸载→重装（clean install）正常工作，但 `--force`（覆盖式重装）的 junction 重写路径有问题。`--force` 的重装没有走"先卸载再安装"的完整流程，而是尝试在原地覆盖 junction，旧 junction 删除失败就报 183。建议 `--force` 直接走卸载→安装的完整路径。

### 证据

第九轮实测 §5.4（curl --force 回滚）、§8.5（7zip 升级回滚）

---

## post_install 脚本中 `$false` 被当作命令而非 PowerShell 变量 ⭐⭐⭐

### 现象

git 安装后 post_install stderr：

```
C:\...\git\2.54.0: The term '...' is not recognized as a name of a cmdlet
```

7zip 同样报 `false: The term 'false' is not recognized as a cmdlet`。但注意：**git 和 7zip 安装成功了**（`✔ git 2.54.0 安装完成（8）`），post_install 的部分错误未导致回滚。

### 证据

第九轮实测 §5.5（git 安装成功但有 stderr）

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。

---

## 搜索输出排版问题：描述字段导致错位 ⭐⭐ [设计]

### 现象

`hit search git` 输出表格中"描述"列内容过长（如 `A free and open source distributed version control system.`），表格列宽被撑开，名称和版本列无法对齐。单行显示不下时视觉效果混乱。

### 建议

1. 搜索结果仅保留"名称"和"版本"两列，去掉"描述"列（大部分场景用户不需要在看搜索结果时读描述）
2. 或描述列固定最大宽度（如 40 字符），超出截断加 `…`
3. 或通过参数控制是否显示描述（如 `hit search git -d` 含描述，默认不含）
4. 参考 Scoop 原版：`scoop search` 默认只输出名称

---

## 引入色彩系统美化输出 ⭐⭐ [设计]

### 现象

当前所有命令输出为纯黑白文本，没有色彩区分。例如：
- `✔ bucket 'main' 添加完成` — 成功提示无绿色
- `错误: ...` — 错误提示无红色（stderr 可能有颜色，但 stdout 全黑白）
- 表格无表头高亮
- `[解析]`/`[下载]`/`[校验]` 等 install 进度无色彩区分

### 建议

1. 引入 `colored` crate（项目 Cargo.toml 中已有依赖 `colored = "2"`）实现跨平台色彩
2. 错误信息用红色、成功用绿色、警告用黄色、进度步骤用青色
3. 表格表头加粗或高亮
4. 保持与 Scoop 原版 PowerShell 的色彩风格一致（绿色 ✔ / 红色 ✘）

---

## 已修复（第九轮确认）✅

| Bug | 结果 |
|-----|------|
| 卸载不干净导致重装误判 | ✅ `hit install curl` → `错误: 'curl' 已安装（db.json 有记录）` — 改用 db.json 判据 |
| `$bucket` 变量缺失 | ✅ `git 2.54.0 安装完成（8）` — post_install 正确执行 |
| 搜索结果一致性 | ✅ `hit s git` 稳定 40+ 结果 |
| 其余 11 项 | ✅ 全部保持正常 |
