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
use std::time::{Duration, Instant};

struct App {
    selected_option: usize,
    options: Vec<&'static str>,
    input: Input,
    watchlist: Watchlist,
    startup_time: Instant,
}

impl App {
    fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec!["Add Stock", "Remove Stock", "View Watchlist", "Exit"],
            input: Input::default(),
            watchlist: Watchlist::new(),
            startup_time: Instant::now(),
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

    /// Dynamically update the header title for chunks[1]
    fn get_header_title(&self) -> &str {
        match self.selected_option {
            0 => "Add Stock",
            1 => "Remove Stock",
            2 => "Watchlist",
            _ => "Exit",
        }
    }

    /// Check if the program is ready to accept inputs
    fn is_ready(&self) -> bool {
        self.startup_time.elapsed() > Duration::from_secs(1)
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
        // Prepare watchlist data if viewing it
        let watchlist_rows = if app.selected_option == 2 {
            println!("DEBUG: Watchlist: {:?}", app.watchlist.get_cached_data()); // Debug print
            app.watchlist.get_cached_data()
        } else {
            vec![]
        };

        // Render UI
        terminal.draw(|f| {
            let layout_constraints = if app.selected_option == 2 {
                vec![
                    Constraint::Percentage(20), // Menu
                    Constraint::Percentage(40), // Watchlist Table
                    Constraint::Percentage(20), // Refresh Message
                    Constraint::Percentage(20), // Last Action
                ]
            } else {
                vec![
                    Constraint::Percentage(20), // Menu
                    Constraint::Percentage(60), // Main Content
                    Constraint::Percentage(20), // Last Action
                ]
            };

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

            if app.selected_option == 2 {
                // Render Watchlist Table
                let rows = watchlist_rows.into_iter().map(|(symbol, price, change)| {
                    ratatui::widgets::Row::new(vec![
                        Span::raw(symbol),
                        Span::raw(price),
                        Span::raw(change),
                    ])
                });
                let widths = &[Constraint::Length(10), Constraint::Length(10), Constraint::Length(10)];
                let table = Table::new(rows, widths)
                    .header(
                        ratatui::widgets::Row::new(vec!["Symbol", "Price", "Change (%)"])
                            .style(Style::default().fg(Color::Yellow)),
                    )
                    .block(Block::default().borders(Borders::ALL).title("Watchlist"));
                f.render_widget(table, chunks[1]);

                // Render Refresh Message
                let refresh_message = Paragraph::new("Press Enter to refresh stock data.")
                    .block(Block::default().borders(Borders::ALL).title("Refresh Stock Data"));
                f.render_widget(refresh_message, chunks[2]);
            } else {
                // Render content for other options
                let header_title = app.get_header_title();
                let content_message = match app.selected_option {
                    0 => app.input.value().to_string(),
                    1 => app.input.value().to_string(),
                    3 => "Press Enter to close the program.".to_string(),
                    _ => "".to_string(),
                };

                let content = Paragraph::new(content_message)
                    .block(Block::default().borders(Borders::ALL).title(header_title));
                f.render_widget(content, chunks[1]);
            }

            // Render Last Action box
            let last_action_box = Paragraph::new(last_action_message.as_str())
                .block(Block::default().borders(Borders::ALL).title("Last Action"));
            let last_action_chunk_index = if app.selected_option == 2 { 3 } else { 2 };
            f.render_widget(last_action_box, chunks[last_action_chunk_index]);
        })?;

        // Handle input events
        if app.is_ready() {
            if let CEvent::Key(key) = event::read()? {
                match key {
                    KeyEvent { code: KeyCode::Up, kind: event::KeyEventKind::Press, .. } => app.previous_option(),
                    KeyEvent { code: KeyCode::Down, kind: event::KeyEventKind::Press, .. } => app.next_option(),
                    KeyEvent { code: KeyCode::Enter, kind: event::KeyEventKind::Press, .. } => match app.selected_option {
                        0 => {
                            let raw_input = app.input.value().to_string();
                            let trimmed_symbol = raw_input.trim();
                            println!("DEBUG: Raw Input: '{}', Trimmed Symbol: '{}'", raw_input, trimmed_symbol); // Debug print
                            
                            if trimmed_symbol.is_empty() {
                                last_action_message = "Cannot add an empty stock symbol.".to_string();
                            } else if let Ok(message) = app.watchlist.add_stock(trimmed_symbol.to_string()).await {
                                last_action_message = message;
                            } else {
                                last_action_message = format!("Failed to add stock {}.", trimmed_symbol);
                            }
                            app.input.reset();
                        }

                        1 => {
                            if let Ok(message) = app.watchlist.remove_stock(app.input.value().trim()) {
                                last_action_message = message;
                            } else {
                                last_action_message = format!("Failed to remove stock {}.", app.input.value());
                            }
                            app.input.reset();
                        }
                        2 => {
                            let messages = app.watchlist.refresh_data().await;
                            last_action_message = messages.join("\n");
                        }
                        3 => break,
                        _ => {}
                    },
                    KeyEvent { code: KeyCode::Esc, .. } => break,
                    _ => {
                        app.input.handle_event(&CEvent::Key(key));
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
