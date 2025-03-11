#![allow(dead_code, unused_imports)]
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        Color, Modifier, Style, Stylize,
        palette::tailwind::{BLUE, GREEN, SLATE},
    },
    symbols,
    text::{Line, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
};

use clap::Parser;

use aws_logs_tui::aws;

const FUNCTION_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// AWS Profile to use
    #[arg(short, long)]
    profile: Option<String>,

    /// AWS Region to use
    #[arg(short, long)]
    region: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let config = aws::config::load_config(cli.profile, cli.region).await;

    let lambda_client = aws::lambda::Client::new(&config);
    let lambda_functions = lambda_client.get_all_functions().await;

    println!("Found [{}] lambda functions:", lambda_functions.len());
    for function in &lambda_functions {
        println!("{}", function.name)
    }

    // TODO The app should load the function names itself? Or do we treat this
    // as a static list? Or do we offer an option to refresh? Or automatically
    // refresh?
    let app = App {
        function_list: {
            FunctionList {
                functions: Some(lambda_functions),
                state: ListState::default(),
            }
        },
        ..Default::default()
    };

    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
struct FunctionList {
    functions: Option<Vec<aws::lambda::Function>>,
    state: ListState,
}

#[derive(Debug, Default)]
struct App {
    function_list: FunctionList,
    should_exit: bool,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key)
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            _ => {}
        }
    }

    fn select_none(&mut self) {
        self.function_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.function_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.function_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.function_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.function_list.state.select_last();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("AWS Logs TUI")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(FUNCTION_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `functions` and stylize them.
        let functions: Vec<ListItem> = self
            .function_list
            .functions
            .clone()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, function)| {
                let color = alternate_colors(i);
                ListItem::from(ListItemFunction(function.clone())).bg(color)
            })
            .collect();

        // Create a List from all list functions and highlight the currently selected one
        let list = List::new(functions)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.function_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.function_list.state.selected() {
            match &self.function_list.functions {
                None => "No functions available...".to_string(),
                Some(functions) => functions[i].name.clone(),
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the function's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Function Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(FUNCTION_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

struct ListItemFunction(aws::lambda::Function);

impl From<&ListItemFunction> for ListItem<'_> {
    fn from(value: &ListItemFunction) -> Self {
        let line = Line::styled(value.0.name.clone(), TEXT_FG_COLOR);
        ListItem::new(line)
    }
}

impl From<ListItemFunction> for Text<'_> {
    fn from(value: ListItemFunction) -> Self {
        let line = Line::styled(value.0.name, TEXT_FG_COLOR);
        Text::from(line)
    }
}
