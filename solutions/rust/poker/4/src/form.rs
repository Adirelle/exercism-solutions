use std::cmp::Reverse;

use crate::{card::Card, hand::Hand, rank::Rank};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Form {
    Kickers(Vec<Rank>),
    Nuplet(usize, Rank),
    Flush(Vec<Rank>),
    Straight(Rank),
}

fn extract_nuplet(cards: &mut Vec<Card>, n: usize) -> Option<Form> {
    assert!(n >= 2);
    let l = cards.len();
    if n > l {
        return None;
    }
    let max_i = l - n;
    let mut i = 0;
    while i <= max_i {
        let rank = cards[i].rank;
        let max_j = i + n;
        let mut j = i;
        while j < max_j && cards[j].rank == rank {
            j += 1
        }
        if j == max_j {
            cards.splice(i..j, []);
            return Some(Form::Nuplet(n, rank));
        }
        i += 1
    }
    None
}

fn ranks(cards: &Vec<Card>) -> Vec<Rank> {
    cards.iter().map(|&c| c.rank).collect()
}

fn find_flush(cards: &Vec<Card>) -> Option<Form> {
    if cards.len() != 5 {
        return None;
    }
    let color = cards[0].color;
    for card in cards[1..5].iter() {
        if card.color != color {
            return None;
        }
    }
    Some(Form::Flush(ranks(cards)))
}

fn find_straight(cards: &Vec<Card>) -> Option<Form> {
    if cards.len() != 5 {
        return None;
    }

    let ranks: Vec<Rank> = cards.iter().map(|&c| c.rank).collect();
    if let [Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two] = ranks[..] {
        return Some(Form::Straight(Rank::Five));
    }

    for i in 1..5 {
        if !ranks[i].is_previous_of(ranks[i - 1]) {
            return None;
        }
    }
    Some(Form::Straight(ranks[0]))
}

impl Into<Vec<Form>> for Hand {
    fn into(self) -> Vec<Form> {
        let mut cards: Vec<Card> = self.into();
        cards.sort_by_key(|c| Reverse(c.rank));

        let mut forms: Vec<Form> = Vec::new();

        if let Some(flush) = find_flush(&cards) {
            forms.push(flush);
        }

        if let Some(straight) = find_straight(&cards) {
            forms.push(straight);
        }

        for n in [4, 3, 2, 2] {
            if cards.len() < n {
                break;
            }
            if let Some(nuplet) = extract_nuplet(&mut cards, n) {
                forms.push(nuplet);
            }
        }

        if forms.is_empty() || (1..5).contains(&cards.len()) {
            forms.push(Form::Kickers(ranks(&cards)));
        }

        forms.sort_by(|a, b| a.cmp(b).reverse());

        forms
    }
}
