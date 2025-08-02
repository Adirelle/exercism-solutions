/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;
    for char in code.chars().rev() {
        if let Some(digit) = char.to_digit(10) {
            sum += if len & 1 == 1 {
                if digit > 4 {
                    digit * 2 - 9
                } else {
                    digit * 2
                }
            } else {
                digit
            };
            len += 1;
        } else if char != ' ' {
            return false;
        }
    }
    len > 1 && sum % 10 == 0
}
