use std::io::{self, stdout};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{ui, words};

/// Enum holding current app state
pub enum AppStates {
    /// The user is currently playing
    Playing,
    /// The user is choosing exit or new round
    ChoosingNext,
}

pub struct App {
    app_state: AppStates,
    exit: bool,
    /// State of current input word, valid if found in wordlist
    current_input_valid: bool,
    words: words::Words,
    attempts: [String; 6],
    input: String,
    debug_mode: bool,
}

impl App {
    pub fn new(debug_mode: bool) -> Self {
        let mut instance = Self {
            app_state: AppStates::Playing,
            exit: false,
            current_input_valid: false,
            words: words::Words::new(),
            attempts: Default::default(),
            input: "".to_string(),
            debug_mode: debug_mode,
        };
        instance.words.choose_word();

        instance
    }

    pub fn run(&mut self, terminal: &mut ui::Tui) -> io::Result<()> {
        while !self.exit {
            ui::render(terminal, self);
            self.handle_events();
        }

        Ok(())
    }

    fn handle_events(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Release {
                return;
            }

            if key.code == KeyCode::Esc
                || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
            {
                self.exit = true;
                return;
            }

            match self.app_state {
                AppStates::Playing => self.handle_playing(key),
                AppStates::ChoosingNext => self.handle_choosing_next(key),
            }
        }
    }

    fn handle_playing(&mut self, key: KeyEvent) {}

    /// Handle when playing is finished playing the game
    /// either win or loss
    ///
    /// Now we ask the player if he wants to start a new game or exit (y/n)
    fn handle_choosing_next(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y' | 'Y') => {
                self.attempts = Default::default();
                self.input = "".to_string();
                self.current_input_valid = false;

                self.words.choose_word();
                self.app_state = AppStates::Playing;
            }
            KeyCode::Char('n' | 'N') => {
                self.exit = true;
            }
            _ => {}
        }
    }
}
