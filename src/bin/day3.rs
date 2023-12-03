use std::collections::HashMap;

static DAY: u8 = 3;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", part_sum(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

struct Schematics {
    map: HashMap<Position, char>,
}

impl Schematics {
    fn new(input: &[String]) -> Schematics {
        let mut map = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    map.insert(Position { x: x as isize, y: y as isize }, c);
                }
            }
        }
        Schematics { map }
    }

    fn has_neighboring_part(&self, pos: &Position) -> bool {
        for y in -1 ..= 1 {
            for x in -1 ..= 1 {
                if let Some(obj) = self.map.get(&Position { x: pos.x + x, y: pos.y + y }) {
                    if !obj.is_ascii_digit() {
                        return true;
                    }
                }
            }
        }
        false
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
    }
}
