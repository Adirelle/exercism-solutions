#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if is_sublist(a, b) {
        Comparison::Sublist
    } else if is_sublist(b, a) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.is_empty() {
        true
    } else if a.len() > b.len() {
        false
    } else {
        let a_len = a.len();
        let mut i = 0;
        let max_i = b.len() - a_len;
        while i <= max_i {
            if a == &b[i..(i + a_len)] {
                return true
            }
            i += 1;
        }
        false
    }
}
