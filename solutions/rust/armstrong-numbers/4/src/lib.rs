pub fn is_armstrong_number(num: u32) -> bool {
    num < 10 || calc(num, 1 + num.ilog10()) == num
}

fn calc(n: u32, len: u32) -> u32 { 
    if n > 0 {
        (n % 10).pow(len) + calc(n / 10, len)
    } else {
        0 
    }
}
