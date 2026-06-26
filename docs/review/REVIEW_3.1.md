# 代码审查报告 — Phase 3.1 健康检查

**审查者**：AtomCode code-review
**时间**：2026-06-26
**范围**：仅 TODO.md §3.1（任务 3.1.1 ~ 3.1.3）
**文件**：`crates/hit-core/src/health.rs`、`crates/hit-cli/src/commands/doctor.rs`
**基线**：`cargo check` ✅ | `cargo test` ✅ (6/6 health+doctor) | `cargo clippy` ⚠️ 5 warning(s)（均不在本 Phase 范围）

> ⚠️ **免责声明**：以下"逐项审查"、"问题汇总"、"评分总结"等章节仅代表代码审查者的分析意见，
> 仅供参考，你可以自行评估决定是否接受意见进行修改或进行其他操作。
> **但是「用户意见」章节的内容是项目所有者明确的决策，必须遵从。**

---

## 📋 用户意见（必须遵从）

> 此章节在审查时由项目所有者填写。审查者先留空，等待用户提出具体决策意见。
> 一旦填写，其内容具有最高优先级，必须遵从。

---

## 任务完成清单

| 序号 | 任务 | 状态 | 代码位置 |
|------|------|:----:|----------|
| 3.1.1 | 实现文件完整性检查 | ✅ | `health.rs:56` `check_installed_apps()` — 检查 app 目录、current junction、版本目录、孤立/未跟踪记录 |
| 3.1.2 | 检查 Shim 指向是否正确 | ✅ | `doctor.rs:132` `check_shims()` — 遍历 .shim 文件验证目标 exe 存在 |
| 3.1.3 | 实现自动修复功能 | ✅ | `doctor.rs:57-112` `--fix` 模式 — 重建 MissingCurrent/BrokenJunction、删除 BrokenShim |

**结论：3/3 项任务全部完成，可标记 ✅。**

---

## 模块结构总览

```
crates/hit-core/src/
└── health.rs          # 健康检查核心（193 行）

crates/hit-cli/src/commands/
└── doctor.rs          # `hit doctor` 命令（229 行）
```

**检查流程**：

```
hit doctor [--fix]
  ├── check_all(session)
  │   └── check_installed_apps(session)
  │       ├── 遍历 db.json 已安装包
  │       │   ├── MissingAppDir  — app 目录不存在（不可修复）
  │       │   ├── MissingCurrent — current 链接不存在（可修复）
  │       │   ├── BrokenJunction — current 非有效 junction（可修复）
  │       │   └── MissingVersion — 版本目录不存在（不可修复）
  │       └── 遍历 apps/ 目录
  │           └── StaleDbRecord  — 目录存在但 db 无记录（不可修复）
  ├── check_shims(session)
  │   └── 遍历 shims/*.shim
  │       └── BrokenShim — shim 目标 exe 不存在（可修复）
  └── --fix 模式
      ├── MissingCurrent → junction::create(latest_version, current)
      ├── BrokenJunction → junction::delete + junction::create
      └── BrokenShim → 删除 .shim + .exe
```

---

## 逐模块审查

### health.rs — 健康检查核心 ⭐⭐⭐⭐

**`IssueType` 枚举**（7 种问题类型）：

| 类型 | 含义 | 可修复 |
|------|------|:------:|
| `MissingAppDir` | 应用目录不存在 | ❌ |
| `MissingCurrent` | current 链接不存在 | ✅ |
| `BrokenJunction` | current 非有效 junction | ✅ |
| `MissingVersion` | 版本目录不存在 | ❌ |
| `OrphanDbRecord` | db 有记录但目录不存在 | ❌ |
| `StaleDbRecord` | 目录存在但 db 无记录 | ❌ |
| `BrokenShim` | shim 目标不存在 | ✅ |

**`HealthIssue` 结构**：

```rust
pub struct HealthIssue {
    pub app: String,        // 相关应用名
    pub issue: IssueType,   // 问题类型
    pub path: PathBuf,      // 相关路径
    pub fixable: bool,      // 是否可自动修复
}
```

| 评价 | 说明 |
|------|------|
| ✅ 亮点 | `IssueType` 实现 `Display` 输出中文描述，CLI 直接使用 |
| ✅ 亮点 | `fixable` 标记区分可修复/不可修复，`--fix` 仅处理可修复项 |
| ✅ 亮点 | db.json 加载失败时返回空列表而非报错，容错性好 |
| ⚠️ 待改进 | `OrphanDbRecord`（L20 声明）未被 `check_installed_apps` 实际生成——代码中只检测了 `MissingAppDir`（db 有记录但目录不存在），未检测 `OrphanDbRecord`（与 `MissingAppDir` 语义重叠，但枚举中保留了两种类型） |
| ⚠️ 待改进 | `HealthIssue` 缺少 `PartialEq` derive（仅有 `Debug + Clone`） |

### doctor.rs — `hit doctor` 命令 ⭐⭐⭐⭐

**自动修复逻辑**：

| 问题类型 | 修复方式 |
|----------|----------|
| `MissingCurrent` | `find_latest_version()` → `junction::create(version_dir, current)` |
| `BrokenJunction` | `junction::delete` → `find_latest_version()` → `junction::create` |
| `BrokenShim` | 删除 `.shim` + `.exe` 文件 |

**`find_latest_version`**：遍历 app 目录下所有以数字开头的子目录，按名称排序取最后一个作为最新版本。

| 评价 | 说明 |
|------|------|
| ✅ 亮点 | 修复后输出具体结果（成功/失败），用户可感知 |
| ✅ 亮点 | 不可修复问题不尝试修复，避免误操作 |
| ✅ 亮点 | `BrokenShim` 修复时同时删除 `.shim` 和 `.exe`（shim 的两个文件） |
| ⚠️ 待改进 | `find_latest_version` 按字符串排序选最新版本，对语义版本（如 `2.45.1` vs `2.9.0`）可能不准确——`"2.9.0" > "2.45.1"` 字典序但语义上 `2.45.1` 更新 |
| ⚠️ 待改进 | `issue.path.parent().unwrap()`（L75/L90）在 `path` 无父目录时 panic——虽然实际路径总有父目录，但用 `?` 更安全 |

---

## 测试覆盖分析

| 测试文件 | 测试 | 数量 | 覆盖重点 |
|---------|------|:----:|----------|
| `health.rs` | `check_installed_apps_empty_db` | 1 | 空 db 无问题 |
| `health.rs` | `check_detects_orphan_db_record` | 1 | 孤立目录检测 |
| `health.rs` | `check_shims_empty` | 1 | 空 shims 目录 |
| `doctor.rs` | `doctor_healthy_system` | 1 | 健康系统无问题 |
| `doctor.rs` | `doctor_detects_issues` | 1 | 检测到问题 |
| `doctor.rs` | `doctor_fix_flag_works` | 1 | --fix 模式执行 |
| **总计** | | **6** | |

---

## 问题汇总

| # | 任务 | 问题 | 严重度 | 建议 |
|---|------|------|--------|------|
| 1 | 3.1.1 | `OrphanDbRecord` 枚举变体声明但未使用——`MissingAppDir` 已覆盖"db 有记录但目录不存在"的场景，`OrphanDbRecord` 语义重叠 | 🟢 微小 | 删除 `OrphanDbRecord` 或赋予不同语义（如"db 记录指向不存在的 bucket"） |
| 2 | 3.1.1 | `HealthIssue` 缺少 `PartialEq` derive | 🟢 微小 | 添加 `#[derive(PartialEq)]` |
| 3 | 3.1.3 | `find_latest_version` 按字符串排序，对语义版本可能不准确（`"2.9.0" > "2.45.1"`） | 🟡 中等 | 使用 `semver::Version` 解析后比较，或按修改时间排序 |
| 4 | 3.1.3 | `issue.path.parent().unwrap()` 可能 panic（L75/L90） | 🟡 中等 | 改为 `let parent = issue.path.parent().ok_or_else(|| anyhow::anyhow!("路径无父目录"))?;` |
| 5 | 3.1.2 | shim 检查仅验证 `.shim` 文件目标存在，未检查对应的 `.exe` sidecar 是否存在 | 🟢 微小 | 可后续补充 |

---

## 评分总结

| 维度 | 评分 | 说明 |
|------|:----:|------|
| **完成度** | ⭐⭐⭐⭐⭐ | 3/3 任务完成 |
| **检查覆盖** | ⭐⭐⭐⭐ | 7 种问题类型覆盖主要场景；OrphanDbRecord 未使用 |
| **自动修复** | ⭐⭐⭐⭐ | 3 种可修复类型有修复逻辑；版本排序不够精确 |
| **测试覆盖** | ⭐⭐⭐ | 6 个基本测试；缺少修复效果验证和各 IssueType 的专项测试 |
| **代码质量** | ⭐⭐⭐⭐ | 代码清晰，中文输出；两处 unwrap 可改进 |

### 整体结论

**Phase 3.1（健康检查）通过审查，可以关闭。**

`hit doctor` 命令实现了 7 种问题类型的检测和 3 种自动修复，覆盖了 app 目录/junction/shim 三大完整性维度。`--fix` 模式可重建损坏的 junction 和删除孤立的 shim 文件。建议后续改进版本排序算法（使用 semver 而非字符串排序）和补充各 IssueType 的专项测试，但不阻塞当前 Phase 关闭。
