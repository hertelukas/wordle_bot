use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use clap::Parser;

/// Program showing best moves in Wördle
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of input file with words
    #[arg(short, long)]
    words: Option<String>,
}

#[derive(Debug)]
struct Game {
    words: Vec<String>,
    guesses: Vec<Guess>,
    target: Option<String>,
}

impl Game {
    pub fn new(words: Vec<String>) -> Self {
        Game {
            words,
            guesses: vec![],
            target: None,
        }
    }

    /// Make a new guess
    pub fn guess(&mut self, word: Guess) {
        self.guesses.push(word);
    }

    /// Returns a list, sorted by the best next guesses
    pub fn next(&self) -> Vec<String> {
        let res = vec![];

        res
    }

    /// Returns a list of all possible words
    fn possible(&self) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| self.guesses.iter().all(|guess| guess.allows(&word)))
            .collect()
    }
}

#[derive(Debug)]
struct Guess {
    guess: [GuessedCharacter; 5],
}

impl Guess {
    fn new(guess: [GuessedCharacter; 5]) -> Self {
        Guess { guess }
    }

    /// Wether `candidate` is possible with this guess
    fn allows(&self, candidate: &str) -> bool {
        self.guess.iter().enumerate().all(|(i, ch)| match ch {
            // Would this character be grey for `candidate`?
            GuessedCharacter::Not(ch) => candidate.chars().all(|c| &c != ch),
            // Would this character be orange for `candidate`?
            GuessedCharacter::Elsewhere(ch) => {
                // Is not the current character
                let is_not_current = candidate.chars().nth(i) != Some(*ch);

                // Number of times this character is in the candidate
                let i = candidate.chars().filter(|c| c == ch).count();

                // Number of times this character is green in the candidate
                let j = self
                    .guess
                    .iter()
                    .enumerate()
                    .filter(|(k, ch_check)| {
                        if let GuessedCharacter::Correct(ch_check) = ch_check {
                            ch_check == ch && candidate.chars().nth(*k) == Some(*ch)
                        } else {
                            false
                        }
                    })
                    .count();

                // Number of times this char should be orange, from the start
                let orange = i - j;

                // Number of times the character appears before in our guess
                let already_guessed = self
                    .guess
                    .iter()
                    .enumerate()
                    .filter(|(l, ch_check)| {
                        if let GuessedCharacter::Elsewhere(ch_check) = ch_check {
                            *l < i && ch_check == ch
                        } else {
                            false
                        }
                    })
                    .count();

                is_not_current && already_guessed <= orange
            }
            // Would this character be green for `candidate`?
            GuessedCharacter::Correct(ch) => candidate.chars().nth(i) == Some(*ch),
        })
    }
}

#[derive(Debug)]
enum GuessedCharacter {
    Not(char),
    Elsewhere(char),
    Correct(char),
}

fn main() {
    use GuessedCharacter::{Correct, Elsewhere, Not};
    let args = Args::parse();

    let words = load_words(args.words.unwrap_or("words".to_owned())).expect("Failed to load words");
    let mut game = Game::new(words);

    game.guess(Guess {
        guess: [Not('r'), Not('a'), Not('s'), Not('e'), Correct('n')],
    });

    println!("{:?}", game.possible())
}

fn load_words<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path).unwrap();

    BufReader::new(file).lines().into_iter().collect()
}
