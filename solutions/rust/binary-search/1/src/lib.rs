use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut a = 0;
    let mut b = array.len();
    while a < b {
        let n = (a + b) / 2;
        match key.cmp(&array[n]) {
            Ordering::Equal => return Some(n),
            Ordering::Less => b = n,
            _ => a = n+1,
        }
    }
    None
}
