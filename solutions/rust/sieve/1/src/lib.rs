pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers = (0..=upper_bound).map(|n| n > 1).collect::<Vec<_>>();
    for x in 2..upper_bound as usize {
        if numbers[x] {
            for y in(x*2 ..= upper_bound as usize).step_by(x) {
                numbers[y] = false
            }
        }
    }
    numbers.into_iter()
        .enumerate()
        .filter_map(|(i, is_prime)| is_prime.then_some(i as u64))
        .collect()
}
