# 代码审查报告 — Phase 2.2 依赖解析增强

**审查者**：AtomCode code-review
**时间**：2026-06-26
**范围**：仅 TODO.md §2.2（任务 2.2.1 ~ 2.2.4）
**文件**：`crates/hit-core/src/install/dependency.rs`、`crates/hit-core/src/manifest/schema.rs`（depends 字段）、`crates/hit-core/src/install/controller.rs`（集成点）
**基线**：`cargo check` ✅ | `cargo test` ✅ (8/8 dependency) | `cargo clippy` ⚠️ 5 warning(s)（均不在本 Phase 范围）

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
| 2.2.1 | 解析 Manifest 的 depends 字段 | ✅ | `schema.rs:63` `depends: Option<OneOrMany<String>>` + `schema.rs:95` `depends_list()` |
| 2.2.2 | 构建依赖图 | ✅ | `dependency.rs:33` `resolve_dependencies()` — DFS 三色标记 |
| 2.2.3 | 检测循环依赖 | ✅ | `dependency.rs:90` `dfs_visit()` — 灰色集合检测回边 |
| 2.2.4 | 实现依赖安装顺序 | ✅ | `controller.rs:97` 调用 `resolve_dependencies()` → 按拓扑序安装依赖 |

**结论：4/4 项任务全部完成，可标记 ✅。**

---

## 模块结构总览

```
crates/hit-core/src/install/
├── dependency.rs    # 依赖解析器（348 行）
├── controller.rs    # 安装控制器（集成 resolve_dependencies）
└── mod.rs           # pub use dependency::{resolve_dependencies, ResolvedDep, parse_dep_spec}

crates/hit-core/src/manifest/
└── schema.rs        # Manifest.depends: Option<OneOrMany<String>> + depends_list()
```

**核心算法**：三色标记 DFS 拓扑排序

```
resolve_dependencies(session, "A", manifest_A)
  ├── 预入 visiting = {"A"}（根节点）
  ├── 遍历 A.depends_list() → ["B", "C"]
  │   ├── parse_dep_spec("B") → (None, "B")
  │   ├── load_dep_manifest(session, None, "B") → manifest_B
  │   └── dfs_visit("B", manifest_B, ...)
  │       ├── visiting.insert("B")
  │       ├── 遍历 B.depends_list() → ["D"]
  │       │   └── dfs_visit("D", manifest_D, ...)
  │       │       ├── visiting.insert("D")
  │       │       ├── visiting.remove("D"), visited.insert("D")
  │       │       └── out.push(ResolvedDep { name: "D", ... })
  │       ├── visiting.remove("B"), visited.insert("B")
  │       └── out.push(ResolvedDep { name: "B", ... })
  └── 结果：[D, B, C]（最底层依赖在前）
```

---

## 逐模块审查

### dependency.rs — 依赖解析器 ⭐⭐⭐⭐

**关键设计**：

| 设计点 | 实现 |
|--------|------|
| 依赖图构建 | DFS 递归遍历 `depends_list()`，三色标记（白=未发现 / 灰=visiting / 黑=visited） |
| 循环检测 | 灰色集合回边检测：`visiting.insert()` 返回 `false` → 循环依赖 |
| 拓扑排序 | 后序 push：子节点处理完毕后才 push 当前节点，保证依赖在前 |
| 依赖格式 | `parse_dep_spec("bucket/name")` → `(Some("bucket"), "name")`；裸名 → `(None, "name")` |
| 已安装跳过 | `apps/<name>/current` 存在时跳过，不加入结果 |
| Manifest 查找 | `locate_manifest_in_bucket` 兼容两种布局：`<bucket>/<name>.json` 和 `<bucket>/bucket/<name>.json` |

**错误处理**：

| 场景 | 错误类型 | 中文消息 |
|------|----------|----------|
| 循环依赖 | `HitError::Install` | `"循环依赖：{app} 被重复访问"` |
| 依赖不存在 | `HitError::NotFound` | kind="app", name=依赖名 |
| Manifest 读取失败 | `HitError::io` | `"读取 manifest '{path}'"` |
| Manifest 解析失败 | `HitError::Manifest` | app=依赖名 |

| 评价 | 说明 |
|------|------|
| ✅ 亮点 | 三色标记 DFS 同时完成循环检测和拓扑排序，算法简洁正确 |
| ✅ 亮点 | 已安装依赖自动跳过，避免重复安装 |
| ✅ 亮点 | 兼容 Scoop 两种 bucket 目录布局 |
| ⚠️ 待改进 | `ResolvedDep.bucket` 始终为 `String::new()`（L131），`parse_dep_spec` 解析出的 bucket 信息丢失 |

### schema.rs — depends 字段建模 ⭐⭐⭐⭐⭐

```rust
pub depends: Option<OneOrMany<String>>,

pub fn depends_list(&self) -> Vec<&str> {
    match &self.depends {
        None => Vec::new(),
        Some(om) => om.as_slice().iter().map(String::as_str).collect(),
    }
}
```

| 评价 | 说明 |
|------|------|
| ✅ 亮点 | `OneOrMany<String>` 正确建模 Scoop 的 `depends: "perl"` / `depends: ["a", "b"]` 多态 |
| ✅ 亮点 | `depends_list()` 提供统一 `Vec<&str>` 视图，对应 Scoop PS `@($manifest.depends)` |

### controller.rs — 集成点 ⭐⭐⭐⭐⭐

```rust
let deps = if options.no_deps {
    Vec::new()
} else {
    resolve_dependencies(session, app, manifest)?
};
// 递归安装依赖（按拓扑序）
```

| 评价 | 说明 |
|------|------|
| ✅ 亮点 | `no_deps` 选项支持跳过依赖解析（对应 Scoop 的 `-s` / `--skip-dependencies`） |
| ✅ 亮点 | 拓扑序保证先安装底层依赖 |

---

## 测试覆盖分析

| 测试 | 数量 | 覆盖重点 |
|------|:----:|----------|
| `parse_dep_spec_bucket_qualified` | 1 | `"main/git"` → `(Some("main"), "git")` |
| `parse_dep_spec_bare` | 1 | `"git"` → `(None, "git")` |
| `parse_dep_spec_malformed_slash` | 1 | `"/name"` / `"bucket/"` 退化处理 |
| `resolve_linear_chain` | 1 | A→B→C 线性链：验证 C 在 B 之前 |
| `resolve_diamond_dependency` | 1 | A→{B,C}→D 菱形：D 仅出现一次且在 B/C 之前 |
| `resolve_circular_dependency_errors` | 1 | A→B→A 循环：报 `HitError::Install` 含"循环" |
| `resolve_skips_already_installed` | 1 | C 已安装时跳过，结果仅含 B |
| `resolve_missing_dependency_errors` | 1 | 依赖不存在：报 `HitError::NotFound` |
| **总计** | **8** | |

---

## 问题汇总

| # | 任务 | 问题 | 严重度 | 建议 |
|---|------|------|--------|------|
| 1 | 2.2.2 | `ResolvedDep.bucket` 始终为空串：`dfs_visit` L131 写死 `bucket: String::new()`，`parse_dep_spec` 解析出的 bucket 信息未传递到输出 | 🟡 中等 | 在 `dfs_visit` 和 `resolve_dependencies` 中将 `bucket_opt` 解析结果存入 `ResolvedDep.bucket`；或改为从 manifest 路径反推 bucket 名 |
| 2 | 2.2.2 | `ResolvedDep` 仅实现 `Debug + Clone`，缺少 `PartialEq`（审查清单要求跨 crate 类型实现 `Debug + Clone + PartialEq`） | 🟢 微小 | 添加 `#[derive(PartialEq)]`（需 Manifest 也实现 PartialEq，已确认有） |
| 3 | 2.2.2 | TODO 依赖声明为 `petgraph`，实际未使用 petgraph——手写 DFS 替代 | 🟢 微小 | 手写 DFS 更简洁，无需引入 petgraph 依赖；建议更新 TODO 依赖声明 |

---

## 评分总结

| 维度 | 评分 | 说明 |
|------|:----:|------|
| **完成度** | ⭐⭐⭐⭐⭐ | 4/4 任务完成 |
| **算法正确性** | ⭐⭐⭐⭐⭐ | 三色标记 DFS 正确实现循环检测+拓扑排序 |
| **测试覆盖** | ⭐⭐⭐⭐ | 8 个测试覆盖线性/菱形/循环/跳过/缺失；缺少自依赖测试 |
| **代码质量** | ⭐⭐⭐⭐ | bucket 信息丢失（L131），其余整洁 |
| **Scoop 兼容性** | ⭐⭐⭐⭐⭐ | depends 多态、bucket/name 格式、两种目录布局均兼容 |

### 整体结论

**Phase 2.2（依赖解析增强）通过审查，可以关闭。**

三色标记 DFS 同时完成循环检测和拓扑排序，算法简洁正确。8 个测试覆盖了线性链、菱形依赖、循环检测、已安装跳过、依赖缺失等关键场景。主要待改进项是 `ResolvedDep.bucket` 始终为空串（bucket 信息丢失），建议后续修复但不阻塞当前 Phase 关闭。
