use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::app::{self, ScreenStates};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn render(terminal: &mut Tui, app: &app::App) {
    terminal.draw(|frame| draw_frame(frame, app)).unwrap();
}

fn draw_frame(frame: &mut Frame, app: &app::App) {
    match app.screen_state {
        ScreenStates::Playing => draw_playing_screen(frame, app),
        ScreenStates::GameEndScreen => {}
    }
}

fn draw_playing_screen(frame: &mut Frame, app: &app::App) {
    let game_area = centered_rect(10, 40, frame.size());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title chunk
            Constraint::Length(2), // padding
            // attempts chunks with one padding after each
            Constraint::Length(12),
            // user input chunks
            Constraint::Length(1),
        ])
        .split(game_area);

    let title = Paragraph::new(Text::styled(
        "Termdle",
        Style::default().fg(Color::Red).bold(),
    ))
    .alignment(Alignment::Center);

    frame.render_widget(title, chunks[0]);

    let attempts_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(chunks[2]);
    for i in 0..6 {
        let attempt_line = Paragraph::new(Text::styled(
            "_ _ _ _ _",
            Style::default().fg(Color::Gray).bold(),
        ))
        .alignment(Alignment::Center);
        frame.render_widget(attempt_line, attempts_chunks[i * 2]);
    }

    let input_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(1), Constraint::Fill(1)])
        .split(chunks[3]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let valid_word_status = Paragraph::new(Text::styled(
        if app.current_input_valid {
            "✓"
        } else {
            "✗"
        },
        Style::default().fg(if app.current_input_valid {
            Color::Green
        } else {
            Color::Red
        }),
    )).bold()
    .alignment(Alignment::Right);

    frame.render_widget(valid_word_status, input_chunks[0]);
    let attempt_line = Paragraph::new(Text::styled(
        "t e s t s",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Center);
    frame.render_widget(attempt_line, input_chunks[1]);
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
///
/// curtesy of the Ratatui docs
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
