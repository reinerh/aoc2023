use std::collections::HashMap;

static DAY: u8 = 8;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", required_steps(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

struct Choice {
    left: String,
    right: String,
}

impl Choice {
    fn from(input: &str) -> Choice {
        /* "(AAA, ZZZ)" */
        let tmp = input.chars()
                       .skip(1)
                       .take(input.len()-2)
                       .collect::<String>();
        let (left, right) = tmp.split_once(", ").unwrap();
        Choice { left: left.to_string(), right: right.to_string() }
    }
}

struct Map {
    instructions: Vec<Direction>,
    map: HashMap<String, Choice>,
}

impl Map {
    fn new(input: &[String]) -> Map {
        let instructions = input[0].chars()
                                   .map(Direction::from)
                                   .collect();

        let mut map = HashMap::new();
        for line in input.iter().skip(2) {
            let (from, to) = line.split_once(" = ").unwrap();
            let choice = Choice::from(to);
            map.insert(from.to_string(), choice);
        }
        Map { instructions, map }
    }

    fn get_direction(&self, step: usize) -> Direction {
        let i = step % self.instructions.len();
        self.instructions[i]
    }

    fn next_pos(&self, pos: &str, steps: usize) -> String {
        let choice = self.map.get(pos).unwrap();
        match self.get_direction(steps) {
            Direction::Left => choice.left.clone(),
            Direction::Right => choice.right.clone(),
        }
    }

    fn number_steps(&self, from: &str, to: &str) -> usize {
        let mut steps = 0;
        let mut pos = from.to_string();
        while pos != to {
            pos = self.next_pos(&pos, steps);
            steps += 1;
        }
        steps
    }
}

fn required_steps(input: &[String]) -> usize {
    let map = Map::new(input);
    map.number_steps("AAA", "ZZZ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(required_steps(&input), 2);

        let input = [
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(required_steps(&input), 6);
    }
}
