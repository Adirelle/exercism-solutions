#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    Encoder::new(values.into_iter().copied()).collect()
}

struct Encoder<I> {
    upstream: I,
    current: Option<(u32, u32)>,
}

impl<I: Iterator<Item = u32>> Encoder<I> {
    fn new(upstream: I) -> Self {
        Self { upstream, current: None }
    }
}

impl<I: Iterator<Item = u32>> Iterator for Encoder<I> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.current.is_none() {
            let next = self.upstream.next()?;
            self.current = Some(if next > 0 {
                (next, next.ilog(0x80))
            } else {
                (0, 0)
            });
        }
        match self.current {
            Some((value, 0)) => {
                self.current = None;
                Some((value & 0x7F) as u8)
            }
            Some((value, ref mut pos)) => {
                let shift = 7 * *pos;
                *pos -= 1;
                Some(0x80 | (value >> shift) as u8 & 0x7F)
            }
            None => unreachable!()
        }
    }
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut buf = None;
    let mut result = Vec::new();
    for &b in bytes.into_iter() {
        if buf.is_none() {
            buf = Some(0);
        }
        if b > 127_u8 { 
            let buf = buf.as_mut().unwrap();
            *buf = (*buf + ((b as u32) & 0x7F)) << 7;
        } else {
            result.push(buf.take().unwrap() + b as u32);
        }
    }
    if buf.is_none() {
        Ok(result)
    } else {
        Err(Error::IncompleteNumber)
    }
}
