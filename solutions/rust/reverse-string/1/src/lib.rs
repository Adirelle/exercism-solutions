pub fn reverse(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    for c in input.chars().rev() {
        res.push(c)
    }
    res
}
