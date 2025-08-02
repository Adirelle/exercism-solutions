#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;

    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n && a.windows(n).any(|v| v == b) => Superlist,
        (m, n) if m < n && b.windows(m).any(|v| v == a) => Sublist,
        (m, n) if m == n && a == b => Equal,
        _ => Unequal,
    }
}
