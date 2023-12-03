use std::collections::{HashMap, HashSet};

static DAY: u8 = 3;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", part_sum(&input));
    println!("{DAY}b: {}", gear_ratio_sum(&input));
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Position {
    x: isize,
    y: isize,
}

struct Schematics {
    part_positions: HashMap<Position, char>,
}

impl Schematics {
    fn new(input: &[String]) -> Schematics {
        let mut part_positions = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let pos = Position { x: x as isize, y: y as isize };
                    if !c.is_ascii_digit() {
                        part_positions.insert(pos, c);
                    }
                }
            }
        }
        Schematics { part_positions }
    }

    fn get_neighboring_parts(&self, pos: &Position) -> HashMap<Position, char> {
        let mut parts = HashMap::new();
        for y in -1 ..= 1 {
            for x in -1 ..= 1 {
                if let Some(part) = self.part_positions.get(&Position { x: pos.x + x, y: pos.y + y}) {
                    parts.insert(Position { x: pos.x + x, y: pos.y + y }, *part);
                }
            }
        }
        parts
    }

    fn has_neighboring_part(&self, pos: &Position) -> bool {
        !self.get_neighboring_parts(pos).is_empty()
    }
}

fn part_sum(input: &[String]) -> u32 {
    let schematics = Schematics::new(input);
    let mut part_numbers = Vec::new();

    for (y, line) in input.iter().enumerate() {
        let mut number = 0;
        let mut is_part_number = false;
        for (x, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                number *= 10;
                number += digit;
                if schematics.has_neighboring_part(&Position { x: x as isize, y: y as isize }) {
                    is_part_number = true;
                }
            } else if number > 0 {
                /* number ended within a line */
                if is_part_number {
                    part_numbers.push(number);
                }
                number = 0;
                is_part_number = false;
            }
        }
        if number > 0 && is_part_number {
            /* number ended at the end of line */
            part_numbers.push(number);
        }
    }

    part_numbers.iter()
                .sum()
}

fn gear_ratio_sum(input: &[String]) -> u32 {
    let schematics = Schematics::new(input);
    let mut gear_neighbors = HashMap::<Position, Vec<u32>>::new();

    for (y, line) in input.iter().enumerate() {
        let mut number = 0;
        let mut neighbors = HashSet::new();
        for (x, c) in line.chars().enumerate() {
            let pos = Position { x: x as isize, y: y as isize };
            if let Some(digit) = c.to_digit(10) {
                number *= 10;
                number += digit;
                for (pos, c) in schematics.get_neighboring_parts(&pos) {
                    if c == '*' {
                        neighbors.insert(pos);
                    }
                }
            } else if number > 0 {
                /* number ended within a line */
                for gear in &neighbors {
                    gear_neighbors.entry(*gear).or_default().push(number);
                }
                number = 0;
                neighbors.clear();
            }
        }
        if number > 0 {
            /* number ended at the end of line */
            for gear in &neighbors {
                gear_neighbors.entry(*gear).or_default().push(number);
            }
        }
    }

    gear_neighbors.iter()
                  .filter(|(_, numbers)| numbers.len() == 2)
                  .map(|(_, numbers)| numbers.iter().product::<u32>())
                  .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(part_sum(&input), 4361);
        assert_eq!(gear_ratio_sum(&input), 467835);
    }
}
