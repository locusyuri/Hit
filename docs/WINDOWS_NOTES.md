# Windows 注意事项

## 1. Windows 符号链接权限

- **必须使用符号链接**：版本切换功能依赖符号链接，硬链接和复制不支持
- **安装时自动申请权限**：首次安装时检测权限，若无开发者模式则提示开启
- **权限检测与引导**：

```rust
use std::os::windows::fs::symlink_dir;

fn ensure_symlink_permission() -> Result<()> {
    let test_dir = tempdir().unwrap();
    let test_link = test_dir.path().join("test_link");

    match symlink_dir(test_dir.path(), &test_link) {
        Ok(_) => {
            let _ = std::fs::remove_dir(test_link);
            Ok(())
        },
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            anyhow!("检测到符号链接权限不足。\n\n"
                "请选择以下方案：\n"
                "1. 开启 Windows 开发者模式（推荐）\n"
                "2. 以管理员身份运行 hit（仅首次安装）\n"
                "3. 使用 'hit config set link_mode hardlink' 降级（无法版本切换）")
        },
        Err(e) => anyhow!(e),
    }
}
```

- **提权安装**：若检测到无权限，可自动请求 UAC 提权（仅首次）

```rust
// 检测是否以管理员身份运行
fn is_admin() -> bool {
    unsafe { windows::Win32::Security::IsUserAnAdmin() }.unwrap_or(false)
}

// 若未提权，重新以管理员启动
if !is_admin() {
    let mut cmd = Command::new("powershell");
    cmd.args([
        "-Command",
        &format!("Start-Process hit.exe -ArgumentList '{}' -Verb RunAs", args.join(" ")),
    ]);
    cmd.spawn()?;
    std::process::exit(0);
}
```

## 2. PATH 环境变量刷新

修改 `HKCU\Environment` 后需广播消息：

```rust
use windows::Win32::UI::WindowsAndMessaging::{SendMessageTimeoutW, HWND_BROADCAST, WM_SETTINGCHANGE};

unsafe {
    SendMessageTimeoutW(
        HWND_BROADCAST,
        WM_SETTINGCHANGE,
        0,
        LPARAM("Environment\0".as_ptr() as isize),
        SMTO_ABORTIFHUNG,
        5000,
        &mut result,
    );
}
```

## 3. 深度卸载需要提权

- 深度卸载功能需管理员权限，自动检测并请求 UAC
- 普通卸载（Hit 安装的软件）无需提权

## 4. 安全性注意事项

- **软件签名验证**：建议启用签名验证，确保软件来源可信
- **权限管理**：最小权限原则，仅在必要时请求管理员权限
- **隐私保护**：不收集用户使用数据，配置文件本地存储
- **安全扫描**：集成 VirusTotal 等安全扫描服务（可选）
- **插件安全**：插件执行前进行安全检查，限制敏感操作权限

---

> 详见 [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) — 项目结构与模块说明
