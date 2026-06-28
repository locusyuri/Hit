# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## 卸载时 junction 移除失败 os error 4390 ⭐⭐⭐⭐⭐

> **极为严重** —— `hit rm curl` 卸载时报 `The file or directory is not a reparse point. (os error 4390)`，旧 junction 无法删除。原因是前一轮 junction 修复（`remove_dir` 不跟随 reparse point）导致 `current` 目录实际是**普通目录**而非 junction，卸载时按 junction 删除失败。

### 现象

```
$ hit rm curl
卸载 curl ...
错误: IO 错误：移除 Junction: ...\hit\apps\curl\current：
The file or directory is not a reparse point. (os error 4390)
```

### 根因

第七轮实测确认 junction 创建不再报 183，但那是用 `remove_dir` 兜底时创建了**普通目录**而非真正的 junction。卸载时 `junction::delete` 发现不是 reparse point 就报 4390。`install --force` 的"修复"治标不治本——创建 junction 前要先 `junction::delete`（非 `remove_dir`），而不是 fallback 到普通目录。

### 修复方向

1. `create_junction()` 创建前先 `junction::delete()` 删除旧 junction（而非 `remove_dir`），确保旧链接被正确清除
2. 如 `junction::delete` 失败（不是 junction），再用 `fs::remove_dir` 删除普通目录
3. 确保三种路径一致：`install --force`、`update`、`uninstall`

### 证据

用户实测 `hit rm curl` 输出。

---

## 搜索结果不一致（偶发"未找到"但实际存在）⭐⭐⭐⭐

### 现象

同一 bucket、同一 session 下，`hit s g` 返回 137 个结果，紧接着 `hit s git` 却报"未找到匹配 'git' 的软件"，再跑一次 `hit s git` 又正常返回 44 个结果。`hit s z` / `hit s zo` 也报"未找到"但实际存在 zig、zoom 等包。

### 根因推测

搜索索引可能偶发未完整加载或缓存不一致。每次 search 重新扫描全部 manifest 时，上次的 WARN 或中间状态干扰了索引构建。也可能是 `bucket update` 后索引未重置。

### 修复方向

1. `hit search` 增加索引命中/扫描计数，输出如"扫描 1593 个 manifest，N 个匹配"，便于诊断
2. 确认重复 search 是否触发重新扫描增量索引
3. 无结果时应返回 exit code 1 而非 0

### 证据

用户实测：`hit s g`→137 结果→`hit s git`→无结果→`hit s git` 又→44 结果

---

## post_install 变量 `$bucket` 未定义 ⭐⭐⭐⭐⭐

> **极为严重** —— `$bucketsdir` 已定义（`C:\...\buckets`），但 `$bucket` 变量忘记定义，导致 `$bucketsdir\$bucket\scripts\7-zip` 展开成 `C:\...\buckets\scripts\7-zip`（缺少 bucket 名称段），7zip 安装失败。git 安装也会同样失败。

### 现象

```
$ hit install 7zip
▶ [提交] 7zip... ✔
Get-ChildItem -Path "$bucketsdir\$bucket\scripts\7-zip" -Filter '*.reg'
     | Cannot find path 'C:\Users\Violet\Downloads\test\hit\buckets\scripts\7-zip'
     |                                                      ^^^^^^^^
     | $bucket 展开为空，buckets 后面直接 scripts
WARN 事务回滚 app=7zip
错误: 安装 '7zip' 失败：PostInstall 脚本退出码：1
```

`$bucketsdir = C:\Users\Violet\Downloads\test\hit\buckets`（正确）
`$bucket = ''`（空了！忘了定义）
`$bucketsdir\$bucket\scripts\7-zip` → `C:\...\buckets\\scripts\7-zip` → 路径错误

### 根因

`controller.rs` 的 preamble 定义了 `$dir`、`$version`、`$persist_dir`、`$bucketsdir`、`$scoopdir`、`$app`、`$global`，但**漏了 `$bucket`**。Scoop 的 `$bucket` 是 bucket 名称（如 main、extras），用于拼 `$bucketsdir\$bucket\scripts\<app>` 路径找安装脚本。

### 修复方向

premable 追加 `$bucket='<bucket_name>'`，从 manifest 来源的 bucket 名称取值。

### 证据

用户实测 `hit install 7zip` 输出。

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。
> 设计完成后由产品/设计 agent 解除锁定。

`hit si` 被错误映射到 `i`（install），直接安装第一个搜索结果而非启动交互式 TUI。锁定期间不测不修。

---

## 全量回归测试通过（11 项迁入 SOLVED_BUGS）✅

详情见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。
