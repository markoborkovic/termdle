use rand::Rng;

/// Compile time included wordlist "sgb-words.txt" provided by Stanford University
const WORDLIST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/sgb-words.txt"
));

/// Used for handling anything to do with words or their logic
pub struct Words {
    all_words: Vec<&'static str>,
    chosen_word: Option<&'static str>,
}

/// Status of how the letter of the user input word matches the chosen word.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LetterMatch {
    /// Letter is in the chosen word and is in the correct position
    Correct,
    /// Letter is in the chosen word but it's not in the correct position
    Partial,
    /// Letter is not in the chosen word at all
    Incorrect,
}

impl Words {
    pub fn new() -> Self {
        Self {
            all_words: WORDLIST.lines().filter(|l| l.trim().len() == 5).collect(),
            chosen_word: None,
        }
    }

    pub fn choose_word(&mut self) -> String {
        let random_index: usize = rand::thread_rng().gen_range(0..self.all_words.len());
        self.chosen_word = Some(self.all_words[random_index]);

        self.chosen_word.unwrap().to_string()
    }

    pub fn is_word_valid(&self, word: &str) -> bool {
        if word.len() != 5 {
            false
        } else {
            self.all_words.contains(&word)
        }
    }

    /// Check all letters in the user input word compared to chosen word
    ///
    /// returns array of LetterMatch states for each letter
    pub fn check_word(&self, word: &str) -> [LetterMatch; 5] {
        let mut letter_states = [LetterMatch::Incorrect; 5];
        let chosen_word = self.chosen_word.unwrap();

        for (i, (c1, c2)) in chosen_word.chars().zip(word.chars()).enumerate() {
            if chosen_word.contains(c2) {
                letter_states[i] = LetterMatch::Partial;
            }

            letter_states[i] = if c1 == c2 {
                LetterMatch::Correct
            } else {
                letter_states[i]
            };
        }

        letter_states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_words() {
        let words = Words::new();
        assert_eq!(words.is_word_valid("tests"), true);
        assert_eq!(words.is_word_valid("test"), false);
        assert_eq!(words.is_word_valid("testing"), false);
        assert_eq!(words.is_word_valid(""), false);
    }

    #[test]
    fn check_words() {
        let mut words = Words::new();
        words.chosen_word = Some("tests");

        let expected = [LetterMatch::Correct; 5];
        assert_eq!(words.check_word("tests"), expected);

        let expected = [LetterMatch::Incorrect; 5];
        assert_eq!(words.check_word("abcdf"), expected);

        let expected = [LetterMatch::Partial; 5];
        assert_eq!(words.check_word("ettst"), expected);

        let expected = [
            LetterMatch::Partial,
            LetterMatch::Correct,
            LetterMatch::Correct,
            LetterMatch::Incorrect,
            LetterMatch::Correct,
        ];
        assert_eq!(words.check_word("eesbs"), expected);
    }
}