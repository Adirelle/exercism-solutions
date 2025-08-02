use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = HashSet::new();
    letters.extend('A'..='Z');
    sentence.chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| _ = letters.remove(&c.to_ascii_uppercase()));
    letters.is_empty()
}
