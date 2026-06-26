//! Hit Shim 代理程序
//!
//! 独立的轻量级 exe，负责命令转发：
//! 1. 获取自身路径，推导同名 `.shim` 文件位置
//! 2. 读取并解析 `.shim` 文件获取目标路径和预置参数
//! 3. 拼接预置参数与命令行参数
//! 4. 启动目标进程，继承 stdin/stdout/stderr
//! 5. 返回子进程退出码

use std::process::ExitCode;

use hit_shim::parse::{read_shim_file, shim_file_path};
use hit_shim::process::run_target;

fn main() -> ExitCode {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("hit-shim: 无法获取自身路径: {e}");
            return ExitCode::FAILURE;
        }
    };

    let shim_file = shim_file_path(&exe);

    let data = match read_shim_file(&shim_file) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("hit-shim: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut all_args = data.args;
    all_args.extend(std::env::args_os().skip(1).map(|a| a.to_string_lossy().into_owned()));

    run_target(&data.path, &all_args)
}
