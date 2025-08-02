#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

fn is_palindrome(num: u64) -> bool {
    let mut rev = 0;
    let mut x = num;
    while x > 0 {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    rev == num
}

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        is_palindrome(value).then_some(Palindrome(value))
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_pal = None;
    let mut max_pal = None;
    for i in min..=max {
        for j in min..=max {
            if let Some(p) = Palindrome::new(i * j) {
                min_pal = Some(min_pal.map_or(p, |m| p.min(m)));
                max_pal = Some(max_pal.map_or(p, |m| p.max(m)));
            }
        }
    }
    min_pal.and_then(|min| max_pal.map(|max| (min, max)))
}
