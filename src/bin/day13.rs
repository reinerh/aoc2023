static DAY: u8 = 13;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", summarize_patterns(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Clone,Copy)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

impl Reflection {
    fn summary(&self) -> usize {
        match *self {
            Reflection::Vertical(column) => column,
            Reflection::Horizontal(row) => row * 100,
        }
    }
}

struct Map {
    pattern: Vec<Vec<bool>>,
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut pattern = Vec::new();
        for line in input {
            let row = line.chars()
                          .map(|c| c == '#')
                          .collect();
            pattern.push(row);
        }

        Map { pattern }
    }

    fn is_horizontal_reflection(&self, y: usize) -> bool {
        for pair in (y+1 .. self.pattern.len()).zip((0 ..= y).rev()) {
            if self.pattern[pair.0] != self.pattern[pair.1] {
                return false;
            }
        }
        true
    }

    fn is_vertical_reflection(&self, x: usize) -> bool {
        for pair in (x+1 .. self.pattern[0].len()).zip((0 ..= x).rev()) {
            for y in 0 .. self.pattern.len() {
                if self.pattern[y][pair.0] != self.pattern[y][pair.1] {
                    return false;
                }
            }
        }
        true
    }

    fn find_reflection(&self) -> Reflection {
        for y in 0 .. self.pattern.len() - 1 {
            if self.is_horizontal_reflection(y) {
                return Reflection::Horizontal(y+1)
            }
        }
        for x in 0 .. self.pattern[0].len() - 1 {
            if self.is_vertical_reflection(x) {
                return Reflection::Vertical(x+1)
            }
        }
        panic!("no reflection found");
    }
}

fn summarize_patterns(input: &[String]) -> usize {
    input.split(|line| line.is_empty())
         .map(Map::new)
         .map(|m| m.find_reflection().summary())
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(summarize_patterns(&input), 405);
    }
}
