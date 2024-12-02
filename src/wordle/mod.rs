use std::{collections::HashMap, thread};

use indicatif::{ProgressBar, ProgressStyle};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Wordle {
    words: Vec<String>,
    guesses: Vec<Guess>,
}

impl Wordle {
    pub fn new(words: Vec<String>) -> Self {
        Wordle {
            words,
            guesses: vec![],
        }
    }

    /// Make a new guess
    pub fn guess(&mut self, word: Guess) {
        self.guesses.push(word);
    }

    /// Returns a list, sorted by the best next guesses
    pub fn next(&self) -> Vec<(&String, usize)> {
        let mut hashmap: HashMap<&String, usize> = HashMap::new();

        let mut fake_game = self.clone();

        // For each possible word, we want to check for every guess (all words),
        // which possible words remain
        let bar = ProgressBar::new(self.possible().len().try_into().unwrap());
        bar.set_style(
            ProgressStyle::with_template(
                "{msg} {wide_bar} [{elapsed_precise}/{duration_precise}] {pos:>7}/{len:7}",
            )
            .unwrap(),
        );
        for possible_target in self.possible().iter() {
            bar.inc(1);
            bar.set_message((*possible_target).clone());
            for possible_guess in self.words.iter() {
                let guess = Guess::guess(possible_guess, possible_target);

                fake_game.guess(guess);
                // How many words are now possible, if we assume the `possible_target`
                // is the actual target
                *hashmap.entry(possible_guess).or_insert(0) += fake_game.possible().len();
                fake_game.guesses.pop();
            }
        }
        bar.finish();

        let mut vec: Vec<(&String, usize)> = hashmap.into_iter().collect();

        vec.sort_by(|(_, a), (_, b)| a.cmp(b));
        vec
    }

    /// Returns a list, sorted by the best next guesses
    pub fn next_parallel(&self, n: usize) -> Vec<(&String, usize)> {
        let mut hashmap: HashMap<&String, usize> = HashMap::new();

        // For each possible word, we want to check for every guess (all words),
        // which possible words remain
        let possible = self.possible();
        thread::scope(|s| {
            let (tx, rx) = std::sync::mpsc::channel();
            for chunk in possible.chunks(possible.len() / n) {
                let mut game = self.clone();
                let tx = tx.clone();
                s.spawn(move || {
                    let mut hm: HashMap<&String, usize> = HashMap::new();
                    for possible_target in chunk {
                        for possible_guess in self.words.iter() {
                            let guess = Guess::guess(possible_guess, possible_target);

                            game.guess(guess);
                            // How many words are now possible, if we assume the `possible_target`
                            // is the actual target
                            let possible_for_guess = game.possible().len();
                            *hm.entry(possible_guess).or_insert(0) += possible_for_guess;
                            game.guesses.pop();
                        }
                    }
                    tx.send(hm).unwrap();
                });
            }
            drop(tx);
            while let Ok(hm) = rx.recv() {
                for (k, v) in hm.into_iter() {
                    *hashmap.entry(k).or_insert(0) += v;
                }
            }
        });

        let mut vec: Vec<(&String, usize)> = hashmap.into_iter().collect();

        vec.sort_by(|(_, a), (_, b)| a.cmp(b));
        vec
    }

    /// Returns a list of all possible words
    pub fn possible(&self) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| self.guesses.iter().all(|guess| guess.allows(&word)))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Guess {
    guess: [GuessedCharacter; 5],
}

impl Guess {
    pub fn new(guess: [GuessedCharacter; 5]) -> Self {
        Guess { guess }
    }

    /// Returns a `guess`, assuming that the searched String is `target`
    pub fn guess(guess: &str, target: &str) -> Self {
        let mut res = [
            GuessedCharacter::Correct('a'),
            GuessedCharacter::Correct('a'),
            GuessedCharacter::Correct('a'),
            GuessedCharacter::Correct('a'),
            GuessedCharacter::Correct('a'),
        ];

        // How often `char` appears in target at the wrong location
        let mut frequencies: HashMap<char, usize> = HashMap::new();

        target.chars().enumerate().for_each(|(i, ch)| {
            if guess.chars().nth(i) != Some(ch) {
                *frequencies.entry(ch).or_insert(0) += 1
            }
        });

        guess
            .chars()
            .into_iter()
            .enumerate()
            .for_each(|(i, guessed_char)| {
                if target.chars().nth(i) == Some(guessed_char) {
                    res[i] = GuessedCharacter::Correct(guessed_char);
                } else {
                    let amount = frequencies.entry(guessed_char).or_default();

                    if *amount > 0 {
                        *amount -= 1;
                        res[i] = GuessedCharacter::Elsewhere(guessed_char);
                    } else {
                        res[i] = GuessedCharacter::Not(guessed_char);
                    }
                }
            });

        Guess { guess: res }
    }

    /// Wether `candidate` is possible with this guess
    pub fn allows(&self, candidate: &str) -> bool {
        self.guess.iter().enumerate().all(|(i, ch)| match ch {
            // Would this character be grey for `candidate`?
            GuessedCharacter::Not(ch) => {
                // Would be green
                if candidate.chars().nth(i) == Some(*ch) {
                    return false;
                }

                let orange = self.orange_in_candidate(ch, candidate);

                orange == 0 || self.already_guessed(i, ch) >= orange
            }
            // Would this character be orange for `candidate`?
            GuessedCharacter::Elsewhere(ch) => {
                // Would be green
                if candidate.chars().nth(i) == Some(*ch) {
                    return false;
                }

                let orange = self.orange_in_candidate(ch, candidate);

                orange != 0 && self.already_guessed(i, ch) < orange
            }
            // Would this character be green for `candidate`?
            GuessedCharacter::Correct(ch) => candidate.chars().nth(i) == Some(*ch),
        })
    }

    /// Number of times the character `ch` should be orange in `self` for `candidate`
    fn orange_in_candidate(&self, ch: &char, candidate: &str) -> usize {
        // Number of times this character is in the candidate
        let total_in_candidate = candidate.chars().filter(|c| c == ch).count();

        if total_in_candidate == 0 {
            return 0;
        }

        // Number of times this character is green in the candidate
        let green_in_candidate = self
            .guess
            .iter()
            .zip(candidate.chars().into_iter())
            .filter(|(guess_ch, candidate_ch)| {
                matches!(guess_ch, GuessedCharacter::Correct(_)) && candidate_ch == ch
            })
            .count();

        total_in_candidate - green_in_candidate
    }

    /// Number of times the character `ch` appears before `i` in our guess
    fn already_guessed(&self, i: usize, ch: &char) -> usize {
        self.guess
            .iter()
            .enumerate()
            .filter(|(l, ch_check)| {
                if let GuessedCharacter::Elsewhere(ch_check) = ch_check {
                    *l < i && ch_check == ch
                } else {
                    false
                }
            })
            .count()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GuessedCharacter {
    Not(char),
    Elsewhere(char),
    Correct(char),
}
