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

## 卸载未完全清理目录，导致重装时报"已安装" ⭐⭐⭐⭐⭐

> **极为严重** —— `hit rm 7zip` 提示"✔ 7zip 已卸载"但实际 app 目录残留，后续 `hit i 7zip` 报"错误: '7zip' 已安装"。

### 现象

```
$ hit rm 7zip
✔ 7zip 已卸载          ← 认为已卸载

$ hit i 7zip
错误: '7zip' 已安装      ← 但实际还认为安装着（目录没删干净）

$ hit i 7zip            ← 再执行一次才真正安装
安装 7zip ... → 成功
```

### 根因

`hit rm` 卸载时清除了 junction、删除了 shim，但 app 目录（`apps/7zip/26.02/`）未删除。"已安装"检测依赖特定标记（可能是 app 目录存在或 db.json 记录），目录残留导致状态误判。

### 修复方向

1. `hit rm` 卸载时应彻底删除 `apps/<app>/<version>/` 整个目录
2. "已安装"检测应使用可靠的原子标记（如 db.json 记录），而非依赖目录是否存在
3. 同一命令第二次执行时应正确安装（绕过残留目录的干扰）

### 证据

用户实测：rm→已安装→重试后装上的完整日志。

---

## post_install 脚本中 `$false` 被当作命令而非 PowerShell 变量 ⭐⭐⭐

### 现象

7zip 安装后 post_install 执行时 stderr 输出：

```
false:
Line |   4 |  if (false) { $content = $content -replace 'HKEY_CURRENT_USER...
     |      ~~~~~
     | The term 'false' is not recognized as a name of a cmdlet
```

`$false` 被展开成字面量 `false` 而非 PowerShell 内置变量 `$false`。

### 根因

`controller.rs` 的 preamble 有 `$global=$false`。当 preamble 拼接为字符串传递给 `pwsh -NoProfile -Command` 时，`$false` 可能被字符串插值展开成字面量 `false`，导致后续脚本中出现的 `$false` 也变成裸 `false`。

具体来说：如果 preamble 是用 `format!` 或 `write!` 拼接字符串（Rust 侧），`$false` 在 Rust 字符串里就是字面 `$false`。但传到 pwsh 后，如果 preamble 本身被 `-Command` 参数解析时 `$false` 被 pwsh 自动求值了，那变量定义就变成了 `$global = false`（赋值后 `$false` 变量被定义了），但后续脚本中的 `(false)` 又变成了裸词。

另一个可能：preamble 里 `$global = $false` 赋值正确，但后续脚本中的 `$false` 不在同一个变量作用域内（可能是脚本有自己的作用域）。

### 修复方向

1. 确保 preamble 中 `$global=$false` 在 pwsh 中正确求值，且 `$false` 变量在脚本作用域内可用
2. 或改用 `$global=0`/`$global=$true` 等避开 `$false` 解析问题
3. 参考 Scoop 原版如何定义 `$global` 变量

### 证据

用户实测 `hit install 7zip` 的完整 stderr 输出。

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

---

## 已修复（第八轮确认）✅

---

## 已修复（第八轮确认）✅

| Bug | 验证 |
|-----|------|
| 卸载 junction os error 4390 | ✅ `hit rm curl` → `✔ curl 已卸载` |
| post_install 漏 `$bucket` 变量 | ✅ `hit install 7zip` → `✔ 7zip 26.02 安装完成` |
| 搜索结果偶发不一致 | ✅ 本轮未复现，`hit s git`×3 稳定 44 结果 |

详情见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
