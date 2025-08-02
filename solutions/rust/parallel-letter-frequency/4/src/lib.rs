use std::collections::HashMap;
use std::thread;
use crossbeam_channel::bounded;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
    
        let output_rx = {            
            let (input_tx, input_rx) = bounded(input.len());
            let (output_tx, output_rx) = bounded(worker_count);
        
            for _ in 0..worker_count {
                let input_rx = input_rx.clone();
                let output_tx = output_tx.clone();
                s.spawn(move || {
                    let result = input_rx.iter().fold(HashMap::new(), count_letters);
                    output_tx.send(result).unwrap();
                });
            }
    
            for text in input.iter() {
                input_tx.send(text).unwrap();
            }
    
            output_rx
        };
        
        output_rx.iter().reduce(merge_counts).unwrap_or_default()
    })
}

fn count_letters(counts: HashMap<char, usize>, text: &str) -> HashMap<char, usize> {
    text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(counts, |mut counts, letter| {
            *counts.entry(letter).or_insert(0) += 1;
            counts
        })
}

fn merge_counts(a: HashMap<char, usize>, b: HashMap<char, usize>) -> HashMap<char, usize> {
    a.into_iter().fold(b, |mut counts, (letter, count)| {
        *counts.entry(letter).or_insert(0) += count;
        counts
    })
}
