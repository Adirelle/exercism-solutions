use bitvec::prelude::*;
use std::collections::BTreeSet;

pub fn factors(mut n: u64) -> Vec<u64> {
    let x = 10_u64.pow(n.ilog10() / 2 + 1);
    let primes = primes_up_to(x);    
    let mut factors = Vec::new();
    for prime in primes.iter() {
        while n % prime == 0 {
            factors.push(*prime);
            n /= prime; 
        }
        if primes.contains(&n) {
            factors.push(n);
            break;
        } else if n == 1 {
            break;
        }
    }
    factors
}

fn primes_up_to(n: u64) -> BTreeSet<u64> {
    let k = (n - 1) / 2;
    let mut numbers = bitvec![1; (k+1) as usize];
    for i in 1..=isqrt(k) {
        let mut j = i;
        loop {
            let idx = i + j + 2 * i * j;
            if idx > k {
                break;
            }
            *numbers.get_mut(idx as usize).unwrap() = false;
            j += 1;
        }
    }
    [2].into_iter().chain(
        numbers
        .into_iter()
        .enumerate()
        .skip(1)
        .filter_map(|(i, is_prime)| is_prime.then_some((i * 2 + 1) as u64))
    ).collect()
}

fn isqrt(n: u64) -> u64 {
    (n as f64).sqrt().floor() as u64
}
