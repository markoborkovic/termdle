use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend, crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}}, Frame, Terminal
};

use crate::app;

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

}
