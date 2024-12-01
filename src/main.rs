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

    /// Wether the word is still possible with this guess
    fn allows(&self, candidate: &str) -> bool {
        self.guess.iter().enumerate().all(|(i, ch)| match ch {
            GuessedCharacter::Not(ch) => candidate.chars().all(|c| &c != ch),
            GuessedCharacter::Elsewhere(ch) => {
                candidate.chars().any(|c| &c == ch) && candidate.chars().nth(i) != Some(*ch)
            }
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
