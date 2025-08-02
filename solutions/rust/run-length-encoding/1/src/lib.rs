use std::fmt::Write;
use std::iter::repeat;

pub fn encode(source: &str) -> String {
    let mut output = String::new();
    
    if let Some((last, count)) = source.chars().fold(None, |state, char|
        match state {
            None => Some((char, 1)),
            Some((prev, count)) if prev == char => Some((prev, count + 1)),
            Some((prev, count)) => {
                encode_count(&mut output, prev, count);
                Some((char, 1))
            }            
        }
    ) {
        encode_count(&mut output, last, count);
    }
    output
}

pub fn encode_count(output: &mut String, c: char, count: usize) {
    if count == 1 {
         write!(output, "{}", c);
    } else {
         write!(output, "{}{}", count, c);
    }
}

pub fn decode(source: &str) -> String {
    let mut count = 0;
    let mut output = String::new();
    for char in source.chars() {
        if let Some(digit) = char.to_digit(10) {
            count = count * 10 + digit;
        } else if count == 0 {
            output.push(char);
        } else {            
            output.extend(repeat(char).take(count as usize));
            count = 0;
        }
    }
    output
}
