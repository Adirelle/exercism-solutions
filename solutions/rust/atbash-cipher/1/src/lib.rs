/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::new();
    for (i, c) in plain.chars().filter_map(reverse).enumerate() {
        if i > 0 && i % 5 == 0 {
            result.push(' ');
        }
        result.push(c);
    }
    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter_map(reverse)
        .collect()
}

const UC_END: u32 = 'z' as u32 + 'A' as u32;
const LC_END: u32 = 'z' as u32 + 'a' as u32;

fn reverse(c: char) -> Option<char> {
    match c {
        'A'..='Z' => char::from_u32(UC_END - c as u32),
        'a'..='z' => char::from_u32(LC_END - c as u32),
        '0'..='9' => Some(c),
        _ => None,
    }
}
