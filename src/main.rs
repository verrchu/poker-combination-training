mod deck;

static SUIT_CLUBS_UNICODE: &str = "\u{2660}";
static SUIT_SPADES_UNICODE: &str = "\u{2663}";
static SUIT_HEARTS_UNICODE: &str = "\u{2665}";
static SUIT_DIAMONDS_UNICODE: &str = "\u{2666}";

fn main() {
    println!("CLUBS -> {}", SUIT_CLUBS_UNICODE);
    println!("DIAMONDS -> {}", SUIT_DIAMONDS_UNICODE);
    println!("HEARTS -> {}", SUIT_HEARTS_UNICODE);
    println!("SPADES -> {}", SUIT_SPADES_UNICODE);
}
