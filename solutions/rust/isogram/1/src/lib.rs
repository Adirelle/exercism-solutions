use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    candidate.chars()
        .filter_map(|c| c.is_alphabetic().then_some(c.to_ascii_uppercase()))
        .all(|c| if letters.contains(&c) {
            false
        } else {
            letters.insert(c);
            true
        })
}
