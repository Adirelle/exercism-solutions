/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut f = 10;
    for c in isbn.chars() {
        match c {
            '0'..='9' if f >= 1 => {
                sum += f * c.to_digit(10).unwrap();
                f -= 1;
            }
            'X' if f == 1 => {
                sum += 10;
                f -= 1;
            }
            '-'           => (),
            _             => return false,
        }
    }
    f == 0 && sum % 11 == 0
}
