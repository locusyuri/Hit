$hit = 'C:\Users\Violet\Downloads\test\hit\hit.exe'
& $hit bucket add main 2>&1 | Where-Object { $_ -notmatch 'WARN' }
& $hit bucket list 2>&1 | Where-Object { $_ -notmatch 'WARN' }
