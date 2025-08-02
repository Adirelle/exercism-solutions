use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let word_letters = letter_histogram(&lower_word);
    possible_anagrams
        .into_iter()
        .filter(|to_test| {
            if to_test.len() != lower_word.len() {
                return false;
            }
            let lower_test = to_test.to_lowercase();
            lower_word != lower_test && word_letters == letter_histogram(&lower_test)
        })
        .map(|s| *s)
        .collect::<HashSet<&'a str>>()
}

pub fn letter_histogram(word: &str) -> HashMap<char, usize> {
    let mut letters = HashMap::new();
    for c in word.chars() {
        *(letters.entry(c).or_insert(0)) += 1;
    }
    letters
}
