#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Error {
    InvalidPitch,
    InvalidInterval,
}

static SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

static FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

static INTERVALS: [char; 3] = ['m', 'M', 'A'];

pub struct Scale(Vec<String>);

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let Scale(chromatic) = Self::chromatic(tonic)?;
        let mut i = 0;
        let mut scale: Vec<String> = Vec::new();
        scale.push(chromatic[0].clone());
        for interval in intervals.chars() {
            if let Some(offset) = INTERVALS.iter().position(|&c| c == interval) {
                i = (i + 1 + offset) % 12;
            } else {
                return Err(Error::InvalidInterval);
            }
            scale.push(chromatic[i].clone());
        }
        Ok(Scale(scale))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let pitches = Self::chromatic_pitches(tonic)?;
        let offset = pitches
            .iter()
            .position(|&p| p.eq_ignore_ascii_case(tonic))
            .ok_or(Error::InvalidPitch)?;
        let mut scale: Vec<String> = Vec::new();
        for i in 0..=12 {
            scale.push(pitches[(offset + i) % 12].into());
        }
        Ok(Scale(scale))
    }

    fn chromatic_pitches(tonic: &str) -> Result<&[&str; 12], Error> {
        Ok(match tonic {
            "a" | "A" | "B" | "C" | "D" | "E" | "G" | "F#" | "b" | "c#" | "d#" | "e" | "f#"
            | "g#" => &SHARPS,
            "d" | "F" | "Ab" | "bb" | "Bb" | "c" | "Db" | "eb" | "Eb" | "f" | "g" | "Gb" => &FLATS,
            _ => return Err(Error::InvalidPitch),
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}
