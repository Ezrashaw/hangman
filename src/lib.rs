#![no_std]

use hangman::Hangman;
use libc_print::libc_println;
use rand::Rng;
use termios::Termios;

pub mod hangman;

pub fn main() {
    let mut termios = Termios::from_fd(libc::STDIN_FILENO).unwrap();
    termios.c_lflag &= !termios::ECHO;
    termios.c_lflag &= !termios::ICANON;
    termios::tcsetattr(libc::STDIN_FILENO, termios::TCSANOW, &termios).unwrap();

    let mut rng = rand::thread_rng();

    let word = WORDS
        .split('\n')
        .skip(rng.gen_range(0..count_lines(WORDS)))
        .next()
        .unwrap();

    let mut hangman = Hangman::new(word, 12);

    loop {
        hangman.guess();

        match hangman.state() {
            hangman::HangmanState::Playing => (),
            hangman::HangmanState::Won => {
                hangman.print_state();
                libc_println!("\nYou won!");
                break;
            }
            hangman::HangmanState::Lost => {
                hangman.print_state();
                libc_println!("\nGame over! The word was {}.", word);
                break;
            }
        }
    }
}

const WORDS: &str = include_str!("words.txt");

fn count_lines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1
}
