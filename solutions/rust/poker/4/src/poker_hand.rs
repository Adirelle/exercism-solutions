use crate::{form::Form, hand::Hand, rank::Rank};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum PokerHand {
    HighCard(Vec<Rank>),
    OnePair(Rank, Vec<Rank>),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Vec<Rank>),
    Straight(Rank),
    Flush(Vec<Rank>),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
}

impl From<&str> for PokerHand {
    fn from(s: &str) -> Self {
        let hand: Hand = s.parse().unwrap();
        let forms: Vec<Form> = hand.into();
        PokerHand::from(forms)
    }
}

impl From<Vec<Form>> for PokerHand {
    fn from(forms: Vec<Form>) -> Self {
        use Form::*;
        use PokerHand::*;

        dbg!(&forms);
        let ph = match &forms[..] {
            [Form::Straight(a), Form::Flush(_)] => StraightFlush(*a),
            [Nuplet(4, a), Kickers(b)] => FourOfAKind(*a, b[0]),
            [Nuplet(3, a), Nuplet(2, b)] => FullHouse(*a, *b),
            [Form::Flush(a)] => PokerHand::Flush(a.clone()),
            [Form::Straight(a)] => PokerHand::Straight(*a),
            [Nuplet(3, a), Kickers(c)] => ThreeOfAKind(*a, c.clone()),
            [Nuplet(2, a), Nuplet(2, b), Kickers(c)] => TwoPair(*a, *b, c[0]),
            [Nuplet(2, a), Kickers(b)] => OnePair(*a, b.clone()),
            [Kickers(a)] => HighCard(a.clone()),
            _ => todo!(),
        };

        dbg!(ph)
    }
}
