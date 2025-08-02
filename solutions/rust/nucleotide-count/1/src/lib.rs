use std::collections::HashMap;

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::new();
    result.insert('A', 0);
    result.insert('C', 0);
    result.insert('G', 0);
    result.insert('T', 0);
    for c in dna.chars().map(check_nucleotide) {
        *result.entry(c?).or_insert(0) += 1;
    }
    Ok(result)    
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let nucleotide = check_nucleotide(nucleotide)?;
    let mut count = 0;
    for c in dna.chars().map(check_nucleotide) {
        if c? == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn check_nucleotide(c: char) -> Result<char, char> {
    match c {
        'A'|'a' => Ok('A'),
        'C'|'c' => Ok('C'),
        'G'|'g' => Ok('G'),
        'T'|'t' => Ok('T'),
        _ => Err(c),
    }
}
