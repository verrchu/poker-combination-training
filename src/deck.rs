use ::itertools::Itertools;
use ::rand::Rng;

use ::poker_lib::Card;
use ::poker_lib::Rank;
use ::poker_lib::Suit;

static DECK_SIZE: u8 = 52;

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        let cards = Rank::list()
            .into_iter()
            .cartesian_product(Suit::list().into_iter())
            .map(|(rank, suit)| Card::new(rank, suit))
            .collect();

        Self(cards)
    }

    pub fn take(&mut self, n: u8) -> Vec<Card> {
        let mut cards: Vec<Card> = vec![];

        let mut rng = ::rand::thread_rng();

        for _ in 0..n {
            let index = rng.gen_range(0, self.0.len());
            let card = self.0.remove(index);
            cards.push(card);
        }

        cards
    }
}

#[cfg(test)]
mod tests {
    use ::itertools::Itertools;
    use ::poker_lib::{Rank, Suit};
    use ::rand::Rng;

    use super::Deck;
    use super::DECK_SIZE;

    #[test]
    fn test_new() {
        let deck = Deck::new();

        assert_eq!(deck.0.len(), DECK_SIZE as usize);

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

    #[test]
    fn test_take() {
        let mut deck = Deck::new();

        let mut rng = ::rand::thread_rng();
        let n = rng.gen_range(1, DECK_SIZE + 1);

        let cards = deck.take(n);

        assert_eq!(deck.0.len(), (DECK_SIZE - n) as usize);
        assert_eq!(cards.len(), n as usize);

        assert_eq!(
            cards.clone().into_iter().unique().collect::<Vec<_>>(),
            cards
        );

        for card in cards {
            assert!(!deck.0.to_vec().contains(&card));
        }
    }
}
