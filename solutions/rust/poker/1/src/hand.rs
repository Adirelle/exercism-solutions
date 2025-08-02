use std::str::FromStr;

use crate::card::Card;

pub struct Hand(Vec<Card>);

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = Vec::new();
        for card_str in s.split_whitespace() {
            cards.push(Card::from_str(card_str)?);
        }

        cards.sort_by(|a, b| a.cmp(b).reverse());

        Ok(Hand(cards))
    }
}

impl Into<Vec<Card>> for Hand {
    fn into(self) -> Vec<Card> {
        self.0.clone()
    }
}
