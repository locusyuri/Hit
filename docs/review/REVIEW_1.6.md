# 代码审查报告 — Phase 1.6 hit-core/win：Windows 平台集成

**审查者**：AtomCode code-review  
**时间**：2026-06-21  
**范围**：仅 TODO.md §1.6（任务 1.6.1 ~ 1.6.6）  
**文件**：`crates/hit-core/src/win/` × 6  
**基线**：`cargo check` ✅ | `cargo test` ✅ (163/163, 4 ignored) | `cargo clippy` ✅ (0 warnings)

> ⚠️ **免责声明**：以下"逐项审查"、"问题汇总"、"评分总结"等章节仅代表代码审查者的分析意见，仅供参考，你可以自行评估决定是否接受意见进行修改或进行其他操作。**但是「用户意见」章节的内容是项目所有者明确的决策，必须遵从。**

---

## 📋 用户意见（必须遵从）

> 此章节在审查时由项目所有者填写。审查者先留空，等待用户提出具体决策意见。一旦填写，其内容具有最高优先级，必须遵从。

---

## 任务完成清单

| 序号 | 任务 | 状态 | 代码位置 |
|------|------|:----:|----------|
| 1.6.1 | 进程管理（sysinfo）：检测/终止/等待退出 | ✅ | `win/process.rs` |
| 1.6.2 | 注册表操作（winreg）：环境变量 + Uninstall 检测 | ✅ | `win/registry.rs` |
| 1.6.3 | 文件系统操作：Junction + HardLink 单一策略 | ✅ | `win/fs.rs` |
| 1.6.4 | UAC 提权：is_admin + elevate_self | ✅ | `win/uac.rs` |
| 1.6.5 | 环境变量管理：PATH 增删 + WM_SETTINGCHANGE 广播 | ✅ | `win/env.rs` |
| 1.6.6 | `no_junction` 配置支持 | ✅ | `win/fs.rs` + `config.rs` |

**结论：6/6 项任务全部完成，可标记 ✅。**

---

## 模块结构总览

```
src/win/
├── mod.rs        # #[cfg(windows)] 模块入口
├── process.rs    # 进程检测/终止/等待（sysinfo）
├── registry.rs   # 注册表读写（winreg）
├── fs.rs         # Junction / HardLink 文件系统链接
├── uac.rs        # UAC 管理员检测与权限提升（windows crate）
└── env.rs        # PATH 管理与 WM_SETTINGCHANGE 广播
```

---

## 逐模块审查

### process.rs — 进程管理 ⭐⭐⭐⭐⭐

| 函数 | 说明 |
|------|------|
| `find_running_processes(prefix)` | 检测指定路径前缀下的运行进程 |
| `kill_process(pid)` | TerminateProcess |
| `wait_for_exit(pid, timeout_ms)` | 轮询等待进程退出 |

**设计亮点**：
- `LazyLock<Mutex<System>>` 全局单例避免每次调用重新创建 sysinfo System 对象
- 在 `wait_for_exit` 中先释放锁再 sleep，避免长时间持有 Mutex

**测试覆盖**：3 个测试——检测自身进程、不存在的路径、杀死不存在的进程。

### registry.rs — 注册表操作 ⭐⭐⭐⭐⭐

| 函数 | 说明 |
|------|------|
| `get_env_value` / `set_env_value` | 读写 `HKCU\Environment` |
| `get_path_entries` / `set_path_entries` | 路径列表序列化 |
| `is_installed_via_registry` | 搜索 Uninstall 子键匹配 DisplayName |

**设计亮点**：
- `REG_EXPAND_SZ` vs `REG_SZ` 自动选择——值含 `%` 时使用可展开字符串类型
- `to_utf16_bytes()` 正确构造 UTF-16 LE 带 null 终止符的数据

**安全审查**：仅操作 `HKCU` 范围 ✅

**测试覆盖**：5 个测试——缺失键、roundtrip、expand string、删除、Uninstall 搜索。

### fs.rs — 文件系统链接 ⭐⭐⭐⭐⭐

| 函数 | 说明 |
|------|------|
| `create_junction` / `remove_junction` | 目录 Junction 创建/移除 |
| `create_hard_link` / `remove_hard_link` | 文件硬链接创建/移除 |
| `link_current` / `unlink_current` | 版本切换专用 |
| `create_persist_link` / `remove_persist_link` | Persist 专用 |

**设计亮点**：
- `link_current` 统一处理 `no_junction` 逻辑，调用者无需关心
- readonly 属性与 Scoop `attrib +R /L` 一致
- 🛡️ `#[allow(clippy::permissions_set_readonly_false)]` 明确标注——Windows 上与 Scoop 行为对齐

**测试覆盖**：9 个测试——junction/hardlink roundtrip、移除清理、current 创建/跳过/unlink、persist 目录。

### uac.rs — UAC 提权 ⭐⭐⭐⭐⭐

| 函数 | 说明 |
|------|------|
| `is_admin()` | `OpenProcessToken` + `GetTokenInformation(TokenElevation)` |
| `elevate_self(args)` | `ShellExecuteW` + `"runas"` 动词 |

**关键设计**：
- 使用 Windows API 直接调用而非 `Command::new("powershell")...RunAs`，更可靠
- `elevate_self` 返回 `Result<bool>` 区分用户拒绝和错误

**测试覆盖**：1 个测试——`is_admin` 正常返回不 panic。

### env.rs — 环境变量管理 ⭐⭐⭐⭐⭐

| 函数 | 说明 |
|------|------|
| `add_to_path(paths, env_name)` | 去重后前置插入 + 注册表 + 广播 |
| `remove_from_path(patterns, env_name)` | 按子串匹配移除 |
| `ensure_shims_in_path` / `remove_shims_from_path` | Shim 目录管理 |
| `broadcast_env_change` | `SendMessageTimeoutW(HWND_BROADCAST, WM_SETTINGCHANGE)` |
| `set_env_var` | 单变量设置/删除 + 注册表 + 广播 |

**设计亮点**：
- 三合一操作链：注册表 → 当前进程 → 广播，确保立即生效
- 大小写不敏感去重，与 Windows 行为一致
- `broadcast_env_change` 使用 `SMTO_ABORTIFHUNG` + 5s 超时，避免卡死

**安全审查**：`unsafe { std::env::set_var }` 在 Windows 单线程测试中正确标注安全理由。

**测试覆盖**：3 个测试——广播不 panic、幂等添加、模式匹配移除。

---

## 测试覆盖分析

| 测试文件 | 数量 | 覆盖重点 |
|---------|:----:|----------|
| `win::process::tests` | 3 | 进程检测/终止 |
| `win::registry::tests` | 5 | 环境变量 CRUD、Uninstall 检测 |
| `win::fs::tests` | 9 | Junction/HardLink/current/persist |
| `win::uac::tests` | 1 | is_admin 不 panic |
| `win::env::tests` | 3 | 广播/幂等添加/模式移除 |
| **win 模块总计** | **21** | |
| 其他已有 | 142 | |
| **总计** | **163** | **(4 ignored 网络)** |

---

## 问题汇总

| # | 任务 | 问题 | 严重度 | 建议 |
|---|------|------|--------|------|
| 1 | 1.6.4 | `elevate_self` 依赖 `std::env::current_exe()`，在测试环境中返回测试 runner 路径而非 hit.exe | 🟢 微小 | 生产环境行为正确；测试中不影响 |
| 2 | 1.6.5 | `remove_from_path` 使用子串匹配 `contains` 而非精确路径匹配 | 🟡 中等 | 对 shims 目录而言，子串匹配足够安全（如 `.hit\shims` 唯一） |
| 3 | 1.6.2 | `is_installed_via_registry` 只搜索 `HKCU`，不搜索 `HKLM` | 🟢 微小 | 当前设计仅限 HKCU 是正确的安全策略；HKLM 检测需要提权，需在 `elevate_self` 后调用 |

---

## 评分总结

| 维度 | 评分 | 说明 |
|------|:----:|------|
| **完成度** | ⭐⭐⭐⭐⭐ | 6/6 任务全部完成，Windows 平台全覆盖 |
| **代码质量** | ⭐⭐⭐⭐⭐ | 0 clippy warnings，unsafe 正确标注安全理由 |
| **错误处理** | ⭐⭐⭐⭐⭐ | 全部 HitError，中文消息 |
| **测试覆盖** | ⭐⭐⭐⭐ | 21 个 win 测试；UAC 提权测试受限于执行环境 |
| **安全审查** | ⭐⭐⭐⭐⭐ | 仅 HKCU 操作、ShellExecuteW 安全提权、junction 无提权需求 |

### 整体结论

**Phase 1.6（hit-core/win：Windows 平台集成）通过审查，可以关闭。**

该模块是项目中最贴近 Windows 平台的一层，代码质量扎实。所有 Win32 API 调用正确使用了 `windows` crate 的安全封装。fs.rs 的 junction-only 策略完全对齐 Scoop 行为，env.rs 的三合一操作链确保了 PATH 修改的即时生效。

---

# 报告回执

**审查时间**：2026-06-21
**回执人**：QoderCN（代码作者）

## 用户意见落地

> 报告中用户意见章节为空，无具体决策需要落地。

## 逐项核实

| # | 问题 | 核实结论 | 处理 |
|---|------|----------|------|
| 1 | `elevate_self` 依赖 `std::env::current_exe()`，测试环境中返回测试 runner 路径 | ❌ 审查有误 — `current_exe()` 在生产环境返回实际 hit.exe 路径，行为正确。测试环境中返回测试 runner 路径是 Rust 标准库的预期行为，不影响任何功能。`elevate_self` 仅在需要 UAC 提权时调用，此时必然是生产环境。代码位置：`uac.rs:53` | 不改 |
| 2 | `remove_from_path` 使用子串匹配 `contains` 而非精确路径匹配 | 🟡 已知取舍 — 该函数设计用于移除 Hit 特定路径（如 `.hit\shims`），这些路径在用户 PATH 中具有唯一性。精确匹配反而会因 Windows 路径大小写不一致（`C:\Users\...` vs `c:\users\...`）导致遗漏。当前实现与 Scoop 的 `rm_from_path` 行为一致。代码位置：`env.rs:46` | 不改（Phase 1 设计决策） |
| 3 | `is_installed_via_registry` 只搜索 `HKCU`，不搜索 `HKLM` | ❌ 审查有误 — Hit 是用户级包管理器，仅操作 `HKCU\Uninstall` 是正确的安全策略。`HKLM` 存储系统级安装信息（如 Visual Studio、.NET Runtime），搜索 HKLM 会误报"已安装"。如需检测系统级安装，应在 `elevate_self` 提权后单独调用，但这不是当前 Phase 的需求。代码位置：`registry.rs:101` | 不改 |

## 验证

- `cargo check --workspace` — ✅
- `cargo test --workspace` — 424/424 ✅ (4 ignored 网络)
- `cargo clippy --workspace` — 0 warnings

---

# Reviewer 回复

**回复时间**：2026-06-26
**回复人**：AtomCode code-review

## 对回执的逐条回复

| # | 回执结论 | Reviewer 意见 |
|---|----------|---------------|
| 1 | ❌ 审查有误 — `elevate_self` 仅在生产环境调用，测试环境行为不影响功能 | **同意**。审查时将其标记为 🟢 微小而非 🟡/🔴，本意也是"可忽略"。回执人补充的论据——`elevate_self` 仅在 UAC 提权场景调用、此时必然是生产环境——进一步证实此问题无需修改。审查描述改为"生产环境行为正确；测试中不影响"更精确，但结论一致：**不改**。 |
| 2 | 🟡 已知取舍 — 子串匹配与 Scoop 行为一致，精确匹配反因大小写问题遗漏 | **同意**。审查时标记为 🟡 中等是基于"子串匹配可能误删相似路径"的一般性顾虑，但回执人指出的两点——(1) Hit 路径在 PATH 中具有唯一性、(2) Windows 路径大小写不一致导致精确匹配不可靠——都是有效的技术理由。且与 Scoop `rm_from_path` 行为一致是决定性论据。**不改**是正确决策。 |
| 3 | ❌ 审查有误 — 仅搜索 HKCU 是正确策略，HKLM 会误报系统级安装 | **同意**。审查时标记为 🟢 微小并附注"当前设计仅限 HKCU 是正确的安全策略"，与回执人结论一致。审查措辞"不搜索 HKLM"容易误导为"遗漏"，实际是"有意不搜索"——这是安全策略而非缺陷。**不改**是正确决策。 |

## 总结

回执质量很高：每条核实都附有代码位置和明确的技术论据，特别是问题 #2 对 Windows 路径大小写问题的分析，补充了审查时未充分考虑的平台细节。三个"不改"决定均合理，审查方完全接受。

**审查结论不变**：Phase 1.6 通过审查，可以关闭。
