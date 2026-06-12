################### hit - scoop 包装函数，支持子命令短名 ###################
#   hit i xxx   → scoop install xxx (安装软件包)
#   hit s xxx   → scoop search xxx (搜索软件包)
#   hit st xxx  → scoop status xxx  (显示软件包状态)
#   hit u xxx   → scoop update xxx (更新软件包)
#   hit rm xxx  → scoop uninstall xxx (卸载软件包)
#   hit ls      → scoop list (列出已安装的软件包)
#   hit b      → scoop bucket (列出软件源)
#   hit c      → scoop cleanup (清理缓存)
function hit {
    $shortcuts = @{
        'i'   = 'install'
        's'   = 'search'
        'st'  = 'status'
        'u'   = 'update'
        'rm'  = 'uninstall'
        'ls'  = 'list'
        'b'   = 'bucket'
        'c' = 'cleanup'
    }
    if ($args.Count -gt 0 -and $shortcuts.ContainsKey($args[0])) {
        $cmd = $shortcuts[$args[0]] -split '\s+'
        scoop $cmd $args[1..$args.Count]
    } else {
        scoop @args
    }
}

# si - 交互式搜索并安装 (Search & Install)
#   用法: si <关键词>
#   使用 fzf 模糊搜索，显示详细信息，Enter 安装，Esc 取消
Remove-Alias -Name si -Force -ErrorAction SilentlyContinue
function si {
    param([string]$Query)

    if (-not $Query) {
        Write-Host "用法: si <关键词>" -ForegroundColor Yellow
        return
    }

    Write-Host "正在搜索: $Query ..." -ForegroundColor Cyan
    $results = scoop search $Query 2>$null

    if ($LASTEXITCODE -ne 0 -or -not $results) {
        Write-Host "未找到结果" -ForegroundColor Red
        return
    }

    # 按包名分组，聚合 Version / Source / Binaries
    $groups = $results | Where-Object { $_.PSObject.Properties['Name'] } | Group-Object Name
    if ($groups.Count -eq 0) {
        Write-Host "未找到结果" -ForegroundColor Red
        return
    }

    # 构建展示列表（各字段保持完整，截断在显示时处理）
    $items = $groups | ForEach-Object {
        $name = $_.Name
        $vers = ($_.Group | ForEach-Object { $_.Version }) | Select-Object -Unique
        $sources = ($_.Group | ForEach-Object { $_.Source }) | Select-Object -Unique
        $bins = ($_.Group | ForEach-Object { $_.Binaries }) | Where-Object { $_ -and $_ -ne '' } | Select-Object -Unique
        [PSCustomObject]@{
            Name     = $name
            Version  = $vers -join ', '
            Sources  = $sources
            Source   = $sources -join ', '
            Binaries = $bins -join ', '
        }
    } | Sort-Object Name

    # ---- fzf 模式 ----
    # 计算各列最大宽度（取最长值的字符长度，设上限避免行过长）
    $maxName = [Math]::Min(($items.Name | ForEach-Object { $_.Length } | Measure-Object -Maximum).Maximum, 48)
    $maxVer  = [Math]::Min(($items.Version | ForEach-Object { $_.Length } | Measure-Object -Maximum).Maximum, 28)
    $maxSrc  = [Math]::Min(($items.Source | ForEach-Object { $_.Length } | Measure-Object -Maximum).Maximum, 48)
    $maxBin  = [Math]::Min(($items.Binaries | ForEach-Object { $_.Length } | Measure-Object -Maximum).Maximum, 40)

    $fzfLines = $items | ForEach-Object {
        $name = $_.Name
        if ($name.Length -gt $maxName) { $name = $name.Substring(0, $maxName - 1) + '…' }
        $name = $name.PadRight($maxName)

        $ver = $_.Version
        if ($ver.Length -gt $maxVer) { $ver = $ver.Substring(0, $maxVer - 1) + '…' }
        $ver = $ver.PadRight($maxVer)

        $src = $_.Source
        if ($src.Length -gt $maxSrc) { $src = $src.Substring(0, $maxSrc - 1) + '…' }
        $src = $src.PadRight($maxSrc)

        $bin = $_.Binaries
        if ($bin.Length -gt $maxBin) { $bin = $bin.Substring(0, $maxBin - 1) + '…' }
        $bin = $bin.PadRight($maxBin)

        "$name  $ver  [$src]  $bin"
    }
    $joined = $fzfLines -join "`n"
    $chosen = $joined | fzf --height=80% --layout=reverse --prompt="选择包 > " `
        --header="搜索: $Query | Enter=安装  Esc=取消" `
        --cycle

    if (-not $chosen) {
        Write-Host "已取消" -ForegroundColor Yellow
        return
    }
    $pickedName = ($chosen -split '\s+')[0].Trim()
    $item = $items | Where-Object { $_.Name -eq $pickedName }

    $name = $item.Name
    $sources = $item.Sources

    if ($sources.Count -eq 1) {
        Write-Host "正在安装: $name ($($sources[0])) ..." -ForegroundColor Green
        scoop install $name
    }
    else {
        # 用 fzf 交互式选择源
        $srcJoined = $sources -join "`n"
        $srcChosen = $srcJoined | fzf --height=40% --layout=reverse --prompt="选择源 > " `
            --header="$name 在多个源中存在 | Enter=确认  Esc=取消"

        if (-not $srcChosen) { Write-Host "已取消" -ForegroundColor Yellow; return }
        $source = $srcChosen
        Write-Host "正在安装: $name ($source) ..." -ForegroundColor Green
        scoop install "$source/$name"
    }
}



################### 将 vi 命令别名到 neovide ###################
function vi { if ($args) { neovide $args } else { neovide } }



################### 设置 Visual Studio 环境变量, 以便在 PowerShell 中使用 cl.exe 等工具进行开发 ###################
function vcvars {
    $MSVC_ARCH = "x64"
    $MSVC_VER = "14.50.35717"
    $SDK_VER = "10.0.26100.0"
    $VS_BASE = "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools"
    $KIT_BASE = "C:\Program Files (x86)\Windows Kits\10"

    $env:PATH += ";$VS_BASE\VC\Tools\MSVC\$MSVC_VER\bin\Host$MSVC_ARCH\$MSVC_ARCH"

    $env:INCLUDE = @(
        "$VS_BASE\VC\Tools\MSVC\$MSVC_VER\include"
        "$KIT_BASE\Include\$SDK_VER\ucrt"
        "$KIT_BASE\Include\$SDK_VER\um"
        "$KIT_BASE\Include\$SDK_VER\shared"
    ) -join ";"

    $env:LIB = @(
        "$VS_BASE\VC\Tools\MSVC\$MSVC_VER\lib\$MSVC_ARCH"
        "$KIT_BASE\Lib\$SDK_VER\ucrt\$MSVC_ARCH"
        "$KIT_BASE\Lib\$SDK_VER\um\$MSVC_ARCH"
    ) -join ";"

    Write-Host "MSVC $MSVC_VER / SDK $SDK_VER ($MSVC_ARCH) environment loaded" -ForegroundColor Green
}



################### 初始化 oh-my-posh，设置 PowerShell 提示符美化 ###################
oh-my-posh init pwsh --config C:/Users/Violet/scoop/apps/oh-my-posh/current/themes/catppuccin.omp.json  | Invoke-Expression



################### PSReadLine 配置，启用历史和插件预测 ###################
Set-PSReadLineOption -PredictionSource HistoryAndPlugin
Set-PSReadLineOption -PredictionViewStyle ListView



################### eza 替代 ls、tree 等命令 ###################
Remove-Alias -Name ls -Force -ErrorAction SilentlyContinue
Remove-Alias -Name tree -Force -ErrorAction SilentlyContinue
function ls { eza --icons @args }
function tree { eza --tree --icons @args }



################### Zoxide & fzf 终极协同配置 (PowerShell) ###################
# 检查依赖项是否安装
if ((Get-Command zoxide -ErrorAction SilentlyContinue) -and (Get-Command fzf -ErrorAction SilentlyContinue)) {
    
    # 1. 初始化 zoxide（使用 --no-cmd 避免抢占默认的 cd 别名，我们手动接管）
    Invoke-Expression (& { (zoxide init powershell --no-cmd | Out-String) })

    # 2. 移除原生与第三方可能冲突的别名
    Remove-Item Alias:cd -Force -ErrorAction SilentlyContinue
    Remove-Item Alias:zi -Force -ErrorAction SilentlyContinue

    # 3. 核心函数：智能跳转 (z)
    # 逻辑：若无参数则进入用户家目录；若参数为普通路径则直接 cd；若为模糊参数则交由 zoxide 匹配
    function z {
        [CmdletBinding()]
        param(
            [Parameter(ValueFromRemainingArguments = $true)]
            [string[]]$Path
        )
        if ($Path.Count -eq 0) {
            Set-Location ~
        } elseif ($Path.Count -eq 1 -and (Test-Path -Path $Path[0] -PathType Container)) {
            Set-Location $Path[0]
        } else {
            # 调用 zoxide 的 query 查询
            $target = zoxide query -- $Path
            if ($target) {
                Set-Location $target
            }
        }
        # 记录当前路径到 zoxide 数据库（让 zi 有更多选择）
        zoxide add (Get-Location)
    }

    # 4. 终极协同函数：交互式智能跳转 (zi)
    # 逻辑：从 zoxide 获取历史记录，通过 fzf 渲染并带 eza 目录树预览，最后执行 Set-Location
    function zi {
        # 配置 fzf 预览行为
        # 使用 eza 展示该目录下的结构，限制深度为 2，开启颜色
        $previewCmd = "eza --tree --level=2 --color=always --icons=always --group-directories-first {1}"
        
        # 若系统中未安装 eza，降级使用 PowerShell 原生的 Get-ChildItem 预览
        if (-not (Get-Command eza -ErrorAction SilentlyContinue)) {
            $previewCmd = "powershell -NoProfile -Command `"Get-ChildItem -Path `{1} -ErrorAction SilentlyContinue | Select-Object -First 15 | Format-Table Name, LastWriteTime`""
        }

        # 调用 fzf 并捕获输出
        # zoxide query -l 输出格式为: 纯路径 (例如: "C:\Users\Src")
        # fzf 参数说明:
        # --no-sort: 保持 zoxide 的 Frecency 原始排序
        # --tac: 倒序排列，使最高分排在输入框附近
        # --preview: 实时执行预览命令，{1} 表示取 fzf 输入行的第一列（即路径部分）
        $selected = zoxide query -l | fzf `
            --no-sort `
            --height=45% `
            --layout=reverse `
            --border=sharp `
            --prompt="JUMP > " `
            --preview=$previewCmd `
            --preview-window="right:50%" `
            --bind="ctrl-space:toggle-preview"

        if ($selected) {
            # 解析选中的行，提取实际路径部分（zoxide query -l 输出纯路径，无分数）
            $targetPath = $selected.Trim()
            if (Test-Path -Path $targetPath -PathType Container) {
                Set-Location $targetPath
            } else {
                Write-Warning "路径不存在或已被移动: $targetPath"
            }
        }
    }

    # 5. 重建别名绑定
    Set-Alias -Name cd -Value z -Force
    Set-Alias -Name cdi -Value zi -Force
} else {
    Write-Host "[Warning] 未检测到 zoxide 或 fzf，请先使用 Scoop 或 Winget 安装。" -ForegroundColor Yellow
}



################### 启动横幅 ###################
Write-Host @"
        🐱  ~  Meow, welcome $(whoami)  ~  🐱

        ╭─ PowerShell 7 ready ─────────────────────╮
        │  hit i   → scoop install                 │
        │  hit s   → scoop search                  │
        │  si      → search & install (fzf)        │
        │  z       → smart cd (zoxide)             │
        │  zi/cdi  → interactive jump (fzf+eza)    │
        │  ls/tree → eza with icons                │
        │  vi      → neovide                       │
        │  vcvars  → load MSVC/SDK env             │
        ╰──────────────────────────────────────────╯
"@
