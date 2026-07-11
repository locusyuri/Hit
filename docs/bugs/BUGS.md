# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---

## 升级时 Junction 创建失败 os error 183 ⭐⭐⭐⭐⭐

> **严重** —— `hit update --force` 在软件已是最新版本时，升级流程尝试原地覆盖 junction 导致失败。`hit install --force` 已修复，但 `hit update --force` 仍存在问题。

### 现象

```
$ hit update --force
升级 git → 2.55.0.2
WARN 事务回滚 app=git
✘ 升级失败: IO 错误：创建 Junction: ...\current -> ...\2.55.0.2：
Cannot create a file when that file already exists. (os error 183)
```

### 注意

- ✅ `hit install curl --force`（**不卸载**直接 `--force` 重装）→ **已修复**，走卸载→安装完整路径
- ❌ `hit update --force` 中软件已是最新版本时升级 → 报 os error 183

**关键发现**：`hit install --force` 已改为走"先卸载再安装"的完整路径，junction 问题已修复。但 `hit update --force` 在软件已是最新版本时，仍尝试原地覆盖 junction，旧 junction 删除失败就报 183。建议 upgrade 流程中检测版本相同情况时跳过重装，或也走卸载→安装完整路径。

### 证据

第九轮实测 §5.4、§8.5；第十一轮实测 §8.5（update --force）

---

## `hit search/info` 指定 bucket 时返回未找到 ⭐⭐⭐

### 现象

`hit search git --bucket main` 和 `hit info git` 返回"未找到"，但 main bucket 已添加且包含 git 软件。

```
$ hit search git --bucket main
未找到匹配 'git' 的软件

$ hit info git
错误: 未找到软件 'git'

$ hit info git --bucket main
# 正常返回 git 详情
```

### 证据

第十一轮实测 §3.4、§4.1

---

## Bucket 更新时出现"无法打开 git 仓库"错误 ⭐⭐

### 现象

多次执行 `hit update` 后，部分 bucket 突然出现"does not appear to be a git repository"错误，需要重新克隆才能恢复。

```
$ hit update curl
✘ extras 失败: Bucket 'extras' 错误：无法打开 git 仓库："C:\...\buckets\extras" does not appear to be a git repository
```

### 证据

第十一轮实测 §8.3、§8.4

---

## `hit cleanup --cache` 输出为空 ⭐

### 现象

`hit cleanup --cache` 命令执行后没有任何输出反馈，用户无法判断是否执行成功。

### 证据

第十轮实测 §11.3；第十一轮实测 §11.3

---

## `hit si` 变成直接安装第一个软件 ⭐⭐⭐⭐⭐ [已锁定]

> **锁定说明**：此功能需重新设计交互方案，暂不修复。锁定时间：2026-06-28。

---

## 已修复（第十一轮确认）✅

| Bug | 结果 |
|-----|------|
| 卸载不干净导致重装误判 | ✅ `hit install curl` → `错误: 'curl' 已安装（db.json 有记录）` |
| `$bucket` 变量缺失 | ✅ `git 2.55.0.2 安装完成（8）` — post_install 正确执行 |
| 搜索结果一致性 | ✅ `hit s git` 稳定 39+ 结果 |
| `hit install --force` junction os error 183 | ✅ `hit install curl --force` 成功重装 |
| `hit doctor --fix` 无法修复损坏 junction | ✅ `hit doctor --fix` 成功修复 git 损坏的 current 链接 |
| 日志级别 `-v/-vv/-vvv` 输出相同 | ✅ `-v` INFO / `-vv` INFO+DEBUG / `-vvv` INFO+DEBUG+TRACE |
| `hit wrongcmd` 错误提示误导 | ✅ 不再显示误导性的"similar subcommand exists"提示 |
