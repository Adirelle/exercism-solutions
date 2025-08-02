use std::collections::HashMap;
use std::thread;
use crossbeam_channel::bounded;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    let output_rx = {            
        let (input_tx, input_rx) = bounded(input.len());
        let (output_tx, output_rx) = bounded(worker_count);
    
        for _ in 0..worker_count {
            let input_rx = input_rx.clone();
            let output_tx = output_tx.clone();
            thread::spawn(move || {
                let result = input_rx.iter().fold(HashMap::new(), count_letters);
                output_tx.send(result).unwrap();
            });
        }

        for text in input.into_iter() {
            input_tx.send(text.to_lowercase()).unwrap();
        }

        output_rx
    };
    
    output_rx.iter().reduce(merge_counts).unwrap_or_default()
}

fn count_letters(mut counts: HashMap<char, usize>, text: String) -> HashMap<char, usize> {
    for letter in text.chars().filter(|c| c.is_alphabetic()) {
        *counts.entry(letter).or_insert(0) += 1;
    }
    counts
}

fn merge_counts(a: HashMap<char, usize>, mut b: HashMap<char, usize>) -> HashMap<char, usize> {
    for (letter, count) in a.into_iter() {
         *b.entry(letter).or_insert(0) += count;
    }
    b
}
