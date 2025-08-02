pub fn translate(mut rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::new();
    while rna.len() >= 3 {
        let (codon, tail) = rna.split_at(3);
        rna = tail;
        proteins.push(match codon {
            "AUG" => "Methionine",
            "UUU" | "UUC" => "Phenylalanine",
            "UUA" | "UUG" => "Leucine",
            "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
            "UAU" | "UAC" => "Tyrosine",
            "UGU" | "UGC" => "Cysteine",
            "UGG" => "Tryptophan",
            "UAA" | "UAG" | "UGA" => return Some(proteins),
            _ => return None,
        })
    }
    rna.is_empty().then_some(proteins)
}
