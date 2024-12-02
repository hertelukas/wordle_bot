use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Wordle {
    words: Vec<String>,
    guesses: Vec<Guess>,
    target: Option<String>,
}

impl Wordle {
    pub fn new(words: Vec<String>) -> Self {
        Wordle {
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

        // For each possible word, we want to check for every guess (all words),
        // which possible words remain
        for possible_target in self.possible() {}
        res
    }

    /// Returns a list of all possible words
    pub fn possible(&self) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| self.guesses.iter().all(|guess| guess.allows(&word)))
            .collect()
    }
}

#[derive(Debug)]
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
                let is_not_current = candidate.chars().nth(i) != Some(*ch);

                let orange = self.orange_in_candidate(ch, candidate);

                is_not_current && self.already_guessed(i, ch) >= orange
            }
            // Would this character be orange for `candidate`?
            GuessedCharacter::Elsewhere(ch) => {
                // Is not the current character
                let is_not_current = candidate.chars().nth(i) != Some(*ch);

                let orange = self.orange_in_candidate(ch, candidate);

                is_not_current && self.already_guessed(i, ch) < orange
            }
            // Would this character be green for `candidate`?
            GuessedCharacter::Correct(ch) => candidate.chars().nth(i) == Some(*ch),
        })
    }

    /// Number of times the character `ch` should be orange in `self` for `candidate`
    fn orange_in_candidate(&self, ch: &char, candidate: &str) -> usize {
        // Number of times this character is in the candidate
        let total_in_candidate = candidate.chars().filter(|c| c == ch).count();

        // Number of times this character is green in the candidate
        let green_in_candidate = self
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

#[derive(Debug, PartialEq)]
pub enum GuessedCharacter {
    Not(char),
    Elsewhere(char),
    Correct(char),
}
