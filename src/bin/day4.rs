use std::collections::HashSet;

static DAY: u8 = 4;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", total_points(&input));
    println!("{DAY}b: {}", 0);
}

struct Card {
    winning_numbers: HashSet<u32>,
    numbers_you_have: HashSet<u32>,
}

impl Card {
    fn new(input: &str) -> Card {
        let (_id, numbers) = input.split_once(": ").unwrap();
        let (win, have) = numbers.split_once(" | ").unwrap();
        let winning_numbers = win.split(' ')
                                 .filter(|x| !x.is_empty())
                                 .map(|x| x.parse().unwrap())
                                 .collect::<HashSet<_>>();
        let numbers_you_have = have.split(' ')
                                   .filter(|x| !x.is_empty())
                                   .map(|x| x.parse().unwrap())
                                   .collect::<HashSet<_>>();
        Card { winning_numbers, numbers_you_have }
    }

    fn matching_numbers(&self) -> HashSet<u32> {
        self.winning_numbers.intersection(&self.numbers_you_have).copied().collect()
    }

    fn points(&self) -> u32 {
        (1 << self.matching_numbers().len()) / 2
    }
}

fn total_points(input: &[String]) -> u32 {
    input.iter()
         .map(|x| Card::new(x))
         .map(|c| c.points())
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(total_points(&input), 13);
    }
}
