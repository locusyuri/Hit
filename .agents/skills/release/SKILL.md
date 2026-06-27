---
name: release
description: >
  Hit 项目发布 skill。构建 release 资产，
  根据用户提示生成草稿内容，等待用户审核后执行发布命令
  （git tag / push / gh release create / git cnb release create）。
license: MIT
compatibility: Hit 项目
metadata:
  version: "1.0.0"
allowed-tools: Bash(cargo:*) Bash(git:*) Read Write Edit Glob Grep
user_invocable: true
disable_model_invocation: false
---

# Release — Hit 项目发布到远程仓库（GitHub + CNB）

构建 Windows 二进制包，生成 release 草稿，待用户审核后执行发布命令。

---

## 触发条件

当用户提到以下关键词时触发：
- `release` / `发布` / `打 tag`
- `推到远程` / `创建 release`
- `打包发布`

---

## 前置检查

在开始之前，先确认环境：

```powershell
# 1. gh CLI 已安装并登录
gh auth status
# 期望：✓ Logged in to github.com as <user>

# 2. git-cnb 已安装
git cnb --version
# 期望：git-cnb version x.x.x

# 3. 当前分支是 main（或用户指定的分支名）
git branch --show-current
```

如果 `gh auth status` 报错，提示用户先运行 `gh auth login`。
如果 `git cnb --version` 报错，提示用户先安装 git-cnb。

---

## 发布流程

### Step 1：确认版本号

询问用户版本号（如 `1.0.0`），或用以下命令推断：

```powershell
# 从 Cargo.toml 读取当前版本
$ver = Select-String -Path "Cargo.toml" -Pattern '^version = "(.+?)"' | ForEach-Object { $_.Matches.Groups[1].Value }
Write-Host "当前 workspace 版本：$ver"
```

用户确认版本号后，设置 `$Version` 和 `$Tag = "v$Version"`。

### Step 2：构建 Release 资产

直接上传 `hit.exe` + `hit-shim.exe`（不再打 zip）。install-hit.ps1 的网络下载模式会同时下载这两个文件（URL 分别为 `$baseUrl/.../hit.exe` 与 `$baseUrl/.../hit-shim.exe`），故两者都必须上传。

```powershell
cargo build --release -p hit-cli -p hit-shim
if ($LASTEXITCODE -ne 0) { Write-Fail "cargo build 失败" }

$assets = @('hit.exe', 'hit-shim.exe')
foreach ($a in $assets) {
    $src = "target\release\$a"
    if (-not (Test-Path $src)) { Write-Fail "产物不存在：$src" }
    Write-Host "产物：$src"
}
```

### Step 3：生成草稿

根据以下模板生成草稿内容，写入 `draft_release.md`，展示给用户审核：

```markdown
# Release 草稿 — Hit v${Version}

## 标题
Hit v${Version}

## 自动生成的 Release Notes
<!-- 占位，gh release create --generate-notes 会自动填充 -->

## 变更摘要
<!-- 从 git log 提取自上一个 tag 以来的关键提交 -->
$(git log $(git describe --tags --abbrev=0 2>/dev/null || echo "HEAD~10")..HEAD --oneline --no-merges | ForEach-Object { "- $_" })

## 包含的资产
- hit.exe（x86_64-pc-windows-msvc，主程序）
- hit-shim.exe（x86_64-pc-windows-msvc，轻量 shim 代理，~200KB）

## 发布操作
将执行以下命令：

```powershell
# 1. 提交构建产物
git add -A
git commit -m "release: v${Version}"

# 2. 打 tag
git tag ${Tag}

# 3. 推送
git push origin main ${Tag}

# 4. 创建 GitHub Release（上传两个资产）
gh release create ${Tag} hit.exe hit-shim.exe --title "Hit v${Tag}" --generate-notes

# 5. 创建 CNB Release（分别上传两个资产）
git cnb release create ${Tag}
git cnb release upload ${Tag} hit.exe
git cnb release upload ${Tag} hit-shim.exe
```

文件写入 `draft_release.md` 后，展示给用户审核。告知用户：
- 可以编辑任何内容（标题、说明、操作列表）
- 回复"执行"/"确认"/"ok"后执行发布
- 回复"取消"则终止，不改变任何 git 状态

### Step 4：用户审核后执行

**用户确认后**，按以下顺序执行。任一命令失败时立即停止并报告：

```powershell
# 1. 确保在正确分支
$branch = git branch --show-current
if ($branch -ne 'main') {
    Write-Warn "当前不在 main 分支（当前：$branch），继续？"
}

# 2. 提交 + tag + push
git add -A
git commit -m "release: v${Version}"
git tag ${Tag}
git push origin main ${Tag}

# 3. GitHub Release（上传 hit.exe + hit-shim.exe）
gh release create ${Tag} hit.exe hit-shim.exe --title "Hit v${Tag}" --generate-notes

# 4. CNB Release（分别上传两个资产）
git cnb release create ${Tag}
git cnb release upload ${Tag} hit.exe
git cnb release upload ${Tag} hit-shim.exe
```

### Step 5：清理

```powershell
# 删除本地 exe（构建产物已在 target/release/，无需保留副本）
# 注：不再复制到仓库根，故无需清理根目录文件
Write-Host "发布完成！GitHub: https://github.com/locusyuri/Hit/releases/tag/${Tag}"
Write-Host "CNB: https://cnb.cool/catmono/Hit/-/releases"
```

---

## 关键注意

1. **绝不提前 push** — 必须等用户确认后才执行任何写远程的操作
2. **草稿留痕** — `draft_release.md` 保留在项目根目录，之后可手动或自动删除
3. **CNB_TOKEN** — `git cnb release` 需要 `CNB_TOKEN` 环境变量；CI 中自动注入，本地需用户自备
4. **幂等性** — 如果 tag 已存在，告知用户，不要重复 push
5. **Cargo.toml bump（可选）** — 发布后询问是否自动 bump 下一个 patch 版本
