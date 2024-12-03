use crate::watchlist::Watchlist;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph, Table},
    Terminal,
};
use tui_input::{backend::crossterm::EventHandler, Input};
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

struct App {
    selected_option: usize,
    options: Vec<&'static str>,
    input: Input,
    watchlist: Watchlist,
}

impl App {
    fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec!["Add Stock", "Remove Stock", "View Watchlist", "Refresh Data", "Exit"],
            input: Input::default(),
            watchlist: Watchlist::new(),
        }
    }

    fn next_option(&mut self) {
        if self.selected_option < self.options.len() - 1 {
            self.selected_option += 1;
        }
    }

    fn previous_option(&mut self) {
        if self.selected_option > 0 {
            self.selected_option -= 1;
        }
    }
}

pub async fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut last_action_message = String::new();

    loop {
        let watchlist_rows = if app.selected_option == 2 {
            app.watchlist.get_cached_data()
        } else {
            vec![]
        };

        terminal.draw(|f| {
            let layout_constraints = vec![
                Constraint::Percentage(20), // Menu
                Constraint::Percentage(60), // Main Content or Watchlist Table
                Constraint::Percentage(20), // Last Action
            ];

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(layout_constraints)
                .split(f.area());

            // Render menu
            let menu_items: Vec<ListItem> = app
                .options
                .iter()
                .enumerate()
                .map(|(i, option)| {
                    let style = if i == app.selected_option {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(Span::styled(*option, style))
                })
                .collect();
            let menu = List::new(menu_items)
                .block(Block::default().borders(Borders::ALL).title("Options"));
            f.render_widget(menu, chunks[0]);

            // Render main content
            if app.selected_option == 2 {
                // View Watchlist: Render Watchlist Table
                let rows = watchlist_rows.into_iter().map(|(symbol, open, close, change, ema)| {
                    ratatui::widgets::Row::new(vec![
                        Span::raw(symbol),
                        Span::raw(open),
                        Span::raw(close),
                        Span::raw(change),
                        Span::raw(ema), // Display the 5 most recent EMA values
                    ])
                });
                let widths = [
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(40),
                ];
                let table = Table::new(rows, widths)
                    .header(
                        ratatui::widgets::Row::new(vec!["Symbol", "Open", "Close", "Change (%)", "EMA (Last 5)"])
                            .style(Style::default().fg(Color::Yellow)),
                    )
                    .block(Block::default().borders(Borders::ALL).title("Watchlist"));
                f.render_widget(table, chunks[1]);
                
            } else if app.selected_option == 0 || app.selected_option == 1 {
                // Add/Remove Stock: Render Input Box
                let title = if app.selected_option == 0 { "Add Stock" } else { "Remove Stock" };
                let content = Paragraph::new(app.input.value())
                    .block(Block::default().borders(Borders::ALL).title(title));
                f.render_widget(content, chunks[1]);
            } else if app.selected_option == 3 {
                // Refresh Data: Render Refresh Message
                let content = Paragraph::new("Press Enter to refresh stock data.")
                    .block(Block::default().borders(Borders::ALL).title("Refresh Data"));
                f.render_widget(content, chunks[1]);
            } else {
                // Exit: Render Exit Message
                let content = Paragraph::new("Press Enter to exit the program.")
                    .block(Block::default().borders(Borders::ALL).title("Exit"));
                f.render_widget(content, chunks[1]);
            }

            // Render Last Action box
            let last_action_box = Paragraph::new(last_action_message.as_str())
                .block(Block::default().borders(Borders::ALL).title("Last Action"));
            f.render_widget(last_action_box, chunks[2]);
        })?;

        // Handle input events
        if let CEvent::Key(key) = event::read()? {
            match key {
                KeyEvent { code: KeyCode::Up, kind: event::KeyEventKind::Press, .. } => app.previous_option(),
                KeyEvent { code: KeyCode::Down, kind: event::KeyEventKind::Press, .. } => app.next_option(),
                KeyEvent { code: KeyCode::Enter, kind: event::KeyEventKind::Press, .. } => match app.selected_option {
                    0 => {
                        // Add Stock
                        let trimmed_symbol = app.input.value().trim().to_string();
                        if trimmed_symbol.is_empty() {
                            last_action_message = "Cannot add an empty stock symbol.".to_string();
                        } else if let Ok(message) = app.watchlist.add_stock(trimmed_symbol).await {
                            last_action_message = message;
                        } else {
                            last_action_message = "Failed to add stock.".to_string();
                        }
                        app.input.reset();
                    }
                    1 => {
                        // Remove Stock
                        let trimmed_symbol = app.input.value().trim();
                        if let Ok(message) = app.watchlist.remove_stock(trimmed_symbol) {
                            last_action_message = message;
                        } else {
                            last_action_message = "Failed to remove stock.".to_string();
                        }
                        app.input.reset();
                    }
                    3 => {
                        // Refresh Data
                        let messages = app.watchlist.refresh_data().await;
                        last_action_message = messages.join("\n");
                    }
                    4 => break, // Exit
                    _ => {}
                },
                _ => {
                    app.input.handle_event(&CEvent::Key(key));
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

