use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter()
        .filter(|factor| **factor != 0)
        .flat_map(|factor| (1..limit).filter(|x| x % *factor == 0))
        .collect::<HashSet<_>>()
        .into_iter()
        .sum()
}
