#Requires -Version 5
<#
.SYNOPSIS
    Hit 一键安装脚本（Windows）

.DESCRIPTION
    下载 hit.exe、初始化 ~/.hit/ 目录布局、注册 shims 到用户 PATH、广播
    WM_SETTINGCHANGE 通知其它进程刷新环境变量。

    用法：
        # 交互式安装（推荐）：无参数运行，一步步引导
        .\install-hit.ps1

        # 静默安装：直接指定参数
        irm https://get.hit.sh | iex
        .\install-hit.ps1 -Path C:\Users\me\.hit
        .\install-hit.ps1 -Mirror tuna -Version 1.0.0
        .\install-hit.ps1 -FromLocal .\target\release\hit.exe
        .\install-hit.ps1 -Force

.PARAMETER Path
    安装目录。优先级：-Path > $env:HIT_ROOT > $env:SCOOP > $HOME\.hit

.PARAMETER Mirror
    下载镜像：github（默认）/ tuna / aliyun

.PARAMETER Version
    指定版本号（如 1.0.0）；默认 latest

.PARAMETER Force
    覆盖现有安装

.PARAMETER FromLocal
    跳过网络下载，直接复制本地预编译的 hit.exe（开发调试用）

.PARAMETER NonInteractive
    跳过交互提示，全部使用参数或默认值（静默模式）

.NOTES
    参考实现：`ref/Scoop/bin/install.ps1` 与 `ref/Scoop/lib/install.ps1`
#>

param(
    [string]$Path,
    [ValidateSet('github', 'tuna', 'aliyun')]
    [string]$Mirror = 'github',
    [string]$Version = 'latest',
    [switch]$Force,
    [string]$FromLocal,
    [switch]$NonInteractive
)

$ErrorActionPreference = 'Stop'

# ── 辅助函数 ─────────────────────────────────────────────────────────────

function Write-Step  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Cyan; Write-Host $msg }
function Write-Ok    { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Green; Write-Host $msg }
function Write-Warn  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Yellow; Write-Host "WARNING: $msg" }
function Write-Fail  { param([string]$msg) Write-Host "[Hit] " -NoNewline -ForegroundColor Red; Write-Host "ERROR: $msg"; exit 1 }

function Read-WithDefault {
    param([string]$Prompt, [string]$Default)
    $input = Read-Host "$Prompt（默认: $Default）"
    if ([string]::IsNullOrWhiteSpace($input)) { $Default } else { $input }
}

function Confirm-YesNo {
    param([string]$Prompt, [bool]$Default)
    $hint = if ($Default) { 'Y/n' } else { 'y/N' }
    $input = Read-Host "$Prompt [$hint]"
    if ([string]::IsNullOrWhiteSpace($input)) { return $Default }
    return $input -in @('y', 'Y', 'yes', 'YES')
}

# ── 0. 交互式引导 ───────────────────────────────────────────────────────

if (-not $NonInteractive -and -not $FromLocal) {
    Write-Host ""
    Write-Host "╔══════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║      Hit 安装向导                        ║" -ForegroundColor Cyan
    Write-Host "║  直接回车使用默认值，一路 Enter 即可       ║" -ForegroundColor Cyan
    Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""

    # 安装路径
    $defaultPath = Join-Path $env:USERPROFILE '.hit'
    $pathInput = Read-WithDefault "请输入安装路径" $defaultPath
    $Path = $pathInput

    # 安装方式
    Write-Host ""
    Write-Host "  安装方式：" -ForegroundColor Yellow
    Write-Host "    1) 从 GitHub 下载（默认）"
    Write-Host "    2) 使用本地编译的 exe（开发调试）"
    $modeInput = Read-Host "请选择 [1/2]（默认: 1）"
    if ($modeInput -eq '2') {
        do {
            $localInput = Read-Host "请输入 hit.exe 路径"
            if ([string]::IsNullOrWhiteSpace($localInput)) {
                Write-Host "  路径不能为空，请重新输入" -ForegroundColor Yellow
            }
            elseif (-not (Test-Path $localInput)) {
                Write-Host "  文件不存在：$localInput，请重新输入" -ForegroundColor Yellow
            }
            else { break }
        } while ($true)
        $FromLocal = $localInput
    }
    else {
        # 镜像
        Write-Host ""
        Write-Host "  下载镜像：" -ForegroundColor Yellow
        Write-Host "    1) GitHub（默认）"
        Write-Host "    2) 清华大学 TUNA 镜像"
        Write-Host "    3) 阿里云镜像"
        $mirrorInput = Read-Host "请选择 [1/2/3]（默认: 1）"
        $Mirror = switch ($mirrorInput) {
            '2' { 'tuna' }
            '3' { 'aliyun' }
            default { 'github' }
        }

        # 版本
        Write-Host ""
        $versionInput = Read-WithDefault "请输入版本号（latest 表示最新版）" 'latest'
        $Version = $versionInput
    }

    # 覆盖安装
    Write-Host ""
    $exePath = Join-Path $Path 'hit.exe'
    if (Test-Path $exePath) {
        $Force = -not (Confirm-YesNo "检测到已有安装，是否保留不覆盖" $true)
        if (-not $Force) {
            Write-Ok "保留现有安装，退出"
            exit 0
        }
    }

    Write-Host ""
    Write-Host "╔══════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║  开始安装...                             ║" -ForegroundColor Cyan
    Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

# ── 1. 环境检查 ─────────────────────────────────────────────────────────

Write-Step "检查 PowerShell 版本..."
if ($PSVersionTable.PSVersion.Major -lt 5) {
    Write-Fail "需要 PowerShell 5+，当前版本：$($PSVersionTable.PSVersion)"
}

# 执行策略（CurrentUser 级别，无需管理员）
$policy = Get-ExecutionPolicy -Scope CurrentUser -List |
    Where-Object { $_ -notin @('Undefined', 'Restricted') } |
    Select-Object -First 1
if (-not $policy) {
    Write-Step "设置执行策略 RemoteSigned (CurrentUser)..."
    Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
}

# 确定安装路径：-Path > $HIT_ROOT > $SCOOP > $HOME\.hit
if (-not $Path) {
    $Path = $env:HIT_ROOT
}
if (-not $Path) {
    $Path = $env:SCOOP
}
if (-not $Path) {
    $homeDir = if ($env:USERPROFILE) { $env:USERPROFILE } else { $env:HOME }
    if (-not $homeDir) {
        Write-Fail "无法确定用户目录：请设置 -Path 或 $env:HIT_ROOT"
    }
    $Path = Join-Path $homeDir '.hit'
}
$resolved = Resolve-Path $Path -ErrorAction SilentlyContinue
if ($resolved) { $Path = $resolved.Path }

# 非交互模式下输出配置概览
if ($NonInteractive) {
    Write-Step "安装目录：$Path"
    if ($FromLocal) { Write-Step "安装方式：本地文件 ($FromLocal)" }
    else { Write-Step "安装方式：网络下载 | 镜像：$Mirror | 版本：$Version" }
}

$hitExe = Join-Path $Path 'hit.exe'
$shimsDir = Join-Path $Path 'shims'

if ((Test-Path $hitExe) -and -not $Force -and -not $FromLocal) {
    Write-Ok "Hit 已安装在 $hitExe"
    Write-Host "    如需更新请运行：hit update"
    Write-Host "    强制重装请添加：  -Force"
    exit 0
}

# ── 2. 获取 hit.exe ─────────────────────────────────────────────────────

if ($FromLocal) {
    # 本地预编译二进制模式
    if (-not (Test-Path $FromLocal)) {
        Write-Fail "本地二进制不存在：$FromLocal"
    }
    $exeSource = Resolve-Path $FromLocal
    Write-Step "使用本地二进制：$exeSource"
}
else {
    # 架构检测
    $arch = if ([Environment]::Is64BitOperatingSystem) { 'x86_64' } else { 'i686' }
    if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { $arch = 'aarch64' }

    $baseUrl = switch ($Mirror) {
        'github' { 'https://github.com/hit-buckets/hit/releases' }
        'tuna'   { 'https://mirrors.tuna.tsinghua.edu.cn/github-release/hit-buckets/hit' }
        'aliyun' { 'https://mirrors.aliyun.com/github-release/hit-buckets/hit' }
    }

    if ($Version -eq 'latest') {
        $downloadUrl = "$baseUrl/latest/download/hit-$arch-pc-windows-msvc.zip"
    }
    else {
        $downloadUrl = "$baseUrl/download/v$Version/hit-$arch-pc-windows-msvc.zip"
    }

    $zipPath = Join-Path $env:TEMP "hit-$Version-$arch.zip"
    Write-Step "下载 $downloadUrl"
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
    }
    catch {
        Write-Fail "下载失败：$($_.Exception.Message)`n    请检查网络或切换镜像 (-Mirror tuna)"
    }

    $extractDir = Join-Path $env:TEMP "hit-extract-$(Get-Random)"
    Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force
    $exeSource = Get-ChildItem $extractDir -Filter hit.exe -Recurse | Select-Object -First 1
    if (-not $exeSource) {
        Write-Fail "zip 中找不到 hit.exe：$zipPath"
    }
}

# ── 3. 初始化目录布局 ──────────────────────────────────────────────────

Write-Step "初始化目录布局..."
$subDirs = @('apps', 'shims', 'cache', 'persist', 'buckets', 'logs')
foreach ($d in $subDirs) {
    $full = Join-Path $Path $d
    if (-not (Test-Path $full)) {
        New-Item -ItemType Directory -Path $full -Force | Out-Null
    }
}

# 复制 hit.exe 到安装根目录与 shims/
Copy-Item -Path $exeSource -Destination $hitExe -Force
Copy-Item -Path $exeSource -Destination (Join-Path $shimsDir 'hit.exe') -Force
Write-Ok "hit.exe 已部署到 $Path"

# 写入默认 config.json（仅首次安装）
$configPath = Join-Path $Path 'config.json'
if (-not (Test-Path $configPath)) {
    $mirrorValue = if ($Mirror -eq 'github') { $null } else { $Mirror }
    $config = [pscustomobject]@{
        proxy                          = $null
        mirror                         = $mirrorValue
        aria2_enabled                  = $false
        no_junction                    = $false
        root_path                      = $null
        auto_cleanup_days              = 30
        health_check_interval_days     = 7
    }
    $config | ConvertTo-Json -Depth 3 | Set-Content -Path $configPath -Encoding UTF8
    Write-Ok "默认配置已写入 $configPath"
}

# 清理临时下载
if (-not $FromLocal -and (Test-Path $zipPath)) {
    Remove-Item $zipPath -Force -ErrorAction SilentlyContinue
    Remove-Item $extractDir -Recurse -Force -ErrorAction SilentlyContinue
}

# ── 4. 注册 PATH（仅 CurrentUser，无需管理员） ──────────────────────

Write-Step "注册 shims 目录到用户 PATH..."
try {
    $regKey = 'HKCU:\Environment'
    $currentPath = (Get-ItemProperty -Path $regKey -Name Path -ErrorAction SilentlyContinue).Path
    if ($currentPath -and ($currentPath -split ';' | ForEach-Object { $_.TrimEnd('\') }) -contains $shimsDir.TrimEnd('\')) {
        Write-Ok "PATH 已包含 $shimsDir（无需修改）"
    }
    else {
        $newPath = if ($currentPath) { "$currentPath;$shimsDir" } else { $shimsDir }
        Set-ItemProperty -Path $regKey -Name Path -Value $newPath -Type ExpandString
        Write-Ok "已追加 $shimsDir 到 HKCU\Environment\Path"

        # 广播 WM_SETTINGCHANGE 通知其它进程刷新环境变量
        Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class HitBroadcast {
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
        [HitBroadcast]::SettingChange()
    }
}
catch {
    Write-Warn "PATH 注册失败：$($_.Exception.Message)`n    请手动将 $shimsDir 加入系统 PATH"
}

# ── 5. 完成 ─────────────────────────────────────────────────────────────

Write-Host ""
Write-Ok "Hit 安装完成！"
Write-Host ""
Write-Host "    安装路径：$Path"
Write-Host "    二进制  ：$hitExe"
Write-Host "    配置    ：$configPath"
Write-Host "    Shims   ：$shimsDir"
Write-Host ""
Write-Host "    请重新打开终端让 PATH 生效，然后运行："
Write-Host ""
Write-Host "        hit --help"
Write-Host "        hit bucket add main"
Write-Host "        hit install <package>"
Write-Host ""
