use rusty_rich::{Cell, Column, Console, Table};

use crate::output::header_style;

const MAX_DESC_WIDTH: usize = 40;

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        s.chars().take(max_len - 1).collect::<String>() + "…"
    }
}

pub struct SearchRow {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub struct ListRow {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub bucket: String,
    pub install_date: String,
    pub held: String,
}

pub struct CacheRow {
    pub app: String,
    pub version: String,
    pub size: String,
    pub path: String,
}

pub struct BucketRow {
    pub name: String,
    pub manifests: String,
    pub description: String,
}

pub fn print_search_table(rows: &[SearchRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let mut console = Console::new();
    let mut table = Table::new();

    let header_style = header_style();
    table.add_column(Column::new("名称").style(header_style.clone()));
    table.add_column(Column::new("版本").style(header_style.clone()));
    table.add_column(Column::new("描述").style(header_style));

    for row in rows {
        table.add_row(vec![
            Cell::from(row.name.clone()),
            Cell::from(row.version.clone()),
            Cell::from(truncate(&row.description, MAX_DESC_WIDTH)),
        ]);
    }

    console.println(&table);
    console.print_str(title);
    console.print_str("\n");
}

pub fn print_list_table(rows: &[ListRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let mut console = Console::new();
    let mut table = Table::new();

    let header_style = header_style();
    table.add_column(Column::new("名称").style(header_style.clone()));
    table.add_column(Column::new("版本").style(header_style.clone()));
    table.add_column(Column::new("架构").style(header_style.clone()));
    table.add_column(Column::new("Bucket").style(header_style.clone()));
    table.add_column(Column::new("安装时间").style(header_style.clone()));
    table.add_column(Column::new("状态").style(header_style));

    for row in rows {
        table.add_row(vec![
            Cell::from(row.name.clone()),
            Cell::from(row.version.clone()),
            Cell::from(row.architecture.clone()),
            Cell::from(row.bucket.clone()),
            Cell::from(row.install_date.clone()),
            Cell::from(row.held.clone()),
        ]);
    }

    console.println(&table);
    console.print_str(title);
    console.print_str("\n");
}

pub fn print_cache_table(rows: &[CacheRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let mut console = Console::new();
    let mut table = Table::new();

    let header_style = header_style();
    table.add_column(Column::new("软件").style(header_style.clone()));
    table.add_column(Column::new("版本").style(header_style.clone()));
    table.add_column(Column::new("大小").style(header_style.clone()));
    table.add_column(Column::new("路径").style(header_style));

    for row in rows {
        table.add_row(vec![
            Cell::from(row.app.clone()),
            Cell::from(row.version.clone()),
            Cell::from(row.size.clone()),
            Cell::from(row.path.clone()),
        ]);
    }

    console.println(&table);
    console.print_str(title);
    console.print_str("\n");
}

pub fn print_bucket_table(rows: &[BucketRow], title: &str) {
    if rows.is_empty() {
        println!("{title}");
        return;
    }

    let mut console = Console::new();
    let mut table = Table::new();

    let header_style = header_style();
    table.add_column(Column::new("名称").style(header_style.clone()));
    table.add_column(Column::new("Manifest").style(header_style.clone()));
    table.add_column(Column::new("描述").style(header_style));

    for row in rows {
        table.add_row(vec![
            Cell::from(row.name.clone()),
            Cell::from(row.manifests.clone()),
            Cell::from(row.description.clone()),
        ]);
    }

    console.println(&table);
    console.print_str(title);
    console.print_str("\n");
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
