use super::*;

#[test]
fn test_allow_full_correct() {
    let guess = Guess::new([
        GuessedCharacter::Correct('a'),
        GuessedCharacter::Correct('b'),
        GuessedCharacter::Correct('c'),
        GuessedCharacter::Correct('d'),
        GuessedCharacter::Correct('e'),
    ]);

    assert!(guess.allows("abcde"));
    assert!(!guess.allows("bbcde"));
}

#[test]
fn test_allow_full_not() {
    let guess = Guess::new([
        GuessedCharacter::Not('a'),
        GuessedCharacter::Not('b'),
        GuessedCharacter::Not('c'),
        GuessedCharacter::Not('d'),
        GuessedCharacter::Not('e'),
    ]);

    assert!(guess.allows("fghij"));
    assert!(!guess.allows("afghi"));
    assert!(!guess.allows("abcde"));
}

#[test]
fn test_one_orange() {
    let guess = Guess::new([
        GuessedCharacter::Elsewhere('a'),
        GuessedCharacter::Not('a'),
        GuessedCharacter::Not('b'),
        GuessedCharacter::Not('b'),
        GuessedCharacter::Not('b'),
    ]);

    assert!(guess.allows("cdadd"));
    assert!(!guess.allows("adddd"));
    assert!(!guess.allows("daddd")); // First 'a' would be grey, second correct
    assert!(!guess.allows("ddaad")); // Second 'a' would be green
}

#[test]
fn test_one_orange_one_green() {
    let guess = Guess::new([
        GuessedCharacter::Elsewhere('a'),
        GuessedCharacter::Correct('a'),
        GuessedCharacter::Not('b'),
        GuessedCharacter::Not('b'),
        GuessedCharacter::Not('b'),
    ]);

    assert!(guess.allows("daadd"));
    assert!(guess.allows("daaad"));
    assert!(!guess.allows("daddd")); // First 'a' would be grey
    assert!(!guess.allows("aaddd")); // First 'a' would be green
}

#[test]
fn test_number_orange() {
    let guess = Guess::new([
        GuessedCharacter::Elsewhere('a'),
        GuessedCharacter::Correct('a'),
        GuessedCharacter::Elsewhere('c'),
        GuessedCharacter::Not('c'),
        GuessedCharacter::Not('b'),
    ]);

    assert_eq!(guess.orange_in_candidate(&'a', "aacca"), 2);
    assert_eq!(guess.orange_in_candidate(&'a', "accaa"), 3);
    assert_eq!(guess.orange_in_candidate(&'c', "accaa"), 1);
}

#[test]
fn test_create_correct_guess() {
    let guess = Guess::guess("hello", "hello");

    for ch in &guess.guess {
        assert!(matches!(ch, GuessedCharacter::Correct(_)));
    }

    assert!(guess
        .guess
        .into_iter()
        .zip("hello".chars().into_iter())
        .all(|(guess, input)| match guess {
            GuessedCharacter::Correct(c) => c == input,
            _ => false,
        }));
}

#[test]
fn test_create_some_guess() {
    let guess = Guess::guess("aabbb", "daadd");

    assert_eq!(guess.guess[0], GuessedCharacter::Elsewhere('a'));
    assert_eq!(guess.guess[1], GuessedCharacter::Correct('a'));
    assert_eq!(guess.guess[2], GuessedCharacter::Not('b'));
    assert_eq!(guess.guess[3], GuessedCharacter::Not('b'));
    assert_eq!(guess.guess[4], GuessedCharacter::Not('b'));
}
