use std::io::{self, stdout};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{ui, words};

/// Enum holding current app state
pub enum ScreenStates {
    /// The user is currently playing
    Playing,
    /// The game has ended and the user is choosing exit or new round
    GameEndScreen,
}

pub struct App {
    exit: bool,
    words: words::Words,
    pub screen_state: ScreenStates,
    /// The word user is currently typing
    pub input: String,
    /// State of current input word, valid if found in wordlist
    pub current_input_valid: bool,
    /// Attempted words user has tried
    pub attempts: [String; 6],
    pub debug_mode: bool,
}

impl App {
    pub fn new(debug_mode: bool) -> Self {
        let mut instance = Self {
            screen_state: ScreenStates::Playing,
            exit: false,
            current_input_valid: false,
            words: words::Words::new(),
            attempts: Default::default(),
            input: "".to_string(),
            debug_mode,
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

            match self.screen_state {
                ScreenStates::Playing => self.handle_playing(key),
                ScreenStates::GameEndScreen => self.handle_choosing_next(key),
                ScreenStates::GameEndScreen => todo!(),
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
                self.screen_state = ScreenStates::Playing;
            }
            KeyCode::Char('n' | 'N') => {
                self.exit = true;
            }
            _ => {}
        }
    }
}
