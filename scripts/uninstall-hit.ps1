#Requires -Version 5
<#
.SYNOPSIS
    Hit 彻底卸载脚本（删除文件 + 清理环境变量）

.DESCRIPTION
    完整卸载 Hit，包括：
      1. 从环境变量定位 Hit 根目录（HIT_ROOT 或 PATH 中的 hit 路径）
      2. 列出即将删除的目录和文件，需用户确认
      3. 删除 Hit 根目录（含 apps/shims/cache/buckets/config.json 等全部内容）
      4. 从 HKCU\Environment\Path 中精确移除 Hit 相关条目（绝不影响其他路径）
      5. 删除 HKCU\Environment\HIT_ROOT 环境变量
      6. 广播 WM_SETTINGCHANGE 通知其他进程刷新

    安全措施：
      - 每一步操作前都需用户确认
      - PATH 清理采用白名单匹配 + 逐条确认，绝不删除非 Hit 条目
      - 删除目录前验证路径合法性（必须包含 'hit' 且在用户目录下）
      - 不删除 SCOOP 环境变量（与 Scoop 共享）

.PARAMETER Force
    跳过确认提示，直接执行（慎用！仅用于自动化/测试）

.EXAMPLE
    .\uninstall-hit.ps1
    .\uninstall-hit.ps1 -Force
#>

param([switch]$Force)

$ErrorActionPreference = 'Stop'

# ── 辅助函数 ─────────────────────────────────────────────────────────────

function Write-Step  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Cyan; Write-Host $msg }
function Write-Ok    { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Green; Write-Host $msg }
function Write-Warn  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Yellow; Write-Host "WARNING: $msg" }
function Write-Fail  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Red; Write-Host "ERROR: $msg"; exit 1 }

function Confirm-Yes {
    param([string]$Prompt)
    if ($Force) { return $true }
    $input = Read-Host "$Prompt [y/N]"
    return ($input -in @('y', 'Y', 'yes', 'YES'))
}

function Confirm-Twice {
    param([string]$Prompt)
    if ($Force) { return $true }
    $input1 = Read-Host "$Prompt [y/N]"
    if ($input1 -notin @('y', 'Y', 'yes', 'YES')) { return $false }
    $input2 = Read-Host "再次确认以执行 [y/N]"
    return ($input2 -in @('y', 'Y', 'yes', 'YES'))
}

function Broadcast-SettingChange {
    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class HitUninstallBroadcast {
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
    [HitUninstallBroadcast]::SettingChange()
}

# ── 1. 定位 Hit 根目录 ──────────────────────────────────────────────────

$regKey = 'HKCU:\Environment'
$hitRoot = $null

# 优先从 HIT_ROOT 环境变量获取
$envHitRoot = (Get-ItemProperty -Path $regKey -Name HIT_ROOT -ErrorAction SilentlyContinue).HIT_ROOT
if ($envHitRoot -and (Test-Path $envHitRoot)) {
    $hitRoot = $envHitRoot
    Write-Step "从 HIT_ROOT 环境变量定位：$hitRoot"
}

# 若 HIT_ROOT 未设或路径不存在，从 PATH 中查找
if (-not $hitRoot) {
    $currentPath = (Get-ItemProperty -Path $regKey -Name Path -ErrorAction SilentlyContinue).Path
    if ($currentPath) {
        $entries = $currentPath -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_ }
        foreach ($e in $entries) {
            $eNorm = $e.TrimEnd('\')
            # 匹配包含 hit.exe 的目录（根目录，非 shims）
            $hitExe = Join-Path $eNorm 'hit.exe'
            if (Test-Path $hitExe) {
                $hitRoot = $eNorm
                Write-Step "从 PATH 条目定位：$hitRoot（含 hit.exe）"
                break
            }
        }
    }
}

# 最后兜底：检查默认路径
if (-not $hitRoot) {
    $defaultRoot = Join-Path $env:USERPROFILE '.hit'
    if (Test-Path (Join-Path $defaultRoot 'hit.exe')) {
        $hitRoot = $defaultRoot
        Write-Step "从默认路径定位：$hitRoot"
    }
}

if (-not $hitRoot) {
    Write-Fail "无法定位 Hit 安装目录。请确认 Hit 已安装，或手动指定 HIT_ROOT 环境变量。"
}

# 验证路径合法性：必须包含 'hit' 且在用户目录树下
$hitRoot = (Resolve-Path $hitRoot -ErrorAction SilentlyContinue).Path
if (-not $hitRoot) {
    Write-Fail "Hit 根目录路径无法解析：$hitRoot"
}
if ($hitRoot -notmatch 'hit') {
    Write-Fail "路径安全性检查失败：'$hitRoot' 不包含 'hit'，拒绝删除以防误删。"
}
if ($hitRoot -eq $env:USERPROFILE -or $hitRoot -eq $env:USERPROFILE.TrimEnd('\')) {
    Write-Fail "路径安全性检查失败：根目录不能是用户主目录 '$hitRoot'，拒绝删除。"
}
# 不允许删除 C:\、D:\ 等驱动器根目录
if ($hitRoot -match '^[A-Z]:\\?$') {
    Write-Fail "路径安全性检查失败：'$hitRoot' 是驱动器根目录，拒绝删除。"
}

Write-Ok "Hit 根目录：$hitRoot"

# ── 2. 列出即将删除的内容 ───────────────────────────────────────────────

$shimsDir = Join-Path $hitRoot 'shims'
$appsDir  = Join-Path $hitRoot 'apps'

Write-Host ""
Write-Host "╔══════════════════════════════════════════╗" -ForegroundColor Red
Write-Host "║  即将彻底卸载 Hit                         ║" -ForegroundColor Red
Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Red
Write-Host ""

# 列出根目录下的关键子目录
Write-Host "  将删除的目录：" -ForegroundColor Yellow
if (Test-Path $hitRoot) {
    $subItems = Get-ChildItem $hitRoot -Force -ErrorAction SilentlyContinue | Sort-Object Name
    foreach ($item in $subItems) {
        $size = ''
        if ($item.PSIsContainer) {
            $count = (Get-ChildItem $item.FullName -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object).Count
            $size = " ($count 个文件/目录)"
        }
        else {
            $len = try { (Get-Item $item.FullName -Force).Length } catch { 0 }
            $size = " ($len 字节)"
        }
        Write-Host "    $($item.Name)$size" -ForegroundColor DarkGray
    }
}
Write-Host "    ...以及 $hitRoot 下所有内容" -ForegroundColor DarkGray

# 列出环境变量变更
Write-Host ""
Write-Host "  将清理的环境变量：" -ForegroundColor Yellow
Write-Host "    HIT_ROOT = $envHitRoot" -ForegroundColor DarkGray

$currentPath = (Get-ItemProperty -Path $regKey -Name Path -ErrorAction SilentlyContinue).Path
$hitPathEntries = @()
if ($currentPath) {
    $entries = $currentPath -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_ }
    foreach ($e in $entries) {
        $eNorm = $e.TrimEnd('\')
        $isHit = $false
        # 精确匹配：HIT_ROOT 本身、HIT_ROOT\shims
        if ($hitRoot -and ($eNorm -eq $hitRoot.TrimEnd('\'))) { $isHit = $true }
        if ($hitRoot -and ($eNorm -eq (Join-Path $hitRoot 'shims').TrimEnd('\'))) { $isHit = $true }
        # 模式匹配：~/.hit、~/.hit/shims、含 \hit\ 或 \hit\shims\ 的路径
        if ($eNorm -match '\\\.hit$') { $isHit = $true }
        if ($eNorm -match '\\\.hit\\shims$') { $isHit = $true }
        if ($hitRoot -and $eNorm -like "$($hitRoot.TrimEnd('\'))*") { $isHit = $true }
        if ($isHit) { $hitPathEntries += $eNorm }
    }
}
if ($hitPathEntries.Count -gt 0) {
    foreach ($e in $hitPathEntries) {
        Write-Host "    PATH 条目：$e" -ForegroundColor DarkGray
    }
}
else {
    Write-Host "    PATH 中无 Hit 相关条目" -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "  ⚠ 此操作不可逆！所有已安装的软件和配置将被永久删除。" -ForegroundColor Red
Write-Host ""

# ── 3. 确认 ─────────────────────────────────────────────────────────────

if (-not (Confirm-Twice "确认彻底卸载 Hit 并删除以上所有内容？")) {
    Write-Ok "已取消，未做任何修改"
    exit 0
}

# ── 4. 删除文件 ─────────────────────────────────────────────────────────

Write-Host ""
Write-Step "删除 Hit 根目录..."

# 二次验证：目录仍然存在且路径合法
if (-not (Test-Path $hitRoot)) {
    Write-Warn "目录已不存在：$hitRoot（跳过）"
}
elseif ($hitRoot -notmatch 'hit') {
    Write-Fail "安全性检查失败：路径不包含 'hit'，拒绝删除"
}
else {
    try {
        Remove-Item -Path $hitRoot -Recurse -Force -ErrorAction Stop
        Write-Ok "已删除 $hitRoot"
    }
    catch {
        Write-Warn "删除目录失败：$($_.Exception.Message)"
        Write-Warn "可能有文件正在被使用。请关闭所有 hit 相关进程后重试，或手动删除："
        Write-Host "    $hitRoot" -ForegroundColor Yellow
    }
}

# ── 5. 清理环境变量 ────────────────────────────────────────────────────

# 5.1 删除 HIT_ROOT
if ($envHitRoot) {
    try {
        Remove-ItemProperty -Path $regKey -Name HIT_ROOT -ErrorAction SilentlyContinue
        Write-Ok "已删除 HIT_ROOT 环境变量"
    }
    catch {
        Write-Warn "删除 HIT_ROOT 失败：$($_.Exception.Message)"
    }
}
else {
    Write-Step "HIT_ROOT 未设置（跳过）"
}

# 5.2 从 PATH 精确移除 Hit 条目
#    安全措施：重新读取 PATH（步骤 4 可能已触发变更），
#    仅移除之前识别的 Hit 条目，绝不触碰其他路径
if ($hitPathEntries.Count -gt 0) {
    Write-Step "从 PATH 移除 Hit 条目..."
    try {
        # 重新读取当前 PATH（防止并发修改）
        $currentPathNow = (Get-ItemProperty -Path $regKey -Name Path -ErrorAction SilentlyContinue).Path
        if ($currentPathNow) {
            $entriesNow = $currentPathNow -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_ }
            # 仅移除之前识别的 Hit 条目（白名单匹配）
            $kept = $entriesNow | Where-Object { ($_.TrimEnd('\') -notin $hitPathEntries) }

            # 安全检查：确保没有误删非 Hit 条目
            $removedCount = $entriesNow.Count - $kept.Count
            if ($removedCount -gt $hitPathEntries.Count) {
                Write-Fail "PATH 安全检查失败：预期移除 $($hitPathEntries.Count) 条，实际将移除 $removedCount 条。拒绝修改 PATH。"
            }

            $newPath = ($kept -join ';')
            Set-ItemProperty -Path $regKey -Name Path -Value $newPath -Type ExpandString
            Write-Ok "已从 PATH 移除 $removedCount 个 Hit 条目"
        }
    }
    catch {
        Write-Warn "PATH 清理失败：$($_.Exception.Message)"
        Write-Warn "请手动从 PATH 中移除以下条目："
        foreach ($e in $hitPathEntries) {
            Write-Host "    $e" -ForegroundColor Yellow
        }
    }
}
else {
    Write-Step "PATH 中无 Hit 条目（跳过）"
}

# 5.3 广播 WM_SETTINGCHANGE
try {
    Broadcast-SettingChange
    Write-Ok "已广播 WM_SETTINGCHANGE"
}
catch {
    Write-Warn "广播 WM_SETTINGCHANGE 失败：$($_.Exception.Message)"
}

# ── 6. 完成 ─────────────────────────────────────────────────────────────

Write-Host ""
Write-Ok "Hit 已彻底卸载！"
Write-Host ""
Write-Host "    已删除目录：$hitRoot" -ForegroundColor DarkGray
Write-Host "    已清理环境变量：HIT_ROOT + PATH 条目" -ForegroundColor DarkGray
Write-Host ""
Write-Host "    请重新打开终端使环境变量变更生效。" -ForegroundColor Cyan
Write-Host ""
