use std::{cmp::Ordering, collections::HashMap};
use std::sync::Mutex;

static DAY: u8 = 7;

static J_IS_JOKER: Mutex<bool> = Mutex::new(false);

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", winnings_no_joker(&input));
    println!("{DAY}b: {}", winnings_joker(&input));
}

fn j_is_joker() -> bool {
    *J_IS_JOKER.lock().unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(c: char) -> Card {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => if j_is_joker() { Card::Joker } else { Card::Jack },
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("invalid card character"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from(hand: &Hand) -> HandType {
        let mut card_counts = HashMap::new();
        for card in &hand.cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let has_count = |c| {
           card_counts.values().any(|x| *x == c)
        };
        if has_count(5) {
            HandType::FiveOfAKind
        } else if has_count(4) {
            HandType::FourOfAKind
        } else if has_count(3) && has_count(2) {
            HandType::FullHouse
        } else if has_count(3) {
            HandType::ThreeOfAKind
        } else if has_count(2) && card_counts.len() == 3 {
            HandType::TwoPair
        } else if has_count(2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn from_joker(hand: &Hand) -> HandType {
        let mut card_counts = HashMap::new();
        for card in &hand.cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let has_count = |c| {
            card_counts.iter().filter(|&(&card, &count)| *card != Card::Joker && count == c).count() > 0
        };
        let jokers = *card_counts.get(&Card::Joker).unwrap_or(&0);
        if has_count(5) || (has_count(4) && jokers >= 1) || (has_count(3) && jokers >= 2) || (has_count(2) && jokers >= 3) || jokers >= 4 {
            HandType::FiveOfAKind
        } else if has_count(4) || (has_count(3) && jokers >= 1) || (has_count(2) && jokers >= 2) || jokers >= 3 {
            HandType::FourOfAKind
        } else if (has_count(3) && has_count(2)) || (has_count(2) && jokers == 1 && card_counts.len() == 3) {
            HandType::FullHouse
        } else if has_count(3) || (has_count(2) && jokers >= 1) || jokers >= 2 {
            HandType::ThreeOfAKind
        } else if has_count(2) && card_counts.len() == 3 {
            HandType::TwoPair
        } else if has_count(2) || (card_counts.len() == 5 && jokers == 1) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn new(input: &str) -> Hand {
        let (values, bid) = input.split_once(' ').unwrap();
        assert_eq!(values.len(), 5);
        let mut cards = [Card::Two; 5];
        for (i, c) in values.chars().enumerate() {
            cards[i] = Card::new(c);
        }
        let bid = bid.parse().unwrap();

        Hand { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        if j_is_joker() {
            HandType::from_joker(self)
        } else {
            HandType::from(self)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        if self_type == other_type {
            /* same hand type -> first highest card wins */
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if self_card == other_card {
                    continue;
                }
                return self_card.cmp(other_card);
            }
            panic!("no ordering found for hand");
        }
        self_type.cmp(&other_type)
    }
}

fn winnings(input: &[String]) -> u32 {
    let mut hands = input.iter()
                     .map(|x| Hand::new(x))
                     .collect::<Vec<_>>();
    hands.sort_unstable();

    hands.iter()
         .enumerate()
         .map(|(i, hand)| (i as u32 + 1) * hand.bid)
         .sum()
}

fn winnings_no_joker(input: &[String]) -> u32 {
    *J_IS_JOKER.lock().unwrap() = false;
    winnings(input)
}

fn winnings_joker(input: &[String]) -> u32 {
    *J_IS_JOKER.lock().unwrap() = true;
    winnings(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(winnings_no_joker(&input), 6440);
        assert_eq!(winnings_joker(&input), 5905);
    }
}
