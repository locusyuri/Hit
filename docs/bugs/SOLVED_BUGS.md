# 已解决的 Bug 记录

> 从 BUGS.md 迁移过来的已修复 Bug，保留历史记录。星星数量表示严重程度，越多越严重。

---

## 欢迎页面未在 `hit --help` 触发 ⭐

执行 `hit --help` 后没有显示欢迎页面，而是直接显示了帮助信息。
之后执行别的命令 `hit bucket list` 时，才显示了欢迎页面。

**修复**：`hit-cli/src/main.rs` — 将 Session 创建和欢迎页检查移到 `Cli::parse()` 之前，避免 clap 拦截 `--help` 后直接退出导致欢迎流程无法触发。

**提交**：`94c6d41` — fix(bugs): welcome before --help; config default_path exe fallback

---

## root 路径未写入 config.json ⭐⭐⭐⭐⭐

使用安装脚本从本地构建的文件安装 hit，并指定了安装路径：

```powershell
.\scripts\install-hit.ps1

╔══════════════════════════════════════════╗
║      Hit 安装向导                        ║
║  直接回车使用默认值，一路 Enter 即可       ║
╚══════════════════════════════════════════╝

请输入安装路径（默认: C:\Users\Violet\.hit）: C:\Users\Violet\Downloads\test\hit

  安装方式：
    1) 从 GitHub 下载（默认）
    2) 使用本地编译的 exe（开发调试）
请选择 [1/2]（默认: 1）: 2
请输入 hit.exe 路径: C:\Repos\Hit\target\release\hit.exe


╔══════════════════════════════════════════╗
║  开始安装...                             ║
╚══════════════════════════════════════════╝

[Hit] 检查 PowerShell 版本...
[Hit] 设置执行策略 RemoteSigned (CurrentUser)...
[Hit] 使用本地二进制：C:\Repos\Hit\target\release\hit.exe
[Hit] 初始化目录布局...
[Hit] hit.exe 已部署到 C:\Users\Violet\Downloads\test\hit
[Hit] 默认配置已写入 C:\Users\Violet\Downloads\test\hit\config.json
[Hit] 注册 shims 目录到用户 PATH...
[Hit] 已追加 C:\Users\Violet\Downloads\test\hit\shims 到 HKCU\Environment\Path

[Hit] Hit 安装完成！

    安装路径：C:\Users\Violet\Downloads\test\hit
    二进制  ：C:\Users\Violet\Downloads\test\hit\hit.exe
    配置    ：C:\Users\Violet\Downloads\test\hit\config.json
    Shims   ：C:\Users\Violet\Downloads\test\hit\shims

    请重新打开终端让 PATH 生效，然后运行：

        hit --help
        hit bucket add main
        hit install <package>
```

但 `C:\Users\Violet\Downloads\test\hit\config.json` 中 `root_path` 为 `null`：

```json
{
  "proxy": null,
  "mirror": null,
  "aria2_enabled": false,
  "no_junction": false,
  "root_path": null,
  "auto_cleanup_days": 30,
  "health_check_interval_days": 7
}
```

导致后续添加 main bucket 时，安装到了默认路径 `C:\Users\Violet\.hit\buckets\main`，而非指定的 `C:\Users\Violet\Downloads\test\hit\buckets\main`。

**修复**：
- `scripts/install-hit.ps1` — `root_path` 写入用户指定的安装路径（原为 `null`）；新增 `HIT_ROOT` 环境变量注册到 `HKCU\Environment`。
- `crates/hit-common/src/config.rs` — `default_path()` 增加 exe 同目录回退（无需 `HIT_ROOT` 环境变量也能找到自定义路径下的 config）。

**提交**：
- `94c6d41` — fix(bugs): welcome before --help; config default_path exe fallback
- `9c1d244` — fix(install): root_path写入config.json;注册HIT_ROOT环境变量
