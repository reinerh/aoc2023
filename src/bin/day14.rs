use std::collections::HashSet;

static DAY: u8 = 14;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", total_load(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

struct Map {
    round_rocks: HashSet<Position>,
    cube_rocks: HashSet<Position>,
    _width: isize,
    height: isize,
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut round_rocks = HashSet::new();
        let mut cube_rocks = HashSet::new();

        for (y, line) in input.iter().enumerate() {
            for (x, rock) in line.chars().enumerate() {
                let pos = Position { x: x as isize, y: y as isize };
                match rock {
                    '#' => { cube_rocks.insert(pos); }
                    'O' => { round_rocks.insert(pos); }
                    _ => {},
                }
            }
        }

        Map { round_rocks, cube_rocks, _width: input[0].len() as isize, height: input.len() as isize }
    }

    fn tilt(&mut self) {
        loop {
            let mut round_rocks = self.round_rocks.clone();

            for rock in &self.round_rocks {
                round_rocks.remove(rock);
                let mut new_pos = *rock;
                for y in (0 .. rock.y).rev() {
                    let pos = Position { x: rock.x, y };
                    if self.cube_rocks.contains(&pos) || round_rocks.contains(&pos) {
                        break;
                    }
                    new_pos = pos;
                }
                round_rocks.insert(new_pos);
            }

            if self.round_rocks == round_rocks {
                break;
            }
            self.round_rocks = round_rocks;
        }
    }

    fn load(&self) -> isize {
        self.round_rocks.iter()
                        .map(|rock| self.height - rock.y)
                        .sum()
    }
}

fn total_load(input: &[String]) -> isize {
    let mut map = Map::new(input);
    map.tilt();
    map.load()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(total_load(&input), 136);
    }
}
