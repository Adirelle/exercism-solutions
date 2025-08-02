pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    (2..)
        .filter(|candidate| 
            if primes.iter().any(|prime| candidate % *prime == 0) {
                false
            } else {
                primes.push(*candidate);
                true
            }
        )
        .nth(n as usize)
        .unwrap()
}
