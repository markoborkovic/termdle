use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame, Terminal,
};

use crate::{
    app::{self, ScreenStates},
    words::LetterMatch,
};

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
        ScreenStates::GameEnd(gameend_status) => draw_gameend_screen(frame, app, gameend_status),
    }
}

fn draw_playing_screen(frame: &mut Frame, app: &app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title chunk
            Constraint::Length(2), // padding
            // attempts chunks with one padding after each
            Constraint::Length(12),
            // user input chunks
            Constraint::Length(1),
            // debug info
            Constraint::Length(1), // padding
            Constraint::Length(1),
        ])
        .split(frame.size());

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
        let mut line_vec = vec![];

        if let Some((attempt, lettermatch)) = &app.attempts[i] {
            for (i, c) in attempt.chars().enumerate() {
                line_vec.push(Span::raw(" "));
                line_vec.push(Span::styled(
                    c.to_string().to_ascii_uppercase(),
                    match lettermatch[i] {
                        LetterMatch::Correct => Style::default().fg(Color::Green).bold(),
                        LetterMatch::Partial => Style::default().fg(Color::Yellow),
                        LetterMatch::Incorrect => Style::default().fg(Color::DarkGray),
                    },
                ));
            }
        } else {
            line_vec.push(Span::styled(
                " _ _ _ _ _",
                Style::default().fg(Color::DarkGray),
            ));
        }

        let attempt_line = Line::from(line_vec).alignment(Alignment::Center);
        frame.render_widget(attempt_line, attempts_chunks[i * 2]);
    }

    let input_verifiction_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(15),
            Constraint::Fill(1),
        ])
        .split(chunks[3])[0];

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
    ))
    .alignment(Alignment::Right)
    .bold();

    frame.render_widget(valid_word_status, input_verifiction_chunk);

    let mut modified_input = app.input.clone().to_ascii_uppercase();
    if modified_input.len() < 5 {
        modified_input.push_str(&"_".repeat(5 - modified_input.len()));
    }
    let input_line_text = modified_input
        .chars()
        .map(String::from)
        .collect::<Vec<_>>()
        .join(" ");

    let input_line = Paragraph::new(Text::styled(
        input_line_text,
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Center);
    frame.render_widget(input_line, chunks[3]);

    if app.debug_mode {
        let debug_info = Paragraph::new(Text::styled(
            format!("Dbg: \"{}\"", app.words.chosen_word.unwrap_or("None")),
            Style::default().fg(Color::Yellow),
        ));
        frame.render_widget(debug_info, chunks[chunks.len() - 1]);
    }
}

fn draw_gameend_screen(frame: &mut Frame, app: &app::App, gameend_status: bool) {
    let result: Paragraph;
    if gameend_status {
        result = Paragraph::new(format!(
            "Correct! The word was \"{}\"! Do you want to start a new game? (y/n)",
            &app.words.chosen_word.unwrap()
        ))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    } else {
        result = Paragraph::new(format!(
            "Incorrect! The word was \"{}\"! Do you want to start a new game? (y/n)",
            &app.words.chosen_word.unwrap()
        ))
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center);
    }
    frame.render_widget(
        result,
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(frame.size())[1],
    );
}
