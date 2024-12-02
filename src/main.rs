use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    path::Path,
};

use clap::Parser;

use wordle_bot::{Guess, GuessedCharacter, Wordle};

/// Program showing best moves in Wördle
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of input file with words
    #[arg(short, long)]
    words: String,

    /// Number of threads to calculate next best move
    #[arg(short, long)]
    threads: Option<usize>,

    #[arg(short, long)]
    interactive: bool,
}

/// Print the best `n` next guesses for `game`, using `t` threads
fn print_best(game: &Wordle, n: usize, t: usize) {
    let mut next = if t == 1 {
        game.next()
    } else {
        game.next_parallel(t)
    };

    let possible = game.possible();

    let mut n = std::cmp::min(n, next.len());
    println!("----- {} possible words -----", possible.len());

    next.sort_by(|(word_a, expected_a), (word_b, expected_b)| {
        // First, compare by `expected`
        let ord = expected_a.cmp(expected_b);

        if ord == std::cmp::Ordering::Equal {
            // If `expected_a == expected_b`, compare based on which is possible
            match (possible.contains(word_a), possible.contains(word_b)) {
                (true, false) => std::cmp::Ordering::Less, // `word_a` before `word_b`
                (false, true) => std::cmp::Ordering::Greater, // `word_b` before `word_a`
                _ => std::cmp::Ordering::Equal, // If both are true or both are false, no specific order
            }
        } else {
            ord // Return the result of the `expected` comparison
        }
    });

    for (word, expected) in &next {
        n -= 1;
        let possible = if possible.contains(word) {
            "✓"
        } else {
            "✗"
        };
        println!("{possible} {} ({})", word, expected);
        if n == 0 {
            return;
        }
    }
}

fn read_guess() -> Guess {
    let mut res = [
        GuessedCharacter::Correct('a'),
        GuessedCharacter::Correct('a'),
        GuessedCharacter::Correct('a'),
        GuessedCharacter::Correct('a'),
        GuessedCharacter::Correct('a'),
    ];

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        if input.len() != 5 {
            println!("Input has to be 5 characters long!");
            continue;
        }

        print!("Input the colors (0 = grey, 1 = yellow, 2 = green): ");
        io::stdout().flush().unwrap();

        let mut levels = String::new();
        io::stdin().read_line(&mut levels).unwrap();

        levels = levels.trim().to_string();

        if levels.len() != 5 || levels.chars().any(|c| c != '0' && c != '1' && c != '2') {
            println!("Expected 5 numbers!");
            continue;
        }

        input
            .chars()
            .into_iter()
            .zip(levels.chars().into_iter())
            .enumerate()
            .for_each(|(i, (ch, lvl))| {
                res[i] = match lvl {
                    '0' => GuessedCharacter::Not(ch),
                    '1' => GuessedCharacter::Elsewhere(ch),
                    '2' => GuessedCharacter::Correct(ch),
                    _ => panic!("This should have been handled when checking levels input!"),
                };
            });

        return Guess::new(res);
    }
}

fn interactive(game: &mut Wordle, n: usize, t: usize) {
    loop {
        game.guess(read_guess());
        print_best(&game, n, t);

        if game.solved() {
            println!(
                "The solution has to be: {}",
                game.possible().first().unwrap()
            );
            return;
        }
    }
}

fn main() {
    let args = Args::parse();

    let words = load_words(args.words).expect("Failed to load words");
    let mut game = Wordle::new(words);

    if args.interactive {
        interactive(&mut game, 6, args.threads.unwrap_or(8));
    } else {
        print_best(&game, 25, args.threads.unwrap_or(8));
    }
}

fn load_words<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path).unwrap();

    BufReader::new(file).lines().into_iter().collect()
}
