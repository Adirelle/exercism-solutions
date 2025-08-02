
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut x = 2;    
    
    while n > 1 {
        while n % x == 0 {
            factors.push(x);
            n /= x;
        }
        x += 1
    }        
    
    factors
}
