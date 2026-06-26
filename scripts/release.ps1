#Requires -Version 5
<#
.SYNOPSIS
    本地发布脚本：编译 release 并复制 hit.exe 到 scripts/ 目录。

.DESCRIPTION
    执行 cargo build --release，然后将产物 hit.exe 复制到 scripts/ 下，
    方便后续手动 git tag + push + 创建 GitHub Release。

    用法：
        .\scripts\release.ps1 --version 1.0.0

.PARAMETER Version
    版本号（如 1.0.0），必填。
#>

param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)

$ErrorActionPreference = 'Stop'

# ── 验证版本号格式 ─────────────────────────────────────────────────────

if ($Version -notmatch '^\d+\.\d+\.\d+') {
    Write-Fail "版本号格式无效：$Version（期望 x.y.z）"
}

$tag = "v$Version"

# ── 编译 ──────────────────────────────────────────────────────────────

Write-Step "编译 release 产物..."
Set-Location (Split-Path $MyInvocation.MyCommand.Path -Parent)
Set-Location ..

cargo build --release -p hit-cli -p hit-shim
if ($LASTEXITCODE -ne 0) {
    Write-Fail "cargo build 失败"
}

$source = Join-Path $PWD "target\release\hit.exe"
if (-not (Test-Path $source)) {
    Write-Fail "编译产物不存在：$source"
}

# ── 复制到 scripts/ ───────────────────────────────────────────────────

$scriptsDir = Split-Path $MyInvocation.MyCommand.Path
$dest = Join-Path $scriptsDir "hit.exe"

Write-Step "复制 hit.exe → scripts\hit.exe"
Copy-Item -Path $source -Destination $dest -Force

$srcHash = (Get-FileHash $source -Algorithm SHA256).Hash
$dstHash = (Get-FileHash $dest -Algorithm SHA256).Hash
if ($srcHash -ne $dstHash) {
    Write-Fail "复制后哈希不一致"
}
Write-Ok "scripts\hit.exe 已更新（SHA256: $srcHash）"

# ── 完成 ──────────────────────────────────────────────────────────────

Write-Host ""
Write-Ok "发布准备完成！"
Write-Host ""
Write-Host "    版本  : $tag"
Write-Host "    产物  : $dest"
Write-Host ""
Write-Host "    接下来请手动执行："
Write-Host ""
Write-Host "        git add -A"
Write-Host "        git commit -m 'release: v$Version'"
Write-Host "        git tag $tag"
Write-Host "        git push origin main $tag"
Write-Host "        # 然后在 GitHub 网页创建 Release，选择 tag $tag，上传 zip"
Write-Host ""
Write-Host "    或使用 GitHub CLI："
Write-Host ""
Write-Host "        gh release create $tag --title 'Hit $tag' target/release/hit-*.zip"
Write-Host ""
