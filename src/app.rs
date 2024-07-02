use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    ui,
    words::{self, LetterMatch},
};

/// Enum holding current app state
pub enum ScreenStates {
    /// The user is currently playing
    Playing,
    /// The game has ended and the user is choosing exit or new round
    GameEnd(bool),
}

pub struct App {
    exit: bool,
    pub words: words::Words,
    pub screen_state: ScreenStates,
    /// The word user is currently typing
    pub input: String,
    /// State of current input word, valid if found in wordlist
    pub current_input_valid: bool,
    /// Attempted words user has tried
    pub attempts: [Option<(String, [LetterMatch; 5])>; 6],
    attempts_made: u8,
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
            attempts_made: 0,
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
                ScreenStates::GameEnd(_) => self.handle_gameend_screen(key),
            }
        }
    }

    fn handle_playing(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if ('a'..='z').contains(&c) => {
                if self.input.len() < 5 {
                    self.input.push(c);
                }
            }
            KeyCode::Backspace => {
                if self.input.len() > 0 {
                    self.input.pop();
                }
            }
            KeyCode::Enter => 'enter: {
                if !self.current_input_valid || self.attempts_made >= 6 {
                    break 'enter;
                }

                let lettermatch = self.words.check_word(&self.input);
                if lettermatch.iter().eq([LetterMatch::Correct; 5].iter()) {
                    self.screen_state = ScreenStates::GameEnd(true);
                    break 'enter;
                }

                self.attempts[self.attempts_made as usize] =
                    Some((self.input.clone(), lettermatch));
                self.attempts_made += 1;
                self.input = "".to_string();

                if self.attempts_made >= 6 {
                    self.screen_state = ScreenStates::GameEnd(false);
                }
            }
            _ => {}
        }

        self.current_input_valid = self.words.is_word_valid(&self.input);
    }

    /// Handle when playing is finished playing the game
    /// either win or loss
    ///
    /// Now we ask the player if he wants to start a new game or exit (y/n)
    fn handle_gameend_screen(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y' | 'Y') => {
                self.attempts = Default::default();
                self.attempts_made = 0;
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
