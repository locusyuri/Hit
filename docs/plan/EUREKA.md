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