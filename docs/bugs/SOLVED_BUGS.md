# 已解决的 Bug 记录

> 从 BUGS.md 迁移过来的已修复 Bug，保留历史记录。星星数量表示严重程度，越多越严重。

---

## 欢迎页面未在 `hit --help` 触发 ⭐

执行 `hit --help` 后没有显示欢迎页面，而是直接显示了帮助信息。
之后执行别的命令 `hit bucket list` 时，才显示了欢迎页面。

**修复**：`hit-cli/src/main.rs` — 将 Session 创建和欢迎页检查移到 `Cli::parse()` 之前，避免 clap 拦截 `--help` 后直接退出导致欢迎流程无法触发。

**提交**：`94c6d41` — fix(bugs): welcome before --help; config default_path exe fallback

---

## root 路径未写入 config.json ⭐⭐⭐⭐⭐

使用安装脚本从本地构建的文件安装 hit，并指定了安装路径：

```powershell
.\scripts\install-hit.ps1

╔══════════════════════════════════════════╗
║      Hit 安装向导                        ║
║  直接回车使用默认值，一路 Enter 即可       ║
╚══════════════════════════════════════════╝

请输入安装路径（默认: C:\Users\Violet\.hit）: C:\Users\Violet\Downloads\test\hit

  安装方式：
    1) 从 GitHub 下载（默认）
    2) 使用本地编译的 exe（开发调试）
请选择 [1/2]（默认: 1）: 2
请输入 hit.exe 路径: C:\Repos\Hit\target\release\hit.exe


╔══════════════════════════════════════════╗
║  开始安装...                             ║
╚══════════════════════════════════════════╝

[Hit] 检查 PowerShell 版本...
[Hit] 设置执行策略 RemoteSigned (CurrentUser)...
[Hit] 使用本地二进制：C:\Repos\Hit\target\release\hit.exe
[Hit] 初始化目录布局...
[Hit] hit.exe 已部署到 C:\Users\Violet\Downloads\test\hit
[Hit] 默认配置已写入 C:\Users\Violet\Downloads\test\hit\config.json
[Hit] 注册 shims 目录到用户 PATH...
[Hit] 已追加 C:\Users\Violet\Downloads\test\hit\shims 到 HKCU\Environment\Path

[Hit] Hit 安装完成！

    安装路径：C:\Users\Violet\Downloads\test\hit
    二进制  ：C:\Users\Violet\Downloads\test\hit\hit.exe
    配置    ：C:\Users\Violet\Downloads\test\hit\config.json
    Shims   ：C:\Users\Violet\Downloads\test\hit\shims

    请重新打开终端让 PATH 生效，然后运行：

        hit --help
        hit bucket add main
        hit install <package>
```

但 `C:\Users\Violet\Downloads\test\hit\config.json` 中 `root_path` 为 `null`：

```json
{
  "proxy": null,
  "mirror": null,
  "aria2_enabled": false,
  "no_junction": false,
  "root_path": null,
  "auto_cleanup_days": 30,
  "health_check_interval_days": 7
}
```

导致后续添加 main bucket 时，安装到了默认路径 `C:\Users\Violet\.hit\buckets\main`，而非指定的 `C:\Users\Violet\Downloads\test\hit\buckets\main`。

**修复**：
- `scripts/install-hit.ps1` — `root_path` 写入用户指定的安装路径（原为 `null`）；新增 `HIT_ROOT` 环境变量注册到 `HKCU\Environment`。
- `crates/hit-common/src/config.rs` — `default_path()` 增加 exe 同目录回退（无需 `HIT_ROOT` 环境变量也能找到自定义路径下的 config）。

**提交**：
- `94c6d41` — fix(bugs): welcome before --help; config default_path exe fallback
- `9c1d244` — fix(install): root_path写入config.json;注册HIT_ROOT环境变量

---

## 格式 ⭐（2026-06-27 解决）

`hit status` 展示的内容未对齐:
```bash
hit status
Hit 0.1.0

  已安装软件:           0
  Bucket 数量:       0
  可用软件总数:          0
  缓存文件:            0 (0 B)
  根目录:             C:\Users\Violet\Downloads\test\hit
```

**根因**：`status.rs` 使用 `{:<16}` 按字节填充，中文字符占 2 个显示宽度但 Rust 按字节计，导致错位。

**修复**：`crates/hit-cli/src/commands/status.rs` — 引入 `display_width()` 函数按 Unicode 显示宽度（CJK 全角字符占 2 列）计算，动态对齐标签列。

**提交**：`e79afb0` — fix(cli): status和bucket输出按Unicode宽度对齐;bucket add未知bucket提示用法示例

---

## 欢迎页面仍未触发 ⭐（2026-06-27 解决）

上一次修复（`94c6d41`）将欢迎检查移到 `Cli::parse()` 之前，但 `is_first_run()` 仍以 `config.json` 是否存在为判据。安装脚本会预先写好默认 config，导致 `is_first_run()` 永远返回 false，欢迎页彻底无法触发（即使 `hit bucket list` 也不显示）。

**根因**：`is_first_run()` 判据错误——config.json 存在 ≠ 用户完成首次引导。

**修复**：`crates/hit-cli/src/welcome.rs` — 改用"bucket 目录是否为空"作为首次运行判据。安装脚本只预置 config，不预置 bucket，因此 buckets 为空时正确触发引导。

**提交**：`f967a07` — fix(welcome): 改用bucket目录是否为空判断首次运行,避免被预置config误判

---

## Manifest 兼容性缺陷：大量真实 Scoop manifest 解析失败 ⭐⭐⭐⭐⭐（2026-06-27 解决）

对官方 bucket 执行 `hit search`/`hit info`/`hit install` 时，stderr 喷出 ~1500 行 WARN，主流软件（chromium/firefox/vscode/calibre/perl 等）manifest 全部被跳过，违反"Scoop 兼容性"首要约束。

**根因（三类解析失败）**：
1. **`HashField` 不支持对象形式** — Scoop 的 `hash` 字段在 `autoupdate` 中允许 `{url, jsonpath}`/`{url, regex}` 对象，Hit 仅支持 `String`/`Vec<String>`。
2. **`suggest` 字段类型错误** — 声明为 `BTreeMap<String, String>`，但 Scoop 中值是字符串数组（如 `"JDK": ["java/opendk", "java/oraclejdk"]`）。
3. **`CheckverField.script` 不支持单字符串** — 声明为 `Vec<String>`，但 Scoop 允许单字符串形式。

**修复**：
- `crates/hit-core/src/manifest/schema.rs` — `HashField` 新增 `Fetch{url, regex?, jsonpath?, xpath?}` 变体；`suggest` 改为 `BTreeMap<String, OneOrMany<String>>`；`Checkver.script` 改为 `ScriptField` 类型。
- `crates/hit-core/src/manifest/validator.rs` — 适配 `suggest` 新类型，逐项校验。
- `crates/hit-core/src/manifest/variables.rs` — 适配 `ScriptField` 和 `HashField::Fetch` 的变量替换。
- `crates/hit-core/tests/manifest_test.rs` — 新增 5 个回归测试（`regression_perl_hash_fetch_jsonpath`、`regression_hash_fetch_top_level`、`regression_suggest_array_value`、`regression_suggest_single_string_value`、`regression_checkver_script_single_string`）。

**验证**：26 个 manifest 测试全部通过；`hit search git` 输出 0 WARN。

**提交**：`cf20905` — fix(manifest): HashField支持Fetch对象;suggest改Vec;checkver.script支持单字符串

---

## Hit 本身的 Shim 异常 ⭐⭐⭐（2026-06-27 解决）

`shims\hit.exe` 大小 10873 KB，与根目录 `hit.exe` 相同。应为轻量代理（~200KB）+ sidecar，而非完整程序副本。

**根因**：`scripts/install-hit.ps1` 第 246 行直接 `Copy-Item $exeSource → shims\hit.exe`，把完整 hit.exe（11MB）当作 shim 用，违反 shim 代理设计。

**修复**：
- `scripts/install-hit.ps1` — 网络下载模式同时下载 `hit-shim.exe`；本地模式从 `hit.exe` 同目录查找 `hit-shim.exe`。部署阶段：完整 `hit.exe` 放根目录，轻量 `hit-shim.exe`（214KB）放 `shims\`，并生成 `hit.shim` sidecar 指向真实 exe。

**验证**：重装后 `shims\hit.exe` = 214,528 字节，`shims\hit.shim` = 53 字节（含 `path = "..."` 指向根目录 hit.exe）。

**提交**：`69f2856` — fix(install): shim目录改用轻量hit-shim.exe代理+sidecar,不再复制完整hit.exe

---

## 设计问题：`bucket add unknownbucket` 提示不友好 ⭐（2026-06-27 解决）

`hit bucket add unknownbucket`（未知名称且无 URL）报错"未知 bucket '...'，请提供 Git 仓库 URL"，但未告知用户具体如何操作。

**修复**：`crates/hit-cli/src/commands/bucket.rs` — 错误信息补充用法示例：`示例：hit bucket add <name> https://github.com/<user>/<bucket>.git`。

**提交**：`e79afb0` — fix(cli): status和bucket输出按Unicode宽度对齐;bucket add未知bucket提示用法示例

---

## 格式问题：所有输出未对齐 ⭐（2026-06-27 解决）

所有命令输出因中文字符宽度计算不准导致错位。

**修复**：`crates/hit-cli/src/commands/status.rs` 和 `bucket.rs` 引入 `display_width()`/`pad()` 辅助函数，按 Unicode 显示宽度对齐。

**提交**：`e79afb0` — fix(cli): status和bucket输出按Unicode宽度对齐;bucket add未知bucket提示用法示例

---

## Welcome 引导错误触发 + clap 错误被 welcome 吞掉 ⭐⭐⭐⭐⭐（2026-06-27 解决）

两个同根同源的五星 bug：welcome 在已安装环境仍触发污染所有命令输出；`hit`/`hit wrongcmd`/`hit install`（无参数）的 clap 错误被 welcome 菜单吞掉。

**根因（双重）**：
1. **welcome 时机错误** — `main.rs` 的 `run()` 在 `Cli::parse()` **之前**调用 `welcome::is_first_run()`，导致 clap 还来不及报错就被 welcome 拦截。
2. **`is_first_run()` 判据错误** — 上一版（`f967a07`）改为仅以 buckets 目录为空判断，但 `paths::buckets_path()` 依赖 `HIT_ROOT` 环境变量解析根目录；通过 shim 调用时若 `current_exe()` 回退到错误路径，buckets 判据会误判为首次运行。

**修复**：
- `crates/hit-cli/src/main.rs` — welcome 检查移到 `Cli::parse()` **之后**，让 clap 先处理 `--help`/无子命令/错误命令/缺参数，直接报错退出不被 welcome 拦截。
- `crates/hit-cli/src/welcome.rs` — `is_first_run()` 改为**双条件**（必须同时满足）：config.json 不存在 **且** buckets 目录为空。已安装环境（config 在）绝不触发。判据改用 `paths::root_path()`（基于 HIT_ROOT/SCOOP/USERPROFILE 回退链）而非 `current_exe()` 同目录，避免 shim 调用误判。

**验证**：hit-cli 73 个单元测试全部通过；`cargo check` 通过。

**说明**：此修复同时是五星 bug "hit install 完全不工作" 和 "hit info 完全不工作" 的潜在根因——welcome 在 parse 前执行污染 stdout（横幅+菜单+"无效选择，已跳过"喷到 stdout），把 install/info 的真实业务输出冲掉。修复后这两个 bug 应随之恢复，待 release 构建后跑 REPORT.md 回归验证。
