use std::{fmt::Display, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Color {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Default for Color {
    fn default() -> Self {
        Color::Club
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;
        match self {
            Club => write!(f, "C"),
            Diamond => write!(f, "D"),
            Heart => write!(f, "H"),
            Spade => write!(f, "S"),
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Color::*;
        match s {
            "C" => Ok(Club),
            "D" => Ok(Diamond),
            "H" => Ok(Heart),
            "S" => Ok(Spade),
            _ => Err(format!("unknown color: {}", s)),
        }
    }
}
