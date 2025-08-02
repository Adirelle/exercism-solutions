use std::fmt::Write;

static UNITS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
];

static TENTHS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];

static POWERS: [(u64, &str); 7] = [
    (1_000_000_000_000_000_000, "quintillion"),
    (1_000_000_000_000_000, "quadrillion"),
    (1_000_000_000_000, "trillion"),
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1000, "thousand"),
    (100, "hundred"),
];

pub fn encode(mut n: u64) -> String {
    let mut number = String::new();
    for &(value, word) in POWERS.iter() {
        if n >= value {
            write!(number, "{} {word}", encode(n / value)).unwrap();
            n %= value;
            if n == 0 {
                return number;
            }
            number.push(' ');
        }
    }
    if n < 20 {
        number += UNITS[n as usize];
    } else {
        number += TENTHS[n as usize / 10 - 2];
        if n % 10 != 0 {
            write!(number, "-{}", UNITS[n as usize % 10]).unwrap();
        }
    }
    number
}
