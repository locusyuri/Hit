Write-Host "检查安装目录..."
$testDir = 'C:\Users\Violet\Downloads\test\hit'
if (Test-Path $testDir) {
    Write-Host "目录存在: $testDir"
} else {
    Write-Host "目录不存在，将创建"
}

Write-Host "运行安装脚本..."
$installScript = 'C:\Repos\Hit\scripts\install-hit.ps1'
& $installScript -Path $testDir -FromLocal 'C:\Repos\Hit\target\release\hit.exe' -NonInteractive -Force 2>&1

Write-Host ""
Write-Host "=== 验证 ==="
$hitExe = Join-Path $testDir 'hit.exe'
if (Test-Path $hitExe) {
    Write-Host "hit.exe 已部署"
    & $hitExe --version
} else {
    Write-Host "错误: hit.exe 未找到"
}

$shimExe = Join-Path $testDir 'hit-shim.exe'
if (Test-Path $shimExe) {
    Write-Host "hit-shim.exe 已部署"
} else {
    Write-Host "错误: hit-shim.exe 未找到"
}

$shimsDirExe = Join-Path $testDir 'shims\hit.exe'
if (Test-Path $shimsDirExe) {
    $item = Get-Item $shimsDirExe
    Write-Host "shim代理已部署: $($item.Length) bytes"
} else {
    Write-Host "shim代理文件未找到"
}

Write-Host ""
Write-Host "=== 简单功能验证 ==="
& $hitExe --version
& $hitExe prefix
