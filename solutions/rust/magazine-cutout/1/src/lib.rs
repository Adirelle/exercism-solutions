use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words: HashMap<&str, usize> = HashMap::new();

    for word in magazine.iter() {
        let count = words.entry(word).or_default();
        (*count) += 1;
    }

    for word in note.iter() {
        let count = words.entry(word).or_default();
        if *count > 0 {
            (*count) -= 1;
        } else {
            return false;
        }
    }

    true
}
