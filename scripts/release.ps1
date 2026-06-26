#Requires -Version 5
<#
.SYNOPSIS
    本地发布脚本：编译 release 并复制 hit.exe 到 scripts/ 目录。

.DESCRIPTION
    执行 cargo build --release，然后将产物 hit.exe 复制到 scripts/ 下，
    方便后续手动上传到远程 Release。

    用法：
        .\scripts\release.ps1
#>

$ErrorActionPreference = 'Stop'

function Write-Step { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Cyan; Write-Host $msg }
function Write-Ok   { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Green; Write-Host $msg }
function Write-Fail { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Red; Write-Host "ERROR: $msg"; exit 1 }

# ── 编译 ──────────────────────────────────────────────────────────────

Write-Step "编译 release 产物..."
$repoRoot = Split-Path (Split-Path $MyInvocation.MyCommand.Path -Parent) -Parent
Set-Location $repoRoot

cargo build --release -p hit-cli
if ($LASTEXITCODE -ne 0) {
    Write-Fail "cargo build 失败"
}

$source = Join-Path $repoRoot "target\release\hit.exe"
if (-not (Test-Path $source)) {
    Write-Fail "编译产物不存在：$source"
}

# ── 复制到 scripts/ ───────────────────────────────────────────────────

$dest = Join-Path (Split-Path $MyInvocation.MyCommand.Path) "hit.exe"

Write-Step "复制 hit.exe → scripts\hit.exe"
Copy-Item -Path $source -Destination $dest -Force

# 校验哈希一致（用 .NET SHA256，兼容 PowerShell 5.1）
$srcHash = ([System.Security.Cryptography.SHA256]::Create().ComputeHash([System.IO.File]::ReadAllBytes($source)) | ForEach-Object { $_.ToString("x2") }) -join ''
$dstHash = ([System.Security.Cryptography.SHA256]::Create().ComputeHash([System.IO.File]::ReadAllBytes($dest)) | ForEach-Object { $_.ToString("x2") }) -join ''
if ($srcHash -ne $dstHash) {
    Write-Fail "复制后哈希不一致"
}
Write-Ok "scripts\hit.exe 已更新（SHA256: $srcHash）"

# ── 完成 ──────────────────────────────────────────────────────────────

Write-Host ""
Write-Ok "发布准备完成！"
Write-Host ""
Write-Host "    产物  : $dest"
Write-Host ""
Write-Host "    接下来请手动上传到 GitHub/CNB Release。"
Write-Host ""
