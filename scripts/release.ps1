#Requires -Version 5
<#
.SYNOPSIS
    本地发布脚本：编译 release 并复制 hit.exe + hit-shim.exe 到 scripts/ 目录。

.DESCRIPTION
    执行 cargo build --release（hit-cli + hit-shim），将两个产物复制到 scripts/ 下，
    方便后续手动上传到远程 Release。

    install-hit.ps1 的本地模式要求 hit.exe 与 hit-shim.exe 同目录，故此处必须同时产出二者。

    用法：
        .\scripts\release.ps1
#>

$ErrorActionPreference = 'Stop'

function Write-Step { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Cyan; Write-Host $msg }
function Write-Ok   { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Green; Write-Host $msg }
function Write-Fail { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Red; Write-Host "ERROR: $msg"; exit 1 }

# 计算文件 SHA256（兼容 PowerShell 5.1）
function Get-FileSha256 { param([string]$Path)
    ([System.Security.Cryptography.SHA256]::Create().ComputeHash([System.IO.File]::ReadAllBytes($Path)) | ForEach-Object { $_.ToString("x2") }) -join ''
}

# ── 编译 ──────────────────────────────────────────────────────────────

Write-Step "编译 release 产物（hit-cli + hit-shim）..."
$repoRoot = Split-Path (Split-Path $MyInvocation.MyCommand.Path -Parent) -Parent
Set-Location $repoRoot

cargo build --release -p hit-cli -p hit-shim
if ($LASTEXITCODE -ne 0) {
    Write-Fail "cargo build 失败"
}

# ── 复制到 scripts/ ───────────────────────────────────────────────────

$scriptsDir = Split-Path $MyInvocation.MyCommand.Path
$assets = @(
    @{ Name = 'hit.exe';       Src = "target\release\hit.exe" },
    @{ Name = 'hit-shim.exe';  Src = "target\release\hit-shim.exe" }
)

foreach ($a in $assets) {
    $src = Join-Path $repoRoot $a.Src
    $dst = Join-Path $scriptsDir $a.Name
    if (-not (Test-Path $src)) {
        Write-Fail "编译产物不存在：$src"
    }
    Write-Step "复制 $($a.Name) → scripts\$($a.Name)"
    Copy-Item -Path $src -Destination $dst -Force

    # 校验哈希一致
    $srcHash = Get-FileSha256 $src
    $dstHash = Get-FileSha256 $dst
    if ($srcHash -ne $dstHash) {
        Write-Fail "$($a.Name) 复制后哈希不一致"
    }
    Write-Ok "scripts\$($a.Name) 已更新（SHA256: $srcHash）"
}

# ── 完成 ──────────────────────────────────────────────────────────────

Write-Host ""
Write-Ok "发布准备完成！"
Write-Host ""
Write-Host "    产物  : $scriptsDir\hit.exe"
Write-Host "             $scriptsDir\hit-shim.exe"
Write-Host ""
Write-Host "    接下来请手动上传到 GitHub/CNB Release（hit.exe + hit-shim.exe）。"
Write-Host ""
