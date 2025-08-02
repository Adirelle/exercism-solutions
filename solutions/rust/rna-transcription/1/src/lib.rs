#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<DnaNucleotide>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<RnaNucleotide>);

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum DnaNucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum RnaNucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Uracil,
}

impl TryFrom<char> for DnaNucleotide {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::Adenine,
            'C' => Self::Cytosine,
            'G' => Self::Guanine,
            'T' => Self::Thymine,
            _ => return Err(()),
        })
    }
}

impl TryFrom<char> for RnaNucleotide {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::Adenine,
            'C' => Self::Cytosine,
            'G' => Self::Guanine,
            'U' => Self::Uracil,
            _ => return Err(()),
        })
    }
}

fn from_str<T: TryFrom<char>>(s: &str) -> Result<Vec<T>, usize> {
    let mut v: Vec<T> = Vec::new();
    for (i, c) in s.chars().enumerate() {
        match T::try_from(c) {
            Ok(n) => v.push(n),
            Err(_) => return Err(i),
        }
    }
    Ok(v)
}

impl DnaNucleotide {
    fn transcribe(&self) -> RnaNucleotide {
        use RnaNucleotide::*;
        match self {
            Self::Guanine => Cytosine,
            Self::Cytosine => Guanine,
            Self::Thymine => Adenine,
            Self::Adenine => Uracil,
        }
    }
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        from_str(dna).map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        let v: Vec<RnaNucleotide> = self.0.iter().map(DnaNucleotide::transcribe).collect();
        Rna(v)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        from_str(rna).map(Rna)
    }
}
