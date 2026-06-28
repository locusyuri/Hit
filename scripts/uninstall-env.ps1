#Requires -Version 5
<#
.SYNOPSIS
    清理 Hit 在系统中注册的环境变量与 PATH 条目（卸载辅助脚本）

.DESCRIPTION
    执行以下清理操作（需两次确认）：
      1. 从 HKCU\Environment\Path 中移除 Hit 的根目录和 shims 目录条目
      2. 删除 HKCU\Environment\HIT_ROOT 环境变量（若存在）
    本脚本不会删除 SCOOP 环境变量（与 Scoop 共享），
    也不会删除 ~/.hit/ 目录或已安装的软件（如需请手动删除）。

    广播 WM_SETTINGCHANGE 通知其它进程刷新环境变量。

.PARAMETER Force
    跳过两次确认，直接执行清理（用于自动化场景，慎用）

.EXAMPLE
    .\uninstall-env.ps1
    .\uninstall-env.ps1 -Force
#>

param([switch]$Force)

$ErrorActionPreference = 'Stop'

# ── 辅助函数 ─────────────────────────────────────────────────────────────

function Write-Step  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Cyan; Write-Host $msg }
function Write-Ok    { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Green; Write-Host $msg }
function Write-Warn  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Yellow; Write-Host "WARNING: $msg" }
function Write-Fail  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Red; Write-Host "ERROR: $msg"; exit 1 }

function Confirm-Twice {
    param([string]$Prompt)
    # 第一次确认
    $input1 = Read-Host "$Prompt [y/N]"
    if ($input1 -notin @('y', 'Y', 'yes', 'YES')) {
        return $false
    }
    # 第二次确认
    $input2 = Read-Host "再次确认以执行 [y/N]"
    return ($input2 -in @('y', 'Y', 'yes', 'YES'))
}

function Broadcast-SettingChange {
    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class HitEnvBroadcast {
    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern IntPtr SendMessageTimeout(
        IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam,
        uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
    public static void SettingChange() {
        UIntPtr result;
        SendMessageTimeout((IntPtr)0xFFFF, 0x001A, UIntPtr.Zero,
            "Environment", 0x0002, 5000, out result);
    }
}
"@
    [HitEnvBroadcast]::SettingChange()
}

# ── 收集待清理项 ────────────────────────────────────────────────────────

$regKey = 'HKCU:\Environment'
$hitRoot = (Get-ItemProperty -Path $regKey -Name HIT_ROOT -ErrorAction SilentlyContinue).HIT_ROOT
$currentPath = (Get-ItemProperty -Path $regKey -Name Path -ErrorAction SilentlyContinue).Path

# 从 PATH 中找出所有 Hit 相关条目（根目录 + shims 目录）
$hitPathEntries = @()
if ($currentPath) {
    $entries = $currentPath -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_ }
    foreach ($e in $entries) {
        $eNorm = $e.TrimEnd('\')
        $isHit = $false
        # 根目录匹配：~/.hit、<HIT_ROOT>、或明显为 hit 测试目录
        if ($eNorm -match '\\\.hit$') { $isHit = $true }
        if ($hitRoot -and ($eNorm -eq $hitRoot.TrimEnd('\'))) { $isHit = $true }
        if ($eNorm -match '\\hit$') { $isHit = $true }
        # shims 目录匹配
        if ($eNorm -match '\\\.hit\\shims$') { $isHit = $true }
        if ($hitRoot -and ($eNorm -eq (Join-Path $hitRoot 'shims').TrimEnd('\'))) { $isHit = $true }
        if ($eNorm -match '\\hit\\shims$') { $isHit = $true }
        if ($isHit) { $hitPathEntries += $eNorm }
    }
}

Write-Host ""
Write-Host "将清理以下内容：" -ForegroundColor Yellow
Write-Host "  HIT_ROOT 环境变量：$hitRoot" -ForegroundColor Yellow
Write-Host "  PATH 中的 Hit 条目：$($hitPathEntries -join ' ; ')" -ForegroundColor Yellow
Write-Host "  （不会删除 SCOOP 变量、~/.hit 目录、已安装软件）" -ForegroundColor DarkGray
Write-Host ""

# ── 两次确认 ────────────────────────────────────────────────────────────

if (-not $Force) {
    Write-Host "需要两次确认才会执行清理。" -ForegroundColor Cyan
    Write-Host ""
    if (-not (Confirm-Twice "确认清理以上环境变量？")) {
        Write-Ok "已取消，未做任何修改"
        exit 0
    }
}

# ── 执行清理 ────────────────────────────────────────────────────────────

# 1. 删除 HIT_ROOT
if ($hitRoot) {
    try {
        Remove-ItemProperty -Path $regKey -Name HIT_ROOT -ErrorAction SilentlyContinue
        Write-Ok "已删除 HIT_ROOT"
    }
    catch {
        Write-Warn "删除 HIT_ROOT 失败：$($_.Exception.Message)"
    }
}

# 2. 从 PATH 移除 hit 相关条目（根目录 + shims）
if ($hitPathEntries.Count -gt 0) {
    try {
        $entries = $currentPath -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_ }
        $kept = $entries | Where-Object { ($_.TrimEnd('\') -notin $hitPathEntries) }
        $newPath = ($kept -join ';')
        Set-ItemProperty -Path $regKey -Name Path -Value $newPath -Type ExpandString
        Write-Ok "已从 PATH 移除 $($hitPathEntries.Count) 个 hit shims 条目"
    }
    catch {
        Write-Warn "PATH 清理失败：$($_.Exception.Message)"
    }
}

# 3. 广播 WM_SETTINGCHANGE
try {
    Broadcast-SettingChange
    Write-Ok "已广播 WM_SETTINGCHANGE（请重新打开终端使变更生效）"
}
catch {
    Write-Warn "广播 WM_SETTINGCHANGE 失败：$($_.Exception.Message)"
}

Write-Host ""
Write-Ok "环境变量清理完成"
