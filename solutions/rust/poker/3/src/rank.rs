use enum_iterator::{first, Sequence};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Sequence, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn is_previous_of(self, other: Rank) -> bool {
        match other.previous() {
            Some(v) if v == self => true,
            _ => false,
        }
    }
}

impl Default for Rank {
    fn default() -> Self {
        first::<Rank>().unwrap()
    }
}

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rank::*;
        match s {
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "10" => Ok(Ten),
            "J" => Ok(Jack),
            "Q" => Ok(Queen),
            "K" => Ok(King),
            "A" => Ok(Ace),
            _ => Err(format!("unknown rank: {}", s)),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Rank::*;
        match self {
            Two => write!(f, "2"),
            Three => write!(f, "3"),
            Four => write!(f, "4"),
            Five => write!(f, "5"),
            Six => write!(f, "6"),
            Seven => write!(f, "7"),
            Eight => write!(f, "8"),
            Nine => write!(f, "9"),
            Ten => write!(f, "10"),
            Jack => write!(f, "J"),
            Queen => write!(f, "Q"),
            King => write!(f, "K"),
            Ace => write!(f, "A"),
        }
    }
}
