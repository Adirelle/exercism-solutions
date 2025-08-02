pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        2 => 5,
        3 => 7,
        4 => 11,
        5 => 13,
        _ => (17..u32::MAX).filter(|x| is_prime(*x)).nth(n as usize - 6).unwrap()
    }    
}

fn is_prime(n: u32) -> bool {
    (2..=isqrt(n)).all(|x| n % x != 0)
}

fn isqrt(n: u32) -> u32 {
    ((n as f32).sqrt().floor()) as u32
}