pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    while n != 1 {
        n = if n & 1 == 1 {
            n * 3 + 1
        } else {
            n / 2
        };
        steps += 1;
    }
    Some(steps)
}
