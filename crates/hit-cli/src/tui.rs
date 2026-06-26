//! TUI 渲染模块
//!
//! 使用 ratatui + crossterm 实现交互式搜索界面。

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Terminal;

use hit_core::bucket::index::{PackageSummary, SoftwareIndex};

/// TUI 应用状态
pub struct App<'a> {
    /// 搜索关键词
    pub query: String,
    /// 搜索结果
    pub results: Vec<&'a PackageSummary>,
    /// 当前选中索引
    pub selected: usize,
    /// 滚动偏移
    pub scroll: usize,
    /// 是否退出
    pub should_quit: bool,
    /// 选中的结果（退出时返回）
    pub selected_result: Option<&'a PackageSummary>,
}

impl<'a> App<'a> {
    /// 创建新应用
    pub fn new(index: &'a SoftwareIndex, initial_query: &str) -> Self {
        let results = if initial_query.is_empty() {
            Vec::new()
        } else {
            index.search(initial_query)
        };

        Self {
            query: initial_query.to_string(),
            results,
            selected: 0,
            scroll: 0,
            should_quit: false,
            selected_result: None,
        }
    }

    /// 更新搜索结果
    pub fn search(&mut self, index: &'a SoftwareIndex) {
        self.results = if self.query.is_empty() {
            Vec::new()
        } else {
            index.search(&self.query)
        };
        self.selected = 0;
        self.scroll = 0;
    }

    /// 处理键盘事件
    pub fn handle_key(&mut self, key: KeyEvent, index: &'a SoftwareIndex) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') if key.modifiers == KeyModifiers::NONE => {
                self.should_quit = true;
            }
            KeyCode::Enter
                if !self.results.is_empty() && self.selected < self.results.len() =>
            {
                self.selected_result = Some(self.results[self.selected]);
                self.should_quit = true;
            }
            KeyCode::Up if self.selected > 0 => {
                self.selected -= 1;
                if self.selected < self.scroll {
                    self.scroll = self.selected;
                }
            }
            KeyCode::Down if self.selected + 1 < self.results.len() => {
                self.selected += 1;
            }
            KeyCode::Char(c) if key.modifiers == KeyModifiers::NONE => {
                self.query.push(c);
                self.search(index);
            }
            KeyCode::Backspace => {
                self.query.pop();
                self.search(index);
            }
            _ => {}
        }
    }
}

/// 运行 TUI 应用
pub fn run_app(
    session: &hit_common::Session,
    initial_query: &str,
) -> anyhow::Result<Option<String>> {
    // 构建索引
    let index = hit_core::bucket::index::build_index(session)?;

    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(&index, initial_query);

    // 事件循环
    let result = loop {
        terminal.draw(|f| draw(f, &mut app))?;

        if event::poll(Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            app.handle_key(key, &index);
            if app.should_quit {
                break app.selected_result.map(|p| p.name.clone());
            }
        }
    };

    // 恢复终端
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(result)
}

/// 绘制 UI
fn draw(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // 搜索框
            Constraint::Min(5),    // 结果列表
            Constraint::Length(2), // 状态栏
        ])
        .split(f.area());

    // 搜索框
    draw_search_box(f, app, chunks[0]);

    // 结果列表
    draw_results(f, app, chunks[1]);

    // 状态栏
    draw_status_bar(f, app, chunks[2]);
}

/// 绘制搜索框
fn draw_search_box(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let input = format!("{} ", app.query);
    let paragraph = Paragraph::new(input).block(
        Block::default()
            .title(" 搜索 ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(paragraph, area);
}

/// 绘制结果列表
fn draw_results(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(format!(" 结果 ({} 个) ", app.results.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    if app.results.is_empty() {
        let empty_msg = if app.query.is_empty() {
            "输入关键词开始搜索"
        } else {
            "未找到匹配结果"
        };
        let paragraph = Paragraph::new(empty_msg)
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app
        .results
        .iter()
        .enumerate()
        .map(|(i, pkg)| {
            let style = if i == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::styled(format!("{:<15}", pkg.name), style),
                Span::styled(format!("{:<10}", pkg.version), style),
                Span::styled(&pkg.description, style),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(block);
    let mut state = ListState::default();
    state.select(Some(app.selected));
    f.render_stateful_widget(list, area, &mut state);
}

/// 绘制状态栏
fn draw_status_bar(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let help_text = if app.results.is_empty() {
        "Esc: 退出"
    } else {
        "Enter: 安装  ↑↓: 导航  Esc: 退出"
    };

    let paragraph = Paragraph::new(Line::from(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(paragraph, area);
}
