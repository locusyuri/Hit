//! 共享表格行类型和渲染辅助函数
//!
//! 使用 `tabled` crate 渲染表格，列宽自适应、对齐整齐。

use tabled::{builder::Builder, Table, Tabled};

/// 搜索结果行
#[derive(Tabled, Clone)]
pub struct SearchRow {
    #[tabled(rename = "名称")]
    pub name: String,
    #[tabled(rename = "版本")]
    pub version: String,
    #[tabled(rename = "描述")]
    pub description: String,
}

/// 已安装软件行
#[derive(Tabled, Clone)]
pub struct ListRow {
    #[tabled(rename = "名称")]
    pub name: String,
    #[tabled(rename = "版本")]
    pub version: String,
    #[tabled(rename = "架构")]
    pub architecture: String,
    #[tabled(rename = "Bucket")]
    pub bucket: String,
    #[tabled(rename = "安装时间")]
    pub install_date: String,
    #[tabled(rename = "状态")]
    pub held: String,
}

/// 缓存文件行
#[derive(Tabled, Clone)]
pub struct CacheRow {
    #[tabled(rename = "软件")]
    pub app: String,
    #[tabled(rename = "版本")]
    pub version: String,
    #[tabled(rename = "大小")]
    pub size: String,
    #[tabled(rename = "路径")]
    pub path: String,
}

/// Bucket 行
#[derive(Tabled, Clone)]
pub struct BucketRow {
    #[tabled(rename = "名称")]
    pub name: String,
    #[tabled(rename = "Manifest")]
    pub manifests: String,
    #[tabled(rename = "描述")]
    pub description: String,
}

const MAX_DESC_WIDTH: usize = 40;

/// 截断字符串到指定长度，超出部分用 `…` 替换
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        s.chars().take(max_len - 1).collect::<String>() + "…"
    }
}

/// 渲染搜索结果表格
pub fn print_search_table(rows: &[SearchRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let mut builder = Builder::default();
    builder.push_record(["名称", "版本", "描述"]);
    for row in rows {
        builder.push_record([
            &row.name,
            &row.version,
            &truncate(&row.description, MAX_DESC_WIDTH),
        ]);
    }

    let table = builder.build().to_string();
    println!("{table}");
    println!("\n{title}");
}

/// 渲染已安装软件表格
pub fn print_list_table(rows: &[ListRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let table = Table::new(rows.to_vec()).to_string();
    println!("{table}");
    println!("\n{title}");
}

/// 渲染缓存表格
pub fn print_cache_table(rows: &[CacheRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let table = Table::new(rows.to_vec()).to_string();
    println!("{table}");
    println!("\n{title}");
}

/// 渲染 Bucket 表格
pub fn print_bucket_table(rows: &[BucketRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let table = Table::new(rows.to_vec()).to_string();
    println!("{table}");
    println!("\n{title}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_search_table_with_data() {
        let rows = vec![
            SearchRow {
                name: "git".into(),
                version: "2.45.1".into(),
                description: "版本控制工具".into(),
            },
            SearchRow {
                name: "curl".into(),
                version: "8.7.1".into(),
                description: "URL 传输工具".into(),
            },
        ];
        print_search_table(&rows, "共 2 个结果");
    }

    #[test]
    fn print_search_table_empty() {
        let rows: Vec<SearchRow> = Vec::new();
        print_search_table(&rows, "没有结果");
    }
}
