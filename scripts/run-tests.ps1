#requires -Version 7
<#
    Hit 批量测试脚本：按 docs/bugs/TEST_FLOW.md 顺序执行，
    stdout → REPORT.md（业务输出），stderr → REPORT_warn.log（WARN 日志）。
    跳过：§1（交互引导）、§15（TUI）、§12.3.1（开浏览器）、§20（完全卸载）、
          §14.2/§14.4（需手动破坏 junction/shim）、§19.4（需断网）。
#>

$ErrorActionPreference = 'Continue'
Set-StrictMode -Version 3

$repoRoot = 'C:\Repos\Hit'
$report   = Join-Path $repoRoot 'docs\bugs\REPORT.md'
$warnLog  = Join-Path $repoRoot 'docs\bugs\REPORT_warn.log'

# 重置文件
"# Hit 实测报告（自动批量执行）`n> 生成时间: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')`n> 按 TEST_FLOW.md 顺序，stdout 入本文件，stderr(WARN) 入 REPORT_warn.log。`n> 跳过: §1/§15/§12.3.1/§20/§14.2/§14.4/§19.4`n" | Set-Content -Path $report -Encoding UTF8
"" | Set-Content -Path $warnLog -Encoding UTF8

# 执行单条命令：业务输出追加到 REPORT，WARN 追加到 warnLog
function Run-Case {
    param(
        [string]$Id,        # 测试编号 如 "2.1.1"
        [string]$Cmd,       # 命令字符串
        [string]$Note = ''  # 备注（跳过原因等）
    )
    $sep = "────────────────────────────────────────────────"
    Add-Content -Path $report -Value "`n$sep`n## §$Id`n$sep" -Encoding UTF8
    Add-Content -Path $report -Value "命令: $Cmd" -Encoding UTF8
    if ($Note) {
        Add-Content -Path $report -Value "备注: $Note" -Encoding UTF8
        Add-Content -Path $report -Value "输出: [跳过]" -Encoding UTF8
        return
    }
    Add-Content -Path $report -Value "输出（原样）:" -Encoding UTF8

    Add-Content -Path $warnLog -Value "`n$sep`n[§$Id] $Cmd" -Encoding UTF8

    # 用合并捕获 2>&1：stdout 入 REPORT，stderr 同时入 warnLog + 追加到 REPORT（非 WARN 行）
    $stdoutTmp = Join-Path $env:TEMP "hit_out_$Id.txt"
    $stderrTmp = Join-Path $env:TEMP "hit_err_$Id.txt"
    $mergedTmp = Join-Path $env:TEMP "hit_merged_$Id.txt"

    $expr = $Cmd
    try {
        Invoke-Expression "$expr *>`$mergedTmp"
    } catch {
        Add-Content -Path $report -Value "[执行异常] $_" -Encoding UTF8
    }
    if (Test-Path $mergedTmp) {
        $allContent = Get-Content -Path $mergedTmp -Raw
        # 全部写入 REPORT（原始输出，含 WARN 和错误）
        Add-Content -Path $report -Value $allContent -Encoding UTF8
        # WARN/tracing 行单独写入 warnLog
        Get-Content -Path $mergedTmp | Where-Object { $_ -match '^\d{4}-\d{2}-\d{2}T|^  |WARN|ERROR|错误' } | Add-Content -Path $warnLog -Encoding UTF8
        Remove-Item $mergedTmp -Force -ErrorAction SilentlyContinue
    }
    if (Test-Path $stdoutTmp) { Remove-Item $stdoutTmp -Force -ErrorAction SilentlyContinue }
    if (Test-Path $stderrTmp) { Remove-Item $stderrTmp -Force -ErrorAction SilentlyContinue }
}

# ═══════════════════════════════════════════════════════════════════════
# §0 测试准备（已安装，只做确认）
# ═══════════════════════════════════════════════════════════════════════
Run-Case '0.3' 'hit --version'
Run-Case '0.3' 'hit --help'
Run-Case '0.4' 'hit prefix'
Run-Case '0.4' 'hit config list'

# ═══════════════════════════════════════════════════════════════════════
# §2 Bucket 管理
# ═══════════════════════════════════════════════════════════════════════
# 2.1 add（main/extras/versions 已存在，重复添加应报错）
Run-Case '2.1.1' 'hit bucket add main'
Run-Case '2.1.4' 'hit bucket add main'
Run-Case '2.1.5' 'hit bucket add myrepo https://github.com/user/repo.git'
Run-Case '2.1.6' 'hit bucket add unknownbucket'
# 2.2 list
Run-Case '2.2.1' 'hit bucket list'
Run-Case '2.2.2' 'hit b ls'
# 2.3 update
Run-Case '2.3.1' 'hit bucket update'
Run-Case '2.3.2' 'hit bucket update main'
Run-Case '2.3.3' 'hit bucket update nonexistent'
# 2.4 remove（先清理 2.1.5 失败的 myrepo 不存在；rm main 后需加回）
Run-Case '2.4.1' 'hit bucket remove myrepo'
Run-Case '2.4.2' 'hit bucket rm main'
Run-Case '2.4.3' 'hit bucket remove nonexistent'
# 加回 main 供后续测试
Run-Case '2.4-restore' 'hit bucket add main'

# ═══════════════════════════════════════════════════════════════════════
# §3 搜索
# ═══════════════════════════════════════════════════════════════════════
Run-Case '3.1' 'hit search git'
Run-Case '3.2' 'hit s python'
Run-Case '3.3' 'hit search GIT'
Run-Case '3.4' 'hit search git --bucket main'
Run-Case '3.5' 'hit search nonexistent_xyz'

# ═══════════════════════════════════════════════════════════════════════
# §4 查看详情
# ═══════════════════════════════════════════════════════════════════════
Run-Case '4.1' 'hit info git'
Run-Case '4.2' 'hit info git --bucket main'
Run-Case '4.3' 'hit info nonexistent'
Run-Case '4.4' 'hit info curl'   # curl 在 main+extras 都有，测试多 bucket 提示

# ═══════════════════════════════════════════════════════════════════════
# §5 安装
# ═══════════════════════════════════════════════════════════════════════
Run-Case '5.1' 'hit install curl'
Run-Case '5.2' 'hit i jq'
Run-Case '5.3' 'hit install curl'          # 重复安装
Run-Case '5.4' 'hit install curl --force'
Run-Case '5.5' 'hit install main/git'      # 注：git 较大，可能耗时
Run-Case '5.6' 'hit install nonexistent_pkg'
Run-Case '5.7' 'hit install jq --arch 64bit'

# ═══════════════════════════════════════════════════════════════════════
# §6 列出已安装
# ═══════════════════════════════════════════════════════════════════════
Run-Case '6.1' 'hit list'
Run-Case '6.2' 'hit ls'
Run-Case '6.3' 'hit list curl'
Run-Case '6.4' 'hit list nonexistent'

# ═══════════════════════════════════════════════════════════════════════
# §7 版本管理（需安装多版本，可能耗时，保留测试）
# ═══════════════════════════════════════════════════════════════════════
Run-Case '7.1.1' 'hit install python@3.11.0'   # 可能不存在或失败，记输出
Run-Case '7.1.2' 'hit install python@3.12.0'
Run-Case '7.1.3' 'hit reset python 3.11.0'
Run-Case '7.1.4' 'hit reset python 9.9.9'
# 7.2 hold/unhold
Run-Case '7.2.1' 'hit hold curl'
Run-Case '7.2.2' 'hit hold curl'
Run-Case '7.2.3' 'hit update --all'
Run-Case '7.2.4' 'hit unhold curl'
Run-Case '7.2.5' 'hit unhold curl'
Run-Case '7.2.6' 'hit hold nonexistent'

# ═══════════════════════════════════════════════════════════════════════
# §8 更新
# ═══════════════════════════════════════════════════════════════════════
Run-Case '8.1' 'hit update'
Run-Case '8.2' 'hit update --all'
Run-Case '8.3' 'hit update curl'
Run-Case '8.4' 'hit update nonexistent'
Run-Case '8.5' 'hit update --force'

# ═══════════════════════════════════════════════════════════════════════
# §9 卸载
# ═══════════════════════════════════════════════════════════════════════
Run-Case '9.1' 'hit uninstall jq'
Run-Case '9.2' 'hit rm curl --purge'
Run-Case '9.3' 'hit uninstall nonexistent'
Run-Case '9.4' 'hit uninstall'
Run-Case '9.5' 'hit uninstall jq curl'   # jq 已卸，curl 已卸，应都报未安装

# ═══════════════════════════════════════════════════════════════════════
# §10 缓存（先重装 curl 产生缓存）
# ═══════════════════════════════════════════════════════════════════════
Run-Case '10-pre' 'hit install curl'
Run-Case '10.1' 'hit cache list'
Run-Case '10.2' 'hit cache dir'
Run-Case '10.3' 'hit cache clean'
Run-Case '10.4' 'hit cache clean curl'
Run-Case '10.5' 'hit cache list'

# ═══════════════════════════════════════════════════════════════════════
# §11 清理旧版本（前提：§7 安装了多版本 python）
# ═══════════════════════════════════════════════════════════════════════
Run-Case '11.1' 'hit cleanup python'
Run-Case '11.2' 'hit cleanup --all'
Run-Case '11.3' 'hit cleanup --cache'
Run-Case '11.4' 'hit cleanup'

# ═══════════════════════════════════════════════════════════════════════
# §12 路径查询
# ═══════════════════════════════════════════════════════════════════════
Run-Case '12.1.1' 'hit which curl'
Run-Case '12.1.2' 'hit which nonexistent'
Run-Case '12.2.1' 'hit prefix'
Run-Case '12.2.2' 'hit prefix curl'
Run-Case '12.2.3' 'hit prefix nonexistent'
Run-Case '12.3.1' 'hit home git'         '跳过：会打开浏览器'
Run-Case '12.3.2' 'hit home nonexistent'

# ═══════════════════════════════════════════════════════════════════════
# §13 配置管理
# ═══════════════════════════════════════════════════════════════════════
Run-Case '13.1'  'hit config list'
Run-Case '13.2'  'hit config set proxy http://127.0.0.1:7890'
Run-Case '13.3'  'hit config set aria2_enabled true'
Run-Case '13.4'  'hit config set aria2_enabled yes'
Run-Case '13.5'  'hit config set aria2_enabled maybe'
Run-Case '13.6'  'hit config set auto_cleanup_days 60'
Run-Case '13.7'  'hit config set auto_cleanup_days abc'
Run-Case '13.8'  'hit config set unknown_key value'
Run-Case '13.9'  'hit config set proxy ""'
Run-Case '13.10' 'hit config list'

# ═══════════════════════════════════════════════════════════════════════
# §14 健康检查
# ═══════════════════════════════════════════════════════════════════════
Run-Case '14.1' 'hit doctor'
Run-Case '14.2' 'hit doctor'  '跳过：需手动删除 current junction 后测试'
Run-Case '14.3' 'hit doctor --fix'
Run-Case '14.4' 'hit doctor --fix' '跳过：需手动创建损坏 .shim 后测试'

# ═══════════════════════════════════════════════════════════════════════
# §16 系统状态
# ═══════════════════════════════════════════════════════════════════════
Run-Case '16.1' 'hit status'
Run-Case '16.2' 'hit st'

# ═══════════════════════════════════════════════════════════════════════
# §17 命令简写（alias）— 用轻量命令验证 alias 解析，不重复触发安装
# ═══════════════════════════════════════════════════════════════════════
Run-Case '17-i'   'hit i nonexistent_alias_test'
Run-Case '17-s'   'hit s nonexistent_alias_test'
Run-Case '17-u'   'hit u nonexistent'
Run-Case '17-rm'  'hit rm nonexistent'
Run-Case '17-ls'  'hit ls'
Run-Case '17-st'  'hit st'
Run-Case '17-b'   'hit b ls'
Run-Case '17-c'   'hit c'
Run-Case '17-r'   'hit r nonexistent 1.0.0'

# ═══════════════════════════════════════════════════════════════════════
# §18 日志级别
# ═══════════════════════════════════════════════════════════════════════
Run-Case '18.1' 'hit -v list'
Run-Case '18.2' 'hit -vv list'
Run-Case '18.3' 'hit -vvv list'

# ═══════════════════════════════════════════════════════════════════════
# §19 错误处理与边界
# ═══════════════════════════════════════════════════════════════════════
Run-Case '19.1' 'hit'
Run-Case '19.2' 'hit wrongcmd'
Run-Case '19.3' 'hit install'
Run-Case '19.4' 'hit bucket add main'  '跳过：需断网环境'
Run-Case '19.5' 'hit list'

Add-Content -Path $report -Value "`n$sep`n## 测试结束`n$sep`n完成时间: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')`n" -Encoding UTF8

Write-Host "`n[完成] REPORT: $report"
Write-Host "[完成] WARN 日志: $warnLog"
