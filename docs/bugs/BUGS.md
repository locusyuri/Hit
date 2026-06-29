# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## 重装/升级时 Junction 创建失败 os error 183 ⭐⭐⭐⭐⭐

> **极为严重** —— `hit install curl --force` 重装时旧 junction 无法删除。开发 Agent 已修三次（`feb7c45`/`f75bd6b`/`eb4e657`），仍未生效。

### 现象

```
$ hit install curl          → 成功（首次安装 ✅）
$ hit install curl --force  → 回滚
WARN 事务回滚 app=curl
错误: IO 错误：创建 Junction: ...\current -> ...\8.21.0_2：
Cannot create a file when that file already exists. (os error 183)
```

### 已修的相关路径

- `hit rm curl`（卸载）→ ✅ `✔ curl 已卸载`（`e06270b` 修了）

### 根因推测

三次修复都尝试"创建前先删除旧 junction"但没成功：
1. `remove_dir_all` → 跟随 junction 误删后端目录
2. `remove_dir` → 不跟随但 junit 删除不彻底
3. `cmd /c rmdir` + `attrib -R` + `fs::remove_dir` 三级 fallback → 仍未正确删除

建议 debug 看哪级 fallback 执行了、为什么失败。或改用覆盖式创建（先 `junction::delete` 再 `junction::create`）。

### 证据

第八轮实测 `hit install curl --force` 输出。

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

---

## 已修复（第八轮确认）✅

| Bug | 验证 |
|-----|------|
| 卸载 junction os error 4390 | ✅ `hit rm curl` → `✔ curl 已卸载` |
| post_install 漏 `$bucket` 变量 | ✅ `hit install 7zip` → `✔ 7zip 26.02 安装完成` |
| 搜索结果偶发不一致 | ✅ 本轮未复现，`hit s git`×3 稳定 44 结果 |

详情见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
