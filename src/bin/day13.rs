use std::collections::HashSet;

static DAY: u8 = 13;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", summarize_patterns(&input));
    println!("{DAY}b: {}", summarize_patterns_with_smudge(&input));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

impl Reflection {
    fn summary(&self) -> usize {
        match *self {
            Reflection::Vertical(column) => column + 1,
            Reflection::Horizontal(row) => (row + 1) * 100,
        }
    }
}

#[derive(Clone)]
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

    fn _print_map(&self) {
        for y in 0 .. self.pattern.len() {
            for x in 0 .. self.pattern[0].len() {
                if self.pattern[y][x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
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

    fn find_reflections(&self) -> HashSet<Reflection> {
        let mut reflections = HashSet::new();
        for y in 0 .. self.pattern.len() - 1 {
            if self.is_horizontal_reflection(y) {
                reflections.insert(Reflection::Horizontal(y));
            }
        }
        for x in 0 .. self.pattern[0].len() - 1 {
            if self.is_vertical_reflection(x) {
                reflections.insert(Reflection::Vertical(x));
            }
        }
        reflections
    }

    fn find_reflection_with_smudge(&self) -> Reflection {
        let orig_reflection = self.find_reflections();
        for y in 0 .. self.pattern.len() {
            for x in 0 .. self.pattern[0].len() {
                let mut map = self.clone();
                map.pattern[y][x] = !map.pattern[y][x];
                let reflections = map.find_reflections();
                let new_reflections = reflections.difference(&orig_reflection).collect::<Vec<_>>();
                if !new_reflections.is_empty() {
                    return *new_reflections[0];
                }
           }
        }
        panic!("no reflection found");
    }
}

fn summarize_patterns(input: &[String]) -> usize {
    input.split(|line| line.is_empty())
         .map(Map::new)
         .map(|m| m.find_reflections().iter().next().unwrap().summary())
         .sum()
}

fn summarize_patterns_with_smudge(input: &[String]) -> usize {
    input.split(|line| line.is_empty())
         .map(Map::new)
         .map(|m| m.find_reflection_with_smudge().summary())
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
        assert_eq!(summarize_patterns_with_smudge(&input), 400);
    }
}
