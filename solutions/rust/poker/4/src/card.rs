use std::{fmt::Display, str::FromStr};

use crate::{color::Color, rank::Rank};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Card {
    pub rank: Rank,
    pub color: Color,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.color)
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.len() {
            2 => Card {
                rank: Rank::from_str(&s[0..1])?,
                color: Color::from_str(&s[1..2])?,
            },
            3 => Card {
                rank: Rank::from_str(&s[0..2])?,
                color: Color::from_str(&s[2..3])?,
            },
            _ => return Err(format!("invalid card: {}", s)),
        })
    }
}

impl From<Card> for Rank {
    fn from(c: Card) -> Self {
        c.rank
    }
}

impl From<Card> for Color {
    fn from(c: Card) -> Self {
        c.color
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}
