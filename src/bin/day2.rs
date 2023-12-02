use std::collections::HashMap;
use regex::Regex;

static DAY: u8 = 2;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sum_possible(&input));
    println!("{DAY}b: {}", 0);
}

struct Game {
    id: usize,
    infos: Vec<HashMap<String, usize>>,
}

impl Game {
    fn new(input: &str) -> Game {
        let re = Regex::new("^Game ([0-9]+): (.*)").unwrap();
        let caps = re.captures(input).unwrap();

        let id = caps[1].parse().unwrap();
        let mut infos = Vec::new();
        for info in caps[2].split("; ") {
            let mut color_amount = HashMap::new();
            for color_info in info.split(", ") {
                let (amount, color) = color_info.split_once(' ').unwrap();
                let amount = amount.parse().unwrap();
                color_amount.insert(color.to_string(), amount);
            }
            infos.push(color_amount);
        }
        Game { id, infos }
    }

    fn is_possible(&self, bagged: &HashMap<String, usize>) -> bool {
        for info in &self.infos {
            for (color, amount) in info {
                if bagged[color] < *amount {
                    return false
                }
            }
        }
        true
    }
}

fn sum_possible(input: &[String]) -> usize {
    let games = input.iter()
                     .map(|x| Game::new(x))
                     .collect::<Vec<Game>>();
    let bag_content = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    games.iter()
         .filter(|x| x.is_possible(&bag_content))
         .map(|x| x.id)
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
         ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(sum_possible(&input), 8);
    }
}
