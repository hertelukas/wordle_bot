use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use clap::Parser;

use wordle_bot::Wordle;

/// Program showing best moves in Wördle
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of input file with words
    #[arg(short, long)]
    words: String,
}

fn main() {
    let args = Args::parse();

    let words = load_words(args.words).expect("Failed to load words");
    let game = Wordle::new(words);

    println!("{:?}", game.next());
}

fn load_words<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path).unwrap();

    BufReader::new(file).lines().into_iter().collect()
}
