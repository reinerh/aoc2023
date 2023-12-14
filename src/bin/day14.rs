use std::collections::{HashSet, HashMap};

static DAY: u8 = 14;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", total_load(&input));
    println!("{DAY}b: {}", total_load_cycles(&input));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Position {
    x: isize,
    y: isize,
}

struct Map {
    round_rocks: HashSet<Position>,
    cube_rocks: HashSet<Position>,
    width: isize,
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

        Map { round_rocks, cube_rocks, width: input[0].len() as isize, height: input.len() as isize }
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

    fn cycle(&mut self) {
        let rotate_right = |rock: Position, width: isize| -> Position {
            Position {
                x: width - rock.y - 1,
                y: rock.x,
            }
        };

        for _ in 0 .. 4 {
            self.tilt();

            self.round_rocks = self.round_rocks.iter()
                                               .map(|&rock| rotate_right(rock, self.width))
                                               .collect();
            self.cube_rocks = self.cube_rocks.iter()
                                             .map(|&rock| rotate_right(rock, self.width))
                                             .collect();
        }
    }

    fn cycles(&mut self, amount: isize) {
        let mut possible_maps = HashMap::new();
        loop {
            let mut rocks = self.round_rocks.iter().cloned().collect::<Vec<_>>();
            rocks.sort_unstable();
            let entry = possible_maps.entry(rocks).or_insert(0);
            if *entry == 2 {
                /* found a loop */
                break;
            }
            *entry += 1;
            self.cycle();
        }
        let before_loop = possible_maps.values().filter(|&x| *x == 1).count() as isize;
        let loop_len = possible_maps.values().filter(|&x| *x == 2).count() as isize;

        let amount = amount - before_loop;
        let remaining = amount % loop_len;
        for _ in 0 .. remaining {
            self.cycle();
        }
    }

    fn load(&self) -> isize {
        self.round_rocks.iter()
                        .map(|rock| self.height - rock.y)
                        .sum()
    }

    fn _print_map(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                let pos = Position { x, y };
                if self.round_rocks.contains(&pos) {
                    assert!(!self.cube_rocks.contains(&pos));
                    print!("O");
                } else if self.cube_rocks.contains(&pos) {
                    assert!(!self.round_rocks.contains(&pos));
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn total_load(input: &[String]) -> isize {
    let mut map = Map::new(input);
    map.tilt();
    map.load()
}

fn total_load_cycles(input: &[String]) -> isize {
    let mut map = Map::new(input);
    map.cycles(1_000_000_000);
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
        assert_eq!(total_load_cycles(&input), 64);
    }
}
