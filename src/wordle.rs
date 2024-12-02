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
                let in_candidate = candidate.chars().filter(|c| c == ch).count();

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

                // Number of times this char should be orange, from the start
                let orange = in_candidate - green_in_candidate;

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

                // println!("({i}, {ch}): {is_not_current} && {already_guessed} < {orange}");
                is_not_current && already_guessed < orange
            }
            // Would this character be green for `candidate`?
            GuessedCharacter::Correct(ch) => candidate.chars().nth(i) == Some(*ch),
        })
    }
}

#[derive(Debug)]
pub enum GuessedCharacter {
    Not(char),
    Elsewhere(char),
    Correct(char),
}
