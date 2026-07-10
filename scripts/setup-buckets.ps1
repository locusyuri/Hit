Write-Host "=== 添加 Bucket ==="
$hit = 'C:\Users\Violet\Downloads\test\hit\hit.exe'

Write-Host "[1/3] 添加 extras..."
& $hit bucket add extras 2>&1 | Where-Object { $_ -notmatch 'WARN' }
Write-Host "[2/3] 添加 versions..."
& $hit bucket add versions 2>&1 | Where-Object { $_ -notmatch 'WARN' }
Write-Host "[3/3] 查看 bucket..."
& $hit bucket list 2>&1 | Where-Object { $_ -notmatch 'WARN' }

Write-Host ""
Write-Host "=== 快速验证 ==="
& $hit --version
& $hit search git 2>&1 | Select-Object -First 5
