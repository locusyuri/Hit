# EUREKA — 灵感速记

> 用于随手记录突发灵感与点子。每条只写**一句话核心想法**，详细的功能设计、方案论证、文档联动等后续再展开到对应文档（PROJECT.md / TODO.md 等）。
>
> 格式建议：`- **关键词** — 一句话描述（可选：日期 / 触发场景）`

---

## 灵感池

### 已收录

- **i18n 国际化** — CLI 输出与错误信息按系统 locale 切换语言（中/英优先），manifest 描述字段也支持 `_zh` / `_en` 多语言 fallback。
- **代管其它包管理器** — hit list / upgrade / uninstall 等命令可列出并操作 bun、npm、pnpm、uv 等包管理器全局安装的包。通过子进程调用对应包管理器命令（如 `npm ls -g --json`）并解析 stdout 实现。由于各包管理器执行速度不确定，需使用异步/spawn 超时控制等方式防止拖慢 hit 自身响应。
- **TOML 配置 + TUI 修改** — 使用 TOML 作为唯一配置源，每项配置带注释方便用户直接编辑；使用 `toml_edit` 库保留注释和格式；CLI 也提供 TUI 界面修改配置，修改结果写回 TOML。
- ~~**开箱即用（零前置依赖）** — Scoop 需要先装 git 和 7zip，Hit 用 Rust 的 `git2` 和 `zip`/`sevenz-rust` 等库直接实现所有功能，用户下载即用，无需预装任何工具。~~ ✅ **已实现**
- **持久化搜索索引** — Scoop 搜索慢的原因是遍历 bucket 的每个 JSON。可用 `sonic_rs` 高性能解析 + 并行遍历 + 轻量嵌入式 KV 数据库（如 `redb`）构建倒排索引。索引可选是否启用，首次构建后增量更新。
- ~~**Bucket 浅克隆** — Scoop bucket 是 git 仓库，大量提交导致体积膨胀。使用 `depth=1` 浅克隆，仅保留最新 manifest。需要旧版本时逐步加深拉取。用户可手动为指定 bucket 开启全量模式。~~ ✅ **已列入 [PROJECT.md](./PROJECT.md)**
- ~~**高频旧版本专用仓库** — 对 maven、gradle、nodejs、python 等频繁下载旧版本的环境类软件可自建 GitHub 仓库托管历史版本清单，或复用 Scoop 的 `versions` bucket。~~ ❌ **不建议实施**（Scoop 已有 `versions` bucket）
- ~~**首次启动快速导入** — 初次运行 `hit` 时提供交互式引导，一键导入 `main`、`extras`、`versions` 等常用 bucket。~~ ✅ **已列入 [PROJECT.md](./PROJECT.md)**
- ~~**快速创建 bucket 命令** — 新增 `hit bucket create` 命令，交互式引导用户创建自己的 bucket 仓库（初始化目录结构、生成 bucket.json、提示推送到 GitHub 等）。~~ ✅ **已列入 [PROJECT.md](./PROJECT.md)**
- ~~**卸载 Hit 自身** — 通过 PowerShell 脚本实现两种卸载模式，与安装脚本 `install-hit.ps1` 对称：`uninstall-env.ps1`（模式1：只清理环境变量，保留已安装软件）和 `uninstall-hit.ps1`（模式2：彻底删除全部内容）。无需 CLI 子命令。~~ ✅ **已实现（脚本方案）**
- ~~**tabled 美化 CLI 输出** — 将 `search`、`list`、`cache list`、`bucket list` 等命令的手写文本表格改为 `tabled` crate 自动渲染的表格，列宽自适应、对齐整齐、无需手写格式化逻辑，比 `println!` 拼接更美观且零维护成本。~~ ✅ **已实现**
- **交互式搜索安装（`hit si`）** — 基于 TUI 框架（如 ratatui）的交互式搜索安装命令，在所有已注册 Bucket 中搜索匹配软件包，通过表格界面展示（名称/版本/来源/可执行文件），支持上下箭头选择、Enter 安装、Esc 取消、同名多 Bucket 来源选择。原 ratatui 实现已移除（对简单命令过重），待 TUI 框架选型稳定后重新实现。

---

## 评审意见

以下是对灵感池中所有灵感的可行性分析与建议。

### ✅ 可直接实施（低风险，高价值）

| # | 灵感 | 建议 | 状态 |
|---|------|------|:----:|
| 1 | **开箱即用** | ✅ 已实现。`Cargo.toml` 已依赖 `gix`、`zip`、`sevenz-rust2` 等，用户下载即用。 | ✅ **已实现** |
| 2 | **Bucket 浅克隆** | 已采纳为正式功能。默认 `depth=1`，用户通过 `hit bucket config <name> --full-clone` 切换。 | ✅ **已列入 [PROJECT.md](./PROJECT.md)** |
| 3 | **首次启动快速导入** | 已采纳为正式功能。检测 `config.json` 不存在时显示欢迎引导。 | ✅ **已列入 [PROJECT.md](./PROJECT.md)** |
| 4 | **快速创建 bucket** | 已采纳为正式功能。`hit bucket create` 交互式创建，可配合 `gh` CLI 推送。 | ✅ **已列入 [PROJECT.md](./PROJECT.md)** |

### 🟡 需进一步设计

| # | 灵感 | 分析 & 建议 |
|---|------|------------|
| 5 | **i18n 国际化** | 低优先级，属于"有更好没有也不影响核心功能"的增强。**建议**：Phase 1-3 不考虑，等 CLI 输出稳定后再做。翻译文件的维护成本高于实现成本。 |
| 6 | **代管其它包管理器** | 需要先确定支持的包管理器范围和优先级（npm/pnpm/bun/uv/...），每个的实现方式和输出格式都不同，解析成本较高。**建议**：做成插件化的"外部包管理器适配器"，每个管理器一个 adapter，通过 SPI 注册。`hit list` 统一调用所有 adapter 收集结果，超时控制每个 adapter 不超过 3 秒。适合 Phase 4+。 |
| 7 | **TOML 配置** | 当前 config 已是 JSON + `sonic-rs`，切换到 TOML 涉及到迁移。`toml_edit` 保留注释的优势仅在用户手动编辑时体现——如果主要修改途径是 TUI/CLI，普通 toml crate 就够了。**建议**：Phase 1 先用 JSON，等配置结构稳定后再迁移。 |
| 8 | **持久化搜索索引** | 思路正确，Scoop 慢的根因就是每次遍历所有 JSON。但需明确：索引格式用 `redb`（KV）做 `name → [bucket, version, desc]` 映射足够，不需要倒排索引，关键词搜索走 `LIKE` 扫描 KV。**建议**：默认启用而非可选，`hit bucket update` 后自动增量更新。备选方案可用 SQLite（已有 rusqlite 经验）。适合 Phase 2-3。 |
| 9 | **高频旧版本专用仓库** | Scoop 已有 `versions` bucket，**不建议自建**。在首次启动导入时默认添加 `versions` bucket 即可，维护成本远低于自建。 |

---

## 新增灵感评审

### ✅ **卸载 Hit 自身**（2026-06-26 新增，2026-06-28 已实现）

**最终方案**：通过 PowerShell 脚本实现，与安装脚本 `install-hit.ps1` 对称，无需 CLI 子命令。

| 脚本 | 模式 | 行为 |
|------|------|------|
| `scripts\uninstall-env.ps1` | 模式1：只清理环境变量 | 删除 `HIT_ROOT` 环境变量 + 从 PATH 移除 Hit 根目录和 shims 目录，**不删除文件和已安装软件** |
| `scripts\uninstall-hit.ps1` | 模式2：彻底卸载 | 删除 `<root>/` 整个目录（含 apps/shims/cache/buckets/config.json）+ 清理环境变量，需两次确认 |

**设计理由**：
- 安装和卸载对称：安装用 `install-hit.ps1`，卸载用 `uninstall-hit.ps1`，用户体验一致
- 脚本方案避免运行中的进程无法删除自身二进制的问题（脚本运行在独立 PowerShell 进程中）
- 安全措施：路径合法性验证（必须包含 `hit`）、不允许删除用户主目录或驱动器根目录、PATH 清理采用白名单匹配

---

### ✅ **ratatui 美化 CLI 输出**（2026-06-27 新增，2026-07-09 已实现 tabled 方案）

**灵感来源**：当前 `search`、`list`、`info` 等命令的输出是简单的文本表格（`println!`），缺少列宽自适应、高亮、滚动等交互能力，视觉上不够美观。

**最终方案**：使用 `tabled` crate 替代 ratatui，为 search/list/cache/bucket 命令提供自动表格渲染。

| 考虑方案 | 说明 | 评价 |
|---------|------|:----:|
| ratatui 全屏渲染 | 进入 TUI 全屏模式显示表格，用户按 `q` 或 `Esc` 返回 | ❌ 已移除。对简单的 list/search 太重，且 si 命令已删除 |
| ratatui 内联输出 | 用 ratatui 仅渲染表格部分（不进入全屏），在终端当前行下方绘制表格 | ❌ ratatui 本质是全屏框架，不适合此用途 |
| `tabled` crate | 保持文本行输出，但用 `tabled` 自动计算列宽、对齐、分区线 | ✅ **已采用**。零侵入，直接从 `Vec<struct>` 输出格式化的表格 |

**实施结果**：

1. **移除 ratatui** — 删除 `tui.rs` 和 `si` 命令，移除 ratatui/crossterm/comfy-table 依赖
2. **新增 `tables.rs`** — 使用 `#[derive(Tabled, Clone)]` 声明式定义 4 种表格行类型（SearchRow、ListRow、CacheRow、BucketRow）
3. **统一表格输出** — search/list/cache/bucket 命令统一调用 `tables::print_*_table()` 函数渲染表格
4. **列宽自适应** — tabled 自动计算列宽、对齐、分区线，效果比手写 `println!` 好得多

**现有依赖**：`tabled = "0.16"`（在 workspace Cargo.toml 中声明）。