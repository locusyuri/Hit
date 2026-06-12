//! Hit Shim 代理程序
//!
//! 独立的轻量级 exe（~200KB），负责命令转发：
//! 1. 接收命令行参数
//! 2. 读取 `~/.hit/db.json` 获取当前激活版本
//! 3. 拼接真实路径：`~/.hit/apps/<package>/<version>/bin/<exe>`
//! 4. 启动真实进程并转发 stdin/stdout/stderr
//! 5. 返回退出码

fn main() {
    println!("hit-shim placeholder");
}
