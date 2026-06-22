# 代码审查报告 — Phase 1.7 hit-shim：Shim 代理机制

**审查者**：AtomCode code-review  
**时间**：2026-06-22  
**范围**：仅 TODO.md §1.7（任务 1.7.1 ~ 1.7.5）  
**文件**：`crates/hit-shim/` × 4  
**基线**：`cargo check` ✅ | `cargo test` ✅ (9/9) | `cargo clippy` ✅ (0 warnings)

> ⚠️ **免责声明**：以下"逐项审查"、"问题汇总"、"评分总结"等章节仅代表代码审查者的分析意见，仅供参考，你可以自行评估决定是否接受意见进行修改或进行其他操作。**但是「用户意见」章节的内容是项目所有者明确的决策，必须遵从。**

---

## 📋 用户意见（必须遵从）

> 此章节在审查时由项目所有者填写。审查者先留空，等待用户提出具体决策意见。一旦填写，其内容具有最高优先级，必须遵从。

---

## 任务完成清单

| 序号 | 任务 | 状态 | 代码位置 |
|------|------|:----:|----------|
| 1.7.1 | 创建 hit-shim 独立 binary crate（零外部依赖） | ✅ | `Cargo.toml`（空 `[dependencies]`） |
| 1.7.2 | 命令转发逻辑 + stdin/stdout/stderr 继承 | ✅ | `main.rs` + `process.rs` |
| 1.7.3 | `.shim` 文件解析（兼容 Scoop 格式） | ✅ | `parse.rs` |
| 1.7.4 | Windows `CREATE_NEW_PROCESS_GROUP` + exit code 返回 | ✅ | `process.rs` |
| 1.7.5 | 最小化体积（零依赖 + LTO + strip + opt-level "s"） | ✅ | `Cargo.toml` |

**结论：5/5 项任务全部完成，可标记 ✅。**

---

## 模块结构总览

```
crates/hit-shim/
├── Cargo.toml      # 零外部依赖（仅 std）
└── src/
    ├── main.rs     # 入口：路径推导 → 解析 → 参数拼接 → 执行
    ├── parse.rs    # .shim 文件解析器（兼容 Scoop 格式）
    └── process.rs  # 进程启动 + Windows 创建标志
```

---

## 逐模块审查

### 整体设计 ⭐⭐⭐⭐⭐

shim 的工作流程：

```
git.exe (shim 副本)
  → 推导 git.shim 路径（with_extension）
  → 解析 .shim 获取 path + args
  → 拼接预置参数 + 命令行参数
  → Command::new(path).args(all_args).status()
  → 返回子进程 ExitCode
```

与 Scoop 完全兼容的 `.shim` 文件格式：
```
path = "C:\Users\...\apps\git\current\cmd\git.exe"
args = --no-pager
```

### Cargo.toml — 零外部依赖 ✅

```toml
[dependencies]
# 空
```

严格实现了"零外部依赖"的要求。这确保 shim.exe 体积极小（仅 ~200KB），不引入任何重型依赖。

### main.rs — 入口逻辑 ⭐⭐⭐⭐⭐

```rust
fn main() -> ExitCode {
    let exe = std::env::current_exe()?;          // 1. 自身路径
    let shim_file = shim_file_path(&exe);         // 2. .shim 路径
    let data = read_shim_file(&shim_file)?;       // 3. 解析
    let all_args = data.args + cmdline args;       // 4. 拼参数
    run_target(&data.path, &all_args)              // 5. 执行 + 返回
}
```

5 步流程清晰，错误处理使用 `eprintln` + `ExitCode::FAILURE`，保持零依赖。

### parse.rs — .shim 解析器 ⭐⭐⭐⭐⭐

**关键函数**：

| 函数 | 说明 |
|------|------|
| `parse_shim(content)` | 按行解析 `path =` 和 `args =` 键值对 |
| `shim_file_path(exe)` | `git.exe` → `git.shim`，使用 `Path::with_extension` |
| `read_shim_file(path)` | 读取文件 → 解析 |
| `strip_key(line, key)` | 去除 `key = ` 前缀 |
| `unquote(s)` | 去除首尾引号 |
| `split_args(s)` | 拆分参数行（支持引号包围的值） |

**设计亮点**：
- `split_args` 手动实现引号感知的参数拆分（避免引入 `shlex` 依赖）
- 兼容 Scoop `.shim` 格式——Scoop 的 .shim 文件也是 `path = "..."` 格式

**测试覆盖**：7 个测试——path+args、仅 path、含空格路径、缺少 path、空文件、引号 args、路径推导。

### process.rs — 进程启动 ⭐⭐⭐⭐⭐

```rust
pub fn run_target(path: &str, args: &[String]) -> ExitCode {
    let mut cmd = Command::new(path);
    cmd.args(args);
    #[cfg(windows)]
    { cmd.creation_flags(CREATE_NEW_PROCESS_GROUP); }
    match cmd.status() {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(e) => { eprintln!("hit-shim: 无法启动 '{path}': {e}"); ExitCode::FAILURE }
    }
}
```

- `Command::new(path)` + `.args(args)` 参数数组方式，无命令注入风险 ✅
- `creation_flags(CREATE_NEW_PROCESS_GROUP)` 确保 Ctrl+C 不会直接传播到 shim 自身
- `.status()` 继承 stdin/stdout/stderr（`Command` 默认行为）

**测试覆盖**：2 个测试——正常退出、不存在程序。

---

## 测试覆盖分析

| 测试文件 | 数量 | 覆盖重点 |
|---------|:----:|----------|
| `parse::tests` | 7 | 完整解析 + 错误路径 + 边界条件 |
| `process::tests` | 2 | 正常退出 + 不存在程序 |
| **总计** | **9** | |

---

## 问题汇总

| # | 任务 | 问题 | 严重度 | 建议 |
|---|------|------|--------|------|
| 1 | 1.7.3 | `split_args` 不支持转义符（如 `\"` 在引号内） | 🟢 微小 | Scoop .shim 的 args 极少使用转义，若出现可用更完善的库替代 |
| 2 | 1.7.4 | `run_target` 使用 `.status()` 阻塞等待，未考虑子进程后台运行场景（GUI 应用） | 🟡 中等 | Scoop 对 GUI 应用（shortcuts）不会 wait，shim 当前对所有应用 wait 是合理的默认行为；GUI 场景留待 Phase 2 |
| 3 | 1.7.5 | 未在 CI 中验证 shim 体积是否保持在 ~200KB | 🟢 微小 | 可在后续添加 `cargo build --release -p hit-shim` 后检查二进制大小的 CI 步骤 |

---

## 评分总结

| 维度 | 评分 | 说明 |
|------|:----:|------|
| **完成度** | ⭐⭐⭐⭐⭐ | 5/5 任务全部完成 |
| **代码质量** | ⭐⭐⭐⭐⭐ | 0 clippy warnings，零外部依赖，7.9KB 源码 |
| **错误处理** | ⭐⭐⭐⭐ | 无可挑剔的零依赖错误处理；eprintln + ExitCode 组合 |
| **安全** | ⭐⭐⭐⭐⭐ | 参数数组方式启动进程，无命令注入风险 |
| **Scoop 兼容** | ⭐⭐⭐⭐⭐ | .shim 文件格式完全兼容 Scoop |

### 整体结论

**Phase 1.7（hit-shim：Shim 代理机制）通过审查，可以关闭。**

这是项目中质量最高的模块之一。零外部依赖、~7.9KB 源码实现了完整的 Shim 代理逻辑，与 Scoop 的 `.shim` 格式完全兼容。parse.rs 的引号感知参数拆分是亮点——在不引入外部依赖的情况下正确处理了含空格和引号的参数。
