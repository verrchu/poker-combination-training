use ::poker_lib::{Card, Combination, Rank, Suit};

static SUIT_CLUBS_UNICODE: &str = "\u{2660}";
static SUIT_SPADES_UNICODE: &str = "\u{2663}";
static SUIT_HEARTS_UNICODE: &str = "\u{2665}";
static SUIT_DIAMONDS_UNICODE: &str = "\u{2666}";

fn rank_as_string(rank: Rank) -> String {
    match rank {
        Rank::Two => "2".to_string(),
        Rank::Three => "3".to_string(),
        Rank::Four => "4".to_string(),
        Rank::Five => "5".to_string(),
        Rank::Six => "6".to_string(),
        Rank::Seven => "7".to_string(),
        Rank::Eight => "8".to_string(),
        Rank::Nine => "9".to_string(),
        Rank::Ten => "10".to_string(),
        Rank::Jack => "J".to_string(),
        Rank::Queen => "Q".to_string(),
        Rank::King => "K".to_string(),
        Rank::Ace => "A".to_string(),
    }
}

fn suit_as_string(suit: Suit) -> String {
    match suit {
        Suit::Clubs => SUIT_CLUBS_UNICODE.to_string(),
        Suit::Diamonds => SUIT_DIAMONDS_UNICODE.to_string(),
        Suit::Hearts => SUIT_HEARTS_UNICODE.to_string(),
        Suit::Spades => SUIT_SPADES_UNICODE.to_string(),
    }
}

pub fn card_as_string(card: Card) -> String {
    format!(
        "{}{}",
        rank_as_string(card.rank()),
        suit_as_string(card.suit())
    )
}

pub fn combination_as_string(combination: Combination) -> String {
    match combination {
        Combination::HighCard { rank: _ } => "HIGH CARD".to_string(),
        Combination::Pair { rank: _, kicker: _ } => "PAIR".to_string(),
        Combination::TwoPairs {
            low: _,
            high: _,
            kicker: _,
        } => "TWO PAIRS".to_string(),
        Combination::ThreeOfAKind { rank: _, kicker: _ } => "THREE OF A KIND".to_string(),
        Combination::Straight { rank: _ } => "STRAIGHT".to_string(),
        Combination::Flush { rank: _ } => "FLUSH".to_string(),
        Combination::FullHouse { two: _, three: _ } => "FULL HOUSE".to_string(),
        Combination::FourOfAKind { rank: _, kicker: _ } => "FOUR OF A KIND".to_string(),
        Combination::StraightFlush { rank: _ } => "STRAIGHT FLUSH".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use ::poker_lib::{Card, Rank, Suit};

    #[test]
    fn test_rank_as_string() {
        assert_eq!(super::rank_as_string(Rank::Two), "2".to_string());
        assert_eq!(super::rank_as_string(Rank::Three), "3".to_string());
        assert_eq!(super::rank_as_string(Rank::Four), "4".to_string());
        assert_eq!(super::rank_as_string(Rank::Five), "5".to_string());
        assert_eq!(super::rank_as_string(Rank::Six), "6".to_string());
        assert_eq!(super::rank_as_string(Rank::Seven), "7".to_string());
        assert_eq!(super::rank_as_string(Rank::Eight), "8".to_string());
        assert_eq!(super::rank_as_string(Rank::Nine), "9".to_string());
        assert_eq!(super::rank_as_string(Rank::Ten), "10".to_string());
        assert_eq!(super::rank_as_string(Rank::Jack), "J".to_string());
        assert_eq!(super::rank_as_string(Rank::Queen), "Q".to_string());
        assert_eq!(super::rank_as_string(Rank::King), "K".to_string());
        assert_eq!(super::rank_as_string(Rank::Ace), "A".to_string());
    }

    #[test]
    fn test_suit_as_string() {
        assert_eq!(
            super::suit_as_string(Suit::Clubs),
            super::SUIT_CLUBS_UNICODE.to_string()
        );
        assert_eq!(
            super::suit_as_string(Suit::Diamonds),
            super::SUIT_DIAMONDS_UNICODE.to_string()
        );
        assert_eq!(
            super::suit_as_string(Suit::Hearts),
            super::SUIT_HEARTS_UNICODE.to_string()
        );
        assert_eq!(
            super::suit_as_string(Suit::Spades),
            super::SUIT_SPADES_UNICODE.to_string()
        );
    }

    #[test]
    fn test_card_as_string() {
        assert_eq!(
            super::card_as_string(Card::new(Rank::Ten, Suit::Clubs)),
            format!(
                "{}{}",
                super::rank_as_string(Rank::Ten),
                super::suit_as_string(Suit::Clubs),
            )
        );
    }
}
