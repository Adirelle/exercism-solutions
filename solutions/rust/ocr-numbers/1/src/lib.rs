use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, char> = HashMap::from([
        ("     |  |   ", '1'),
        (" _  _||_    ", '2'),
        (" _  _| _|   ", '3'),
        ("   |_|  |   ", '4'),
        (" _ |_  _|   ", '5'),
        (" _ |_ |_|   ", '6'),
        (" _   |  |   ", '7'),
        (" _ |_||_|   ", '8'),
        (" _ |_| _|   ", '9'),
        (" _ | ||_|   ", '0'),
    ]);
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    println!("input=\n{input}");
    let lines: Vec<&str> = input.split("\n").collect();
    Ok(
        lines[..]
            .chunks(4)
            .map(|row_lines|
                if row_lines.len() != 4 {
                    Err(Error::InvalidRowCount(row_lines.len()))
                } else if let Some(invalid) = row_lines.iter().find(|l| l.len() % 3 != 0) {
                    Err(Error::InvalidColumnCount(invalid.len()))
                } else {
                    Ok(
                        (0..(row_lines[0].len()/3))
                        .map(|i| {
                            let digit: String = row_lines
                                .iter()
                                .flat_map(|l| l.chars().skip(i * 3).take(3))
                                .collect();
                            let digit_ref: &str = digit.as_ref();
                            NUMBERS.get(digit_ref).unwrap_or(&'?')
                        })
                        .collect::<String>()
                    )
                }
            )
            .collect::<Result<Vec<_>, _>>()?
            .join(",")
    )
}
