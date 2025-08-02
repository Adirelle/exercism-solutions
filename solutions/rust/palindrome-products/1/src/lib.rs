#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let s: Vec<char> = format!("{}", value).chars().collect();
        let r: Vec<char> = s.iter().rev().copied().collect();
        (s == r).then_some(Self(value))
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut range: Option<(Palindrome, Palindrome)> = None;
    for i in min..=max {
        for j in min..=max {
            if let Some(p) = Palindrome::new(i * j) {
                range = range.map_or(Some((p, p)), |(lo, hi)| Some((lo.min(p), hi.max(p))))
            }
        }
    }
    range
}
