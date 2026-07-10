# 【学习工作赛道】Hit —— 基于 Rust 的现代 Windows 软件包管理器，完全兼容 Scoop 生态

## 0. 先和大家打个招呼吧 👋

* **我是谁：** 
  我是 locusyuri（在 CNB 云原生平台上也是 @catmono 的主要维护者之一），一名热爱系统级工具开发的独立开发者。

* **我是怎么用 TRAE 把 Demo 做出来的：**
  作为日常高频使用 Windows 进行开发的技术人员，配置环境、安装依赖和管理各种软件版本是一件极其琐碎却高频的事。之前用 Scoop 等包管理器时，总是遇到 PowerShell 脚本冷启动慢、PATH 容易被污染、版本锁定和多版本切换繁琐等痛点。于是，我萌生了用 Rust 重写一个包管理器的想法，这便是 **Hit**。
  
  在这段开发历程中，**TRAE** 扮演了我无可替代的“结对编程专家”。虽然我有清晰的架构思路，但要从零编写一个高效且零外部依赖的 Windows 运行时代理（Shim），并实现流畅的交互式 TUI 搜索、多版本切换和自修复功能，工作量极其庞大。

  我通过 **TRAE 的 Auto 模式**，将我脑子里的需求模块“一句句”讲给它听。例如：
  - “我需要用 Rust 实现一个极度精简、零依赖的 Windows `.shim` 代理，它需要读取 sidecar 配置并极速转发进程。”
  - “帮我优化 `hit si` 命令的 TUI 交互体验。我们原本使用 `ratatui` 实现，但在 Windows 控制台下显得有些笨重，我们能不能换成 `dialoguer` 或者使用 `tabled` 重构表格渲染？”
  
  TRAE 不仅能完美理解我的意图，而且每一次的代码重构（比如从 `ratatui` 迁移到 `tabled` 优化表格显示）都准而快。它帮我跨过了最枯燥的底层系统调用封装（如 Windows Junction/Symlink 链接修复）以及大量自动化测试脚本编写的坎，让我能把全部精力集中在架构设计、交互逻辑和核心算法上。与 TRAE 配合的感受就是：**灵感能以 10 倍速落地，编程的摩擦感彻底消失了！**

---

## 1. Demo 简介

* **是什么：**
  **Hit** 是一个用 Rust 编写的现代 Windows 软件包管理器，完全兼容已有的 Scoop bucket/manifest 生态。

* **面向谁：**
  所有在 Windows 上进行开发、对命令行效率有极致追求、希望拥有一致且零污染开发环境的程序员与技术爱好者。

* **主要功能：**
  1. **零依赖极速 Shim 代理 (`hit-shim`)**：通过轻量级代理程序（仅约 200KB）实现 PATH 整洁。不写注册表，卸载即自动删除，不留任何系统垃圾。
     ![Hit 命令行截图1](https://locus622.oss-cn-beijing.aliyuncs.com/material/hit1.png?x-oss-credential=LTAI5tEEsQCmu7iQx23XbFi9%2F20260710%2Fcn-beijing%2Foss%2Faliyun_v4_request&x-oss-date=20260710T132935Z&x-oss-expires=604800&x-oss-signature=28ba38794dccec53ef3a4b59156643ae13aef4ad4661c76b9e3173cfa1c6d4f8&x-oss-signature-version=OSS4-HMAC-SHA256)

  2. **版本切换与锁定 (`hit reset` / `hit hold`)**：原生支持同一软件的多版本共存和自由切换，并可通过锁（Hold）防止高频更新时破坏工作流环境。

  3. **健康检查与自动修复 (`hit doctor`)**：全盘扫描安装完整性，并自动修复损坏的 Junction/Shim 链接。

  4. **表格美化渲染**：使用 `tabled` 为 search/list/cache/bucket 命令提供美观的表格输出，表头青色粗体高亮，数据清晰易读。
     ![Hit 命令行截图2](https://locus622.oss-cn-beijing.aliyuncs.com/material/hit2.png?x-oss-credential=LTAI5tEEsQCmu7iQx23XbFi9%2F20260710%2Fcn-beijing%2Foss%2Faliyun_v4_request&x-oss-date=20260710T132935Z&x-oss-expires=604800&x-oss-signature=715c7078df9c0da18ddcd458225210325bd8999608d02e6ffbc3358d87962b09&x-oss-signature-version=OSS4-HMAC-SHA256)

  5. **统一色彩输出**：使用 `owo-colors` 实现语义化彩色输出——成功绿色、错误红色、警告黄色、步骤青色，提升命令行交互体验。

---

## 2. Demo 创作思路

* **灵感来源：**
  Scoop 在 Windows 开发者圈子非常流行，但其基于 PowerShell 脚本的实现导致：
  - 冷启动较慢（尤其在一些低配虚拟机或 CI 容器里）；
  - 搜索和多 bucket 同步体验不够平滑；
  - 交互不够直观，无法做到类似 macOS `brew` 般的现代极简体验。
  由于 Rust 天生具有极强的无运行时、高并发、跨平台和极小体积优势，因此用 Rust 实现 Scoop 兼容版客户端的想法应运而生。

* **想解决的问题：**
  - **PATH 严重污染**：很多 Windows 软件安装后将整个目录塞进系统 Path，极易引发冲突。Hit 采用 `.shim` sidecar 代理模式，只暴露一个极小入口。
  - **无管理员权限安装**：所有软件解压至用户目录（`~/.hit`），无需 UAC 授权，真正做到便携化（删目录即完全卸载）。
  - **环境损坏难定位**：由于手动操作或杀毒软件导致的符号链接失效。`hit doctor` 提供一键健康诊断并自动重写 Shim 代理修复。

* **为什么做这个方向（取舍与判断）：**
  我们没有选择自己再去造一套全新的软件包“软件源（Manifest）”轮子，而是选择**完全兼容已有的 Scoop 庞大生态**。这让 Hit 诞生第一天就能无缝安装、更新数万款成熟的开发者工具。我们把精力放在“重写极速客户端”上，用 Rust 实现了零外部引用的 `hit-shim` 代理二进制，不仅轻量（~200KB），更是将启动响应做到了极致。

---

## 3. Demo 体验与源码地址

为了方便国内与海外开发者，我们同时维护了两个平台上的远程仓库，国内访问极速畅通：

* **主源码仓库（GitHub）：**
  👉 [locusyuri/Hit (GitHub)](https://github.com/locusyuri/Hit)

* **国内极速访问镜像 & 云原生构建（CNB）：**
  👉 [catmono/Hit (CNB · Cloud Native Build)](https://cnb.cool/catmono/Hit)
  *(我们在 CNB 上集成了自动化 CI/CD，虽然目前的 pipeline 因特定配置正在调试，但完全不影响国内用户直接在此流畅拉取最新 Rust 源码构建体验。)*

---

## 4. TRAE 实践过程

我们使用 Cargo workspace 组织项目，保证了各个独立模块的强关注点分离。在开发过程中，我们通过 **TRAE** 对各核心组件进行了深度演进。

### 开发过程截图

#### 1. TRAE 协助实现表格渲染模块
![TRAE 开发截图1](https://locus622.oss-cn-beijing.aliyuncs.com/material/trae1.png?x-oss-credential=LTAI5tEEsQCmu7iQx23XbFi9%2F20260710%2Fcn-beijing%2Foss%2Faliyun_v4_request&x-oss-date=20260710T132935Z&x-oss-expires=604800&x-oss-signature=c0164719dcb6e84e21f435bd12f6108319df97efa008e386187ea20a1dc2933b&x-oss-signature-version=OSS4-HMAC-SHA256)

#### 2. TRAE 协助修复 Windows Junction 链接问题
![TRAE 开发截图2](https://locus622.oss-cn-beijing.aliyuncs.com/material/trae2.png?x-oss-credential=LTAI5tEEsQCmu7iQx23XbFi9%2F20260710%2Fcn-beijing%2Foss%2Faliyun_v4_request&x-oss-date=20260710T132935Z&x-oss-expires=604800&x-oss-signature=98d9487b5b38b114d3bf21485a5cb09cc2f8c8c1fa98a8d8149a04b11813d3e6&x-oss-signature-version=OSS4-HMAC-SHA256)

#### 3. TRAE 协助编写自动化测试用例
![TRAE 开发截图3](https://locus622.oss-cn-beijing.aliyuncs.com/material/trae3.png?x-oss-credential=LTAI5tEEsQCmu7iQx23XbFi9%2F20260710%2Fcn-beijing%2Foss%2Faliyun_v4_request&x-oss-date=20260710T132935Z&x-oss-expires=604800&x-oss-signature=89a32ccdeb2ad12ed0eb743bb50efeeccc9201ac50124f6fed4c6932b3a13a5e&x-oss-signature-version=OSS4-HMAC-SHA256)

### 关键步骤与 Session ID 记录：

#### 1. 🌟 功能开发（Feature Development）
- **核心任务**：使用 `tabled` 库替换 `ratatui` 实现超轻量、美观的命令行表格渲染，为 search/list/cache/bucket 命令提供统一的表格输出。
- **关键 Session ID**：`6a50c984cbc8cdf245e171ee`

#### 2. ⚡ Bug 修复（Bug Fixing）
- **核心任务**：修复 Windows 环境下特定的 Junction/Symlink 创建异常问题，优化 `hit-shim` 代理转发时参数传递的解析鲁棒性。
- **关键 Session ID**：`6a42153cf456dcd062a16dd2`

#### 3. 🧪 自动化测试（Testing & Validation）
- **核心任务**：编写跨平台集成测试用例，覆盖 `git` / `7zip` 模拟安装生命周期；解决 CI 容器中 junction 临时路径无法写入的边界测试。
- **关键 Session ID**：`6a3fe5b65c3cfae8ea13d862`

---

## 5. 当前项目状态说明

> **注意：** 本次提交为初赛 Demo 版本，项目仍在持续开发中，部分功能尚未完善，后续会逐步优化。

### ✅ 已实现功能
- ✅ 基础命令：`hit install` / `hit uninstall` / `hit list` / `hit search` / `hit info` / `hit update`
- ✅ Bucket 管理：`hit bucket add/list/remove/update`
- ✅ 版本管理：`hit reset`（版本切换）/ `hit hold`（版本锁定）/ `hit unhold`（解除锁定）
- ✅ 配置管理：`hit config` / `hit doctor`（健康检查与自动修复）
- ✅ 缓存管理：`hit cache` / `hit cleanup`
- ✅ 辅助命令：`hit home` / `hit which` / `hit prefix`
- ✅ 表格渲染：`tabled` 自动表格输出（search/list/cache/bucket）
- ✅ 统一色彩输出：`owo-colors` 语义化彩色日志
- ✅ 进度条：`indicatif` 下载/解压进度
- ✅ 首次启动引导：`hit welcome`

### 🔄 待实现功能
- 🔄 交互式搜索（`hit si`）：原 `ratatui` 版本已移除，计划使用 `dialoguer` 重新实现
- 🔄 交互式安装/卸载选择：批量操作时的交互式确认

### ⚠️ 已知问题
- ⚠ 部分集成测试用例仍在调试中（如 `uninstall_without_junction_fails_gracefully`）
- ⚠ 可能存在未发现的边界 case 和兼容性问题

### 🚀 未来计划
- Phase 4：SDK 多版本管理、深度卸载、软件束 (Bundle)、沙盒环境
- Phase 5：插件系统、配置同步、增量更新、跨平台支持（Linux/macOS）

---

## 6. 对应的报名审核通过的帖子链接

*(帖子链接：待补充 —— 审核通过后我会立即更新在这里)*

---

欢迎大家试用、拍砖或贡献代码！让我们一起把 Windows 下的环境管理变得像写 Rust 一样优雅和安心！🦀