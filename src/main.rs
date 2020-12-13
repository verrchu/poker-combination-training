mod deck;
mod util;

use std::convert::TryFrom;

use ::poker_lib::Board;
use ::poker_lib::Card;
use ::poker_lib::Combination;
use ::poker_lib::Game;
use ::poker_lib::Hand;
use ::poker_lib::HandOf2;
use ::poker_lib::Variant;

use ::itertools::Itertools;
use ::rand::Rng;

use crate::deck::Deck;

static MIN_PLAYERS: u8 = 2;
static MAX_PLAYERS: u8 = 7;

fn main() {
    let mut rng = ::rand::thread_rng();

    loop {
        let hands_number = rng.gen_range(MIN_PLAYERS, MAX_PLAYERS + 1);

        let mut deck = Deck::new();

        let board = Board::try_from(deck.take(5)).unwrap();
        print_board(board.clone());
        let hands = (0..hands_number)
            .map(|_| HandOf2::try_from(deck.take(2)).unwrap())
            .collect::<Vec<_>>();
        print_hands(hands.clone());

        let game = Game::TexasHoldem(board, hands.clone());
        let ranked_hands = Game::rank_hands(game);

        let (winning_hand, combination, variant) = ranked_hands
            .clone()
            .into_iter()
            .max_by(|(_hand_a, comb_a, _var_a), (_hand_b, comb_b, _var_b)| comb_a.cmp(comb_b))
            .unwrap();

        let i = hands
            .into_iter()
            .enumerate()
            .find(|(_, hand)| hand.cards() == winning_hand)
            .unwrap()
            .0
            + 1;

        let n: u8 = ::text_io::read!("{}");

        if i == n.into() {
            println!("SUCCESS!");
        } else {
            println!("WRONG!");
            print_result(ranked_hands.clone());
        }
    }
}

fn print_board(board: Board) {
    println!("BOARD:");
    let cards_str = board
        .cards()
        .into_iter()
        .map(util::card_as_string)
        .format(" ");
    println!("{}", cards_str);
}

fn print_hands<H: Hand>(hands: Vec<H>) {
    println!("HANDS:");
    hands
        .into_iter()
        .map(|hand| {
            hand.cards()
                .into_iter()
                .map(util::card_as_string)
                .format(" ")
        })
        .enumerate()
        .for_each(|(n, hand_str)| println!("{}) {}", n + 1, hand_str));
}

fn print_result(hands: Vec<(Vec<Card>, Combination, Variant)>) {
    println!("RESULT:");
    hands
        .into_iter()
        .map(|(hand, comb, var)| {
            let hand_str = hand.into_iter().map(util::card_as_string).format(" ");

            let comb_str = util::combination_as_string(comb);

            let var_str = var
                .cards()
                .into_iter()
                .sorted()
                .map(util::card_as_string)
                .format(" ");

            (hand_str, comb_str, var_str)
        })
        .enumerate()
        .for_each(|(n, (hand_str, comb_str, var_str))| {
            println!("{}) {} -> {}: {}", n + 1, hand_str, comb_str, var_str);
        })
}
