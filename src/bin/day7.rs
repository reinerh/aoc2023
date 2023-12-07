use std::{cmp::Ordering, collections::HashMap};

static DAY: u8 = 7;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", winnings(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
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
            'J' => Card::Jack,
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
        HandType::from(self)
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
        assert_eq!(winnings(&input), 6440);
    }
}
