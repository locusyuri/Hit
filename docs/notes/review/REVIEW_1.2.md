# 代码审查报告 — Phase 1.2 Scoop Manifest 格式兼容解析

**审查者**：AtomCode code-reviewer  
**时间**：2026-06-15  
**范围**：仅 TODO.md §1.2（任务 1.2.1 ~ 1.2.7）  
**文件**：`crates/hit-core/src/manifest/`（7 文件）+ `crates/hit-core/tests/`（5 文件）  
**基线**：`cargo test -p hit-core` ✅（119/119）| `cargo clippy` ✅（0 warnings）

---

## 任务完成清单

| 序号 | 任务 | 状态 | 代码位置 |
|------|------|:----:|----------|
| 1.2.1 | 分析 Scoop Manifest JSON Schema | ✅ | `ref/Main/bucket/` 真实 manifest |
| 1.2.2 | 定义 Manifest 数据结构 schema.rs | ✅ | `src/manifest/schema.rs` |
| 1.2.3 | 实现变量替换引擎 variables.rs | ✅ | `src/manifest/variables.rs` |
| 1.2.4 | 实现 Manifest 解析器 parser.rs | ✅ | `src/manifest/parser.rs` |
| 1.2.5 | 实现 Manifest 验证器 validator.rs | ✅ | `src/manifest/validator.rs` |
| 1.2.6 | 支持 Scoop 特殊字段 | ✅ | schema.rs 覆盖 |
| 1.2.7 | 编写 Manifest 解析单元测试 | ✅ | `tests/manifest_*.rs` × 5 |

**结论：7 项任务全部完成，可标记 ✅。**

---

## 模块结构总览

```
src/manifest/
├── mod.rs           # 子模块导出 & pub use 重导出
├── schema.rs        # Manifest 数据结构（865 行）
├── parser.rs        # 解析器 + FlatManifest（418 行）
├── variables.rs     # 变量替换引擎（760 行）
├── validator.rs     # Manifest 验证器（446 行）
└── diagnostic.rs    # 诊断消息类型（130 行）

tests/
├── manifest_parser.rs       # 9 集成测试
├── manifest_smoke.rs        # 9 冒烟测试（含真实 fixture）
├── manifest_test.rs         # 21 功能测试
├── manifest_validator.rs    # 26 验证器测试
└── manifest_variables.rs    # 9 变量替换测试

fixtures/
└── manifest/                # 6 个真实 Scoop manifest
    ├── git.json / 7zip.json / python.json
    ├── nodejs.json / ack.json / aws-sam-cli.json
```

---

## 逐模块审查

### schema.rs — 数据结构定义 ⭐⭐⭐⭐⭐

**Manifest 结构体**完整覆盖 Scoop 所有核心字段：

| 分类 | 字段 | 建模方式 |
|------|------|----------|
| 必填 | version / description / homepage / license | `String` + `License` 枚举（字符串/对象） |
| 下载 | url / hash / architecture / extract_dir / extract_to / cookie | `OneOrMany` + `HashField` + `Architecture` |
| 集成 | bin / shortcuts / env_set / env_add_path / persist | 手写 `BinList`/`PersistList`/`ShortcutItem` Visitor |
| 脚本 | pre/post_install / pre/post_uninstall / installer / uninstaller | `ScriptField`（单行/多行） |
| 依赖 | depends / suggest | `OneOrMany` + `BTreeMap` |
| 元信息 | notes / checkver / autoupdate / innosetup / psmodule | `CheckverField` / `Autoupdate` / `PowerShellModule` |
| 扩展 | alias | `#[serde(default, skip_serializing_if)]` 占位 |

**多态字段的手写 Visitor**：

| 字段 | JSON 多态形式 | 实现 |
|------|-------------|------|
| `bin` | `string \| [string] \| [tuple[2..=5]]` | `BinItemVisitor`（`deserialize_any`） |
| `persist` | `string \| [string \| tuple[2]]` | `PersistItemVisitor` |
| `shortcuts` | `[tuple[2..=4]]` | `ShortcutVisitor` |
| `license` | `string \| { identifier, url? }` | `License` 枚举 |
| `hash` | `string \| [string]` | `HashField` 枚举 |
| `checkver` | `string \| { github \| url \| script ... }` | `CheckverField` + `Checkver` |
| `autoupdate` | `{ architecture?, hash? }` | `Autoupdate` + `AutoupdateHash` |
| `url` / `depends` / `env_add_path` / `extract_dir` / `extract_to` | `string \| [string]` | 泛型 `OneOrMany<T>` |

✅ `#[serde(default)]` 顶层属性，所有缺失字段优雅降级  
✅ 结构体、枚举核心类型都 derive 了 `Debug + Clone + Default + Serialize + Deserialize`  
✅ `alias` 字段已预留但 `skip_serializing_if` 避免影响输出

### parser.rs — 解析器 ⭐⭐⭐⭐⭐

| 功能 | 函数 | 说明 |
|------|------|------|
| 解析 JSON | `parse_str(input)` → `Result<Manifest>` | 单步解析 |
| 架构合并 | `FlatManifest::resolve_architecture(m, arch)` | 将架构分支合并到顶层 |
| 一键解析+合并 | `parse_and_resolve(input, arch)` | parse + resolve_architecture 组合 |
| 脚本解析 | `resolve_script(hook)` | 按 HookType 返回对应脚本 |
| 架构判断 | `supports_arch()` / `supported_architectures()` | 检查 manifest 支持哪些平台 |
| 环境变量收集 | `collect_all_env_sets()` | 收集所有架构的 env_set |

**`FlatManifest` 设计合理**：
- `resolve_architecture()` 将 `architecture.64bit` 等分支的字段合并到顶层
- 遵循 Scoop 的 `arch_specific` 逻辑（`ref/Scoop/lib/core.ps1`）
- 合并后 `architecture` 字段置为 `None`，避免重复引用

### variables.rs — 变量替换引擎 ⭐⭐⭐⭐⭐

**支持的变量**：

| 变量 | 来源 | 示例 |
|------|------|------|
| `$version` | InstallVars / AutoupdateVars | `2.45.1` |
| `$dotVersion` | AutoupdateVars | 替换 `_-` → `.` |
| `$underscoreVersion` | AutoupdateVars | 替换 `.-` → `_` |
| `$dashVersion` | AutoupdateVars | 替换 `._` → `-` |
| `$cleanVersion` | AutoupdateVars | 去掉所有分隔符 |
| `$preReleaseVersion` | AutoupdateVars | `-` 之后的部分 |
| `$major/minor/patch/buildVersion` | AutoupdateVars | 语义化版本分段 |
| `$matchHead` / `$matchTail` | AutoupdateVars | 版本号头部与剩余 |
| `$match*` (自定义) | AutoupdateVars | checkver 捕获组 |
| `$architecture` | InstallVars | `64bit` / `32bit` / `arm64` |
| `$url` / `$fname` / `$filename` | UrlContext | 下载 URL 衍生 |
| `$dir` / `$appdir` / `$scoopdir` / `$persist_dir` | InstallVars | 安装路径 |

**替换范围**（`substitute_manifest_in_place`）：
- 顶层字段：url / hash / bin / shortcuts / env_set / persist / pre/post_install 等
- 架构分支：每个 arch 的 url / hash / bin / shortcuts / installer
- autoupdate：各架构的 url / hash（含 Fetch 模式）
- checkver：url / regex / jsonpath

### validator.rs — 验证器 ⭐⭐⭐⭐⭐

**检查项清单**：

| 类别 | 检查项 | 严重度 |
|------|--------|:------:|
| 必填 | version 格式 | Error |
| 必填 | description 非空 | Error |
| 必填 | homepage 合法 URI | Error |
| 必填 | license 存在 | Error |
| 必填 | 至少一个 URL（顶层或架构内） | Error |
| URL | URL 与 hash 数量一致 | Error |
| URL | hash 格式合法（sha256/sha512/blake3） | Error |
| URL | HTTP URL 警告（建议 HTTPS） | Warning |
| checkver | regex 语法有效 | Error |
| checkver | github/url 合法 URI | Error |
| checkver | sourceforge 非空 | Error |
| env_set | 值为空 | Warning |
| installer | script 非空 | Error |
| SPDX | 许可证标识符是否已知 | Warning |
| 信息 | 缺少 maintainer 注释 | Info |

✅ 使用 `Regex` T 的 `OnceLock` 惰性编译，避免重复初始化  
✅ SPDX 列表覆盖 29 个常见许可证  
✅ `looks_like_uri` 快速检测 URI 合法性

### diagnostic.rs — 诊断类型 ⭐⭐⭐⭐⭐

```rust
pub struct Diagnostics { items: Vec<Diagnostic> }
// Diagnostic { severity: Severity, field: String, message: String }
```

- `push_error` / `push_warning` / `push_info` 便捷方法
- `has_errors()` / `has_warnings()` / `errors()` / `warnings()` / `infos()` 过滤
- `into_result(app)`：存在 error 时返回 `Err(HitError::Manifest{app, message})`

---

## 测试覆盖分析

### 测试统计数据

| 测试文件 | 数量 | 覆盖重点 |
|---------|:----:|----------|
| manifest unit tests（`#[cfg(test)]` 内联） | 45 | 类型行为（OneOrMany、PersistItem、BinItem、License、HookType） |
| `tests/manifest_parser.rs` | 9 | 架构合并、fallback、多架构支持 |
| `tests/manifest_smoke.rs` | 9 | 真实 fixture 解析/验证/roundtrip |
| `tests/manifest_test.rs` | 21 | 完整流水线（parse → resolve → substitute → validate） |
| `tests/manifest_validator.rs` | 26 | 每个检查项独立测试（error/warning/info） |
| `tests/manifest_variables.rs` | 9 | 变量替换、autoupdate vars、URL context |
| **总计** | **119** | |

### 真实 Scoop manifest fixture

从 `ref/Main/bucket/` 提取的 6 个真实 manifest（`include_str!` 编译期嵌入），覆盖了 Scoop 清单的绝大部分多态场景：

| Fixture | 覆盖的多态特征 |
|---------|---------------|
| git.json | license 对象、shortcuts tuple[3]、pre_install 多行脚本、autoupdate hash Fetch 模式、64bit+arm64 |
| python.json | bin tuple[2] alias、license 字符串、installer.script |
| 7zip.json | persist 字符串数组、hash 字符串 |
| nodejs.json | persist 字符串数组、bin 路径 |
| ack.json | depends 单字符串、bin 单字符串、hash 单字符串 |
| aws-sam-cli.json | depends + suggest 并存、bin 混 string+tuple、license MIT 字符串 |

---

## 问题汇总

| # | 任务 | 问题 | 严重度 | 建议 |
|---|------|------|--------|------|
| 1 | 1.2.2 | `Manifest` 未实现 `PartialEq` | 🟢 微小 | 测试中的 roundtrip 只能比较 `version` 和 `description` 少数字段，如果 derive `PartialEq` 可做全字段 assert_eq |
| 2 | 1.2.5 | SPDX 列表目前为 29 个常见协议 | 🟢 微小 | 建议补充完整列表（或使用 `spdx` crate），当前数量不影响验证功能 |
| 3 | 1.2.2 | `OneOrMany<T>` 序列化时始终输出数组 | 🟡 建议 | 当前 `serialize` 对 `One` 也输出数组格式，Scoop 允许但某些工具期望单值时为字符串。测试未发现兼容性问题 |
| 4 | 1.2.3 | `substitute_manifest_in_place` 对 `ScriptField::Lines` 不递归替换 | 🟢 注意 | 脚本内的变量引用（如 `$version` 出现在 pre_install 中）不会被替换。Scoop 实际也不替换脚本，当前行为一致 |

---

## 评分总结

| 维度 | 评分 | 说明 |
|------|:----:|------|
| **完成度** | ⭐⭐⭐⭐⭐ | 7/7 项任务全部完成，覆盖所有 Scoop manifest 字段 |
| **数据建模** | ⭐⭐⭐⭐⭐ | 多态字段的手写 Visitor 质量高，覆盖 8 种多态模式 |
| **变量替换** | ⭐⭐⭐⭐⭐ | 完整支持所有 Scoop 变量 + autoupdate 衍生变量 |
| **验证器** | ⭐⭐⭐⭐⭐ | 15+ 检查项覆盖必填/格式/一致性/安全 |
| **测试覆盖** | ⭐⭐⭐⭐⭐ | 119 测试，含 6 个真实 Scoop fixture，0 失败 |
| **代码质量** | ⭐⭐⭐⭐⭐ | 无 clippy warnings，无 unsafe，注释完整 |

### 整体结论

**Phase 1.2（Scoop Manifest 格式兼容解析）通过审查，可以关闭。**

这是目前项目中质量最高的模块。schema.rs 的多态字段建模、variables.rs 的替换引擎深度、validator.rs 的检查项覆盖，以及对 Scoop 真实 fixture 的测试验证，都达到了生产就绪水准。
