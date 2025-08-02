use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    (num > 0).then_some(
        match aliqot_sum(num).cmp(&num) {
            Ordering::Greater => Classification::Abundant,
            Ordering::Equal => Classification::Perfect,
            Ordering::Less => Classification::Deficient,
        }
    )
}

fn aliqot_sum(num: u64) -> u64 {
    (1..num)
        .filter(|div| num % div == 0)
        .sum()
}