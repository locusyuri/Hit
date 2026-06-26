//! `hit which` — 查找命令对应的 shim 路径和真实 exe 路径

use clap::Args as ClapArgs;
use colored::Colorize;
use hit_common::Session;

/// which 参数
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// 命令名称（如 git、python）
    pub command: String,
}

/// 执行查找
pub fn execute(args: &Args, session: &Session) -> anyhow::Result<()> {
    let shim_path = session.shims_path().join(format!("{}.shim", args.command));

    if !shim_path.exists() {
        return Err(anyhow::anyhow!(
            "未找到 '{}' 的 shim 文件",
            args.command
        ));
    }

    let shim_data = hit_shim::parse::read_shim_file(&shim_path).map_err(|e| {
        anyhow::anyhow!("解析 {} 失败: {e}", shim_path.display())
    })?;

    println!("{}:   {}", "Shim".bold(), shim_path.display());
    println!("{}: {}", "Target".bold(), shim_data.path);

    if !shim_data.args.is_empty() {
        println!("{}:  {}", "Args".bold(), shim_data.args.join(" "));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;

    fn test_session(dir: &std::path::Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    #[test]
    fn which_not_found_errors() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("shims")).unwrap();
        let session = test_session(dir.path());
        let args = Args {
            command: "nonexistent".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到"));
    }

    #[test]
    fn which_with_valid_shim() {
        let dir = tempfile::tempdir().unwrap();
        let shims_dir = dir.path().join("shims");
        std::fs::create_dir_all(&shims_dir).unwrap();

        // 创建 .shim 文件
        std::fs::write(
            shims_dir.join("git.shim"),
            r#"path = "C:\test\git.exe""#,
        )
        .unwrap();

        let session = test_session(dir.path());
        let args = Args {
            command: "git".into(),
        };

        let result = execute(&args, &session);
        assert!(result.is_ok());
    }
}
