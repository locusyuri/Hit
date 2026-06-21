# 代码审查报告 — Phase 1.5 hit-core/compress：解压模块

**审查者**：AtomCode code-review  
**时间**：2026-06-21  
**范围**：仅 TODO.md §1.5（任务 1.5.1 ~ 1.5.4）  
**文件**：`crates/hit-core/src/compress/` × 5  
**基线**：`cargo check` ✅ | `cargo test` ✅ (143/143, 4 ignored) | `cargo clippy` ✅ (0 warnings)

> ⚠️ **免责声明**：以下"逐项审查"、"问题汇总"、"评分总结"等章节仅代表代码审查者的分析意见，仅供参考，你可以自行评估决定是否接受意见进行修改或进行其他操作。**但是「用户意见」章节的内容是项目所有者明确的决策，必须遵从。**

---

## 📋 用户意见（必须遵从）

> 此章节在审查时由项目所有者填写。审查者先留空，等待用户提出具体决策意见。一旦填写，其内容具有最高优先级，必须遵从。

---

## 任务完成清单

| 序号 | 任务 | 状态 | 代码位置 |
|------|------|:----:|----------|
| 1.5.1 | 实现 ZIP 解压（zip crate） | ✅ | `compress/zip.rs` |
| 1.5.2 | 实现 7z 解压（sevenz-rust2） | ✅ | `compress/sevenz.rs` |
| 1.5.3 | 实现 TAR 解压（tar + flate2 等） | ✅ | `compress/tar.rs` |
| 1.5.4 | 支持安装程序处理（NSIS/Inno/MSI） | ✅ | `compress/installer.rs` |

**结论：4/4 项任务全部完成，可标记 ✅。**

---

## 模块结构总览

```
src/compress/
├── mod.rs        # 统一入口 decompress() + detect_format() + ArchiveFormat 枚举
├── zip.rs        # ZIP 解压（zip crate v2）
├── sevenz.rs     # 7z 解压（sevenz-rust2）+ tar-in-7z 递归
├── tar.rs        # TAR 系列（tar / tar.gz / tar.bz2 / tar.xz）
└── installer.rs  # NSIS / Inno Setup / MSI 安装程序
```

---

## 逐模块审查

### mod.rs — 统一入口 + 格式检测 ⭐⭐⭐⭐⭐

**`ArchiveFormat` 枚举**覆盖 8 种格式：Zip / SevenZip / Gzip / Bzip2 / Xz / Tar / Msi / Exe。

**`detect_format()`** 基于文件魔数（magic bytes）识别，不依赖扩展名：

| 格式 | 魔数 | 字节数 |
|------|------|:------:|
| 7z | `37 7A BC AF 27 1C` | 6 |
| Xz | `FD 37 7A 58 5A 00` | 6 |
| Zip | `PK\x03\x04` / `PK\x05\x06` | 4 |
| bzip2 | `42 5A 68` | 3 |
| gzip | `1F 8B` | 2 |
| Tar | `ustar` at offset 257 | 5 |
| MSI | `D0 CF 11 E0 A1 B1 1A E1` | 8 |
| Exe | `MZ` | 2 |

检测策略正确——长签名优先匹配（7z/xz 先于 zip），避免误判。

**`decompress()`** 统一路由：detect → 分发 → 事件上报，业务逻辑清晰。

### zip.rs — ZIP 解压 ⭐⭐⭐⭐⭐

| 特性 | 说明 |
|------|------|
| `extract_dir` | 子目录过滤 + 前缀去除，与 Scoop 行为一致 |
| 错误处理 | 逐步骤完整，含中文错误消息 |
| 安全 | 使用 `entry.enclosed_name()`（zip crate 内置的路径穿越防护） |

**测试覆盖**：6 个测试——平坦/嵌套/extract_dir/空/corrupt。

### sevenz.rs — 7z 解压 ⭐⭐⭐⭐

| 特性 | 说明 |
|------|------|
| `sevenz-rust2` | 活跃 fork，替代原版 sevenz-rust |
| extract_dir 策略 | 先全量到 tmp，再移动子目录（extract-then-move） |
| tar-in-7z 递归 | 提取后扫描 `.tar` 文件，递归调用 `extract_tar` 并删除中间文件 |

**测试覆盖**：2 个测试（不存在文件/无效文件），缺少对正常 7z 文件提取的测试。⚠️ 7z 测试文件构造复杂，当前覆盖合理。

### tar.rs — TAR 解压 ⭐⭐⭐⭐⭐

四种入口函数：`extract_tar` / `extract_tar_gz` / `extract_tar_bz2` / `extract_tar_xz`。

| 特性 | 说明 |
|------|------|
| 路径遍历防护 | ✅ 拒绝 `../` 和绝对路径 |
| extract_dir | ✅ 前缀匹配 + 移动策略 |
| 格式覆盖 | tar / tar.gz / tar.bz2 / tar.xz |

**安全审查通过**——`extract_tar_inner` 在解压前做了两项安全检查：

```rust
// 拒绝路径遍历
if raw_path.components().any(|c| matches!(c, std::path::Component::ParentDir)) { ... }
// 拒绝绝对路径
if raw_path.is_absolute() { ... }
```

**测试覆盖**：7 个测试——4 种 tar 格式 + extract_dir + **路径遍历攻击检测**（重要！）。

### installer.rs — 安装程序 ⭐⭐⭐⭐

| 函数 | 说明 |
|------|------|
| `run_msi_extract` | `msiexec /a /qn` administrative install 提取到目标目录 |
| `run_installer` | NSIS（默认 `args`）或 Inno Setup（`/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /DIR=`）|

**安全考量**：使用 `Command::new().arg()` 参数数组而非字符串拼接，防止命令注入。

**测试覆盖**：3 个测试——文件不存在、参数构建验证。无法在测试环境实际执行安装程序，当前覆盖合理。

---

## 测试覆盖分析

| 测试文件 | 数量 | 覆盖重点 |
|---------|:----:|----------|
| `compress::tests`（mod.rs） | 10 | detect_format 全格式 + 未知/过小 |
| `compress::zip::tests` | 6 | 平坦/嵌套/extract_dir/空/corrupt |
| `compress::sevenz::tests` | 2 | 不存在/无效文件 |
| `compress::tar::tests` | 7 | 4 种 tar 格式 + extract_dir + 路径穿越防御 |
| `compress::installer::tests` | 3 | 不存在文件 + innosetup 参数验证 |
| **压缩模块总计** | **28** | |
| 其他已有 | 115 | |
| **总计** | **143** | **(4 ignored 网络)** |

---

## 问题汇总

| # | 任务 | 问题 | 严重度 | 建议 |
|---|------|------|--------|------|
| 1 | 1.5.2 | `sevenz.rs` 缺少正常 7z 文件提取测试 | 🟢 微小 | 7z 测试文件构造复杂，可接受 |
| 2 | 1.5.3 | `tar.rs` 中 `extract_tar_inner` 同时使用了 `unpack_in` + 手动 rename 两套机制 | 🟡 中等 | unpack_in 解压到原始路径后手动移动，逻辑正确但有性能损耗。建议在 extract_dir 模式下直接用 `entry.path()` 拼接目标路径 |
| 3 | 1.5.4 | `run_installer` 未设置超时，安装程序可能无限阻塞 | 🟡 中等 | 建议添加 `cmd.timeout(Duration::from_secs(600))` |
| 4 | 1.5.4 | `run_msi_extract` 调用 `msiexec` 在非 Windows 平台编译失败 | 🟢 微小 | 可通过 `#[cfg(windows)]` 条件编译隔离——但不影响当前开发 |

---

## 评分总结

| 维度 | 评分 | 说明 |
|------|:----:|------|
| **完成度** | ⭐⭐⭐⭐⭐ | 4/4 任务全部完成，格式覆盖全面 |
| **代码质量** | ⭐⭐⭐⭐⭐ | 0 clippy warnings，无 unsafe，层次分明 |
| **错误处理** | ⭐⭐⭐⭐⭐ | 全部 HitError，中文消息 |
| **测试覆盖** | ⭐⭐⭐⭐ | 28 个测试覆盖主要路径；7z 真实提取和安装器执行受限于环境 |
| **安全** | ⭐⭐⭐⭐⭐ | TAR 路径遍历防御、ZIP enclosed_name、参数数组防止命令注入 |

### 整体结论

**Phase 1.5（hit-core/compress：解压模块）通过审查，可以关闭。**

统一入口 `decompress()` + `detect_format()` 的设计是该模块最大的架构亮点，8 种格式覆盖 Scoop 生态的全部常见类型。TAR 子模块的路径遍历安全检查是重要的防御性编程实践。建议在 Phase 2 为 installer 添加超时配置。
