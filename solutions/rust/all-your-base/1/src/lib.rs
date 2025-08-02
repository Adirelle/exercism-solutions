#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        Err(Error::InvalidInputBase)
    } else if to_base < 2 {
        Err(Error::InvalidOutputBase)
    } else if from_base == to_base {
        Ok(number.to_owned())
    } else {
        Ok(convert_to_base(convert_from_base(number, from_base)?, to_base))
    }
}

fn convert_from_base(number: &[u32], base: u32) -> Result<u32, Error> {
    let mut result = 0;
    for digit in number.iter() {
        if *digit >= base {
            return Err(Error::InvalidDigit(*digit)) 
        }
        result = result * base + digit;
    }
    Ok(result)
}

fn convert_to_base(mut number: u32, base: u32) -> Vec<u32> {
    let mut result = Vec::new();
    loop {
        result.push(number % base);
        number /= base;
        if number == 0 {
            result.reverse();
            break result;
        }
    }
}