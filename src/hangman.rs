use libc_print::*;

pub struct Hangman<'a> {
    word: &'a str,
    letters_used: u32,
    guesses_remaining: u32,
}

impl<'a> Hangman<'a> {
    pub fn new(word: &'a str, guesses_remaining: u32) -> Self {
        Self {
            word,
            letters_used: 0,
            guesses_remaining,
        }
    }

    pub fn state(&self) -> HangmanState {
        let mut won = true;
        for l in self.word.as_bytes() {
            if !self.is_letter_used(*l) {
                won = false;
                break;
            }
        }

        if won {
            HangmanState::Won
        } else if self.guesses_remaining == 0 {
            HangmanState::Lost
        } else {
            HangmanState::Playing
        }
    }

    pub fn guess(&mut self) {
        self.guess_prompt();
        let letter = self.input_guess();
        if !self.word.contains(letter as char) {
            self.guesses_remaining -= 1;
        }
        self.set_letter_used(letter);
    }

    fn input_guess(&self) -> u8 {
        loop {
            // SAFETY: `libc::getchar` has no invariants to uphold.
            let c = unsafe { libc::getchar() as u8 };

            if c.is_ascii_lowercase() && !self.is_letter_used(c) {
                break c;
            }
        }
    }

    fn guess_prompt(&self) {
        self.print_state();

        libc_print!("\n\nPlease guess a letter...");
    }

    pub fn print_state(&self) {
        libc_print!(
            "\x1Bc--- HANGMAN ---\nYou have {} guesses remaining.\nLetters used: ",
            self.guesses_remaining
        );

        for l in 'a'..='z' {
            if self.is_letter_used(l as u8) {
                libc_print!("{} ", l);
            }
        }
        libc_println!();

        for l in self.word.chars() {
            if self.is_letter_used(l as u8) {
                libc_print!("{}", l);
            } else {
                libc_print!("_");
            }
        }
    }

    fn is_letter_used(&self, letter: u8) -> bool {
        let bit_index = letter - b'a';
        let used_shifted = self.letters_used >> bit_index;

        used_shifted & 0b1 == 0b1
    }

    fn set_letter_used(&mut self, letter: u8) {
        let bit_index = letter - b'a';
        let used_shifted = 0b1u32 << bit_index;

        self.letters_used |= used_shifted
    }
}

pub enum HangmanState {
    Playing,
    Won,
    Lost,
}
