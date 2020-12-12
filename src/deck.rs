use ::itertools::Itertools;

use ::poker_lib::Card;
use ::poker_lib::Rank;
use ::poker_lib::Suit;

pub struct Deck(Vec<Card>);

impl Deck {
    fn new() -> Self {
        let cards = Rank::list()
            .into_iter()
            .cartesian_product(Suit::list().into_iter())
            .map(|(rank, suit)| Card::new(rank, suit))
            .collect();

        Self(cards)
    }
}

#[cfg(test)]
mod tests {
    use ::itertools::Itertools;
    use ::poker_lib::{Rank, Suit};

    use super::Deck;

    #[test]
    fn test_new() {
        let deck = Deck::new();

        assert_eq!(deck.0.len(), 52);

        for suit in Suit::list() {
            let ranks = deck
                .0
                .iter()
                .filter(|card| card.suit() == suit)
                .map(|card| card.rank())
                .sorted()
                .collect::<Vec<_>>();

            assert_eq!(ranks.len(), 13);

            assert_eq!(ranks, Rank::list());
        }
    }
}
