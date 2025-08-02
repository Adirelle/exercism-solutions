mod card;
mod color;
mod form;
mod hand;
mod poker_hand;
mod rank;

use crate::poker_hand::PokerHand;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.is_empty() {
        return Vec::new();
    }

    let mut entries: Vec<(&'a str, PokerHand)> = hands.iter().map(|&s| (s, s.into())).collect();

    entries.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{:?}", entries);

    let winner = entries.pop().unwrap();
    let mut winners = vec![winner.0];

    while let Some(more) = entries.pop() {
        if more.1.cmp(&winner.1).is_eq() {
            winners.push(more.0);
        } else {
            break;
        }
    }

    winners
}
