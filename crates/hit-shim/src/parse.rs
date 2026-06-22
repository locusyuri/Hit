//! `.shim` sidecar 文件解析器
//!
//! 兼容 Scoop `.shim` 格式：
//! ```text
//! path = "C:\Users\...\apps\git\current\cmd\git.exe"
//! args = --some-flag --other
//! ```

use std::fmt;
use std::path::{Path, PathBuf};

/// 解析后的 shim 数据
#[derive(Debug, Clone)]
pub struct ShimData {
    /// 目标可执行文件的绝对路径
    pub path: String,
    /// 预置参数（来自 .shim 文件的 args 行）
    pub args: Vec<String>,
}

/// shim 解析错误
#[derive(Debug)]
pub enum ShimError {
    /// 缺少必需的 `path` 行
    MissingPath,
    /// `.shim` 文件读取失败
    IoError(std::io::Error),
}

impl fmt::Display for ShimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShimError::MissingPath => write!(f, ".shim 文件缺少 path 行"),
            ShimError::IoError(e) => write!(f, "读取 .shim 文件失败: {e}"),
        }
    }
}

/// 解析 `.shim` 文件内容
pub fn parse_shim(content: &str) -> Result<ShimData, ShimError> {
    let mut path = None;
    let mut args = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = strip_key(line, "path") {
            path = Some(unquote(rest));
        } else if let Some(rest) = strip_key(line, "args") {
            args = split_args(rest);
        }
    }

    match path {
        Some(p) => Ok(ShimData { path: p, args }),
        None => Err(ShimError::MissingPath),
    }
}

/// 根据 shim exe 路径推导对应 `.shim` 文件路径
///
/// `C:\...\shims\git.exe` → `C:\...\shims\git.shim`
pub fn shim_file_path(exe_path: &Path) -> PathBuf {
    exe_path.with_extension("shim")
}

/// 读取并解析 `.shim` 文件
pub fn read_shim_file(path: &Path) -> Result<ShimData, ShimError> {
    let content = std::fs::read_to_string(path).map_err(ShimError::IoError)?;
    parse_shim(&content)
}

/// 去除 `key = value` 格式中的 `key = ` 前缀
fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(key)?;
    let rest = rest.trim_start();
    let rest = rest.strip_prefix('=')?;
    Some(rest.trim_start())
}

/// 去除首尾引号
fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// 拆分参数行（尊重引号）
fn split_args(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }

    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = '"';

    for ch in s.chars() {
        match ch {
            '"' | '\'' if !in_quote => {
                in_quote = true;
                quote_char = ch;
            }
            c if c == quote_char && in_quote => {
                in_quote = false;
            }
            ' ' | '\t' if !in_quote => {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_path_and_args() {
        let content = r#"path = "C:\Users\test\apps\git\current\cmd\git.exe"
args = --no-pager"#;
        let data = parse_shim(content).unwrap();
        assert_eq!(data.path, r"C:\Users\test\apps\git\current\cmd\git.exe");
        assert_eq!(data.args, vec!["--no-pager"]);
    }

    #[test]
    fn parse_path_only() {
        let content = r#"path = "C:\tools\node.exe""#;
        let data = parse_shim(content).unwrap();
        assert_eq!(data.path, r"C:\tools\node.exe");
        assert!(data.args.is_empty());
    }

    #[test]
    fn parse_path_with_spaces() {
        let content = r#"path = "C:\Program Files\Git\cmd\git.exe""#;
        let data = parse_shim(content).unwrap();
        assert_eq!(data.path, r"C:\Program Files\Git\cmd\git.exe");
    }

    #[test]
    fn parse_missing_path() {
        let content = "args = --something";
        assert!(matches!(parse_shim(content), Err(ShimError::MissingPath)));
    }

    #[test]
    fn parse_empty_file() {
        assert!(matches!(parse_shim(""), Err(ShimError::MissingPath)));
    }

    #[test]
    fn parse_args_quoted() {
        let content = r#"path = "C:\app.exe"
args = --name "hello world" --flag"#;
        let data = parse_shim(content).unwrap();
        assert_eq!(data.args, vec!["--name", "hello world", "--flag"]);
    }

    #[test]
    fn shim_file_path_derivation() {
        let exe = Path::new(r"C:\shims\git.exe");
        assert_eq!(shim_file_path(exe), PathBuf::from(r"C:\shims\git.shim"));
    }
}
