use std::collections::{HashMap, HashSet};

static DAY: u8 = 10;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", steps_to_farthest(&input));
    println!("{DAY}b: {}", enclosed_tiles(&input));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from(c: char) -> [Direction; 2] {
        match c {
            '|' => [Direction::South, Direction::North],
            '-' => [Direction::West, Direction::East],
            'L' => [Direction::North, Direction::East],
            'J' => [Direction::North, Direction::West],
            '7' => [Direction::South, Direction::West],
            'F' => [Direction::South, Direction::East],
            _ => panic!("unexpected pipe"),
        }
    }

    fn opposite(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

struct Map {
    start: Position,
    pipes: HashMap<Position, [Direction; 2]>,
    insides: HashSet<Position>,
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut start = Position { x: 0, y: 0 };
        let mut pipes = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Position { x: x as isize, y: y as isize };
                match c {
                    'S' => { start = pos },
                    '.' => continue,
                    pipe => { pipes.insert(pos, Direction::from(pipe)); },
                }
            }
        }
        Map { start, pipes, insides: HashSet::new() }
    }

    fn pos_connectable(&self, pos: &Position, from_direction: Direction) -> bool {
        if *pos == self.start {
            /* pipe at start is unknown; assume that it can connect to anything */
            true
        } else if let Some(directions) = self.pipes.get(pos) {
            match from_direction {
                Direction::North => { directions.contains(&Direction::South) },
                Direction::South => { directions.contains(&Direction::North) },
                Direction::East => { directions.contains(&Direction::West) },
                Direction::West => { directions.contains(&Direction::East) },
            }
        } else {
            false
        }
    }

    fn remove_disconnected_pipes(&mut self) {
        let mut disconnected_pipes = Vec::new();
        loop {
            disconnected_pipes.clear();
            for (pos, directions) in &self.pipes {
                for direction in directions {
                    match direction {
                        Direction::North => if !self.pos_connectable(&Position { x: pos.x, y: pos.y - 1 }, *direction) { disconnected_pipes.push(*pos); },
                        Direction::South => if !self.pos_connectable(&Position { x: pos.x, y: pos.y + 1 }, *direction) { disconnected_pipes.push(*pos); },
                        Direction::West => if !self.pos_connectable(&Position { x: pos.x - 1, y: pos.y }, *direction) { disconnected_pipes.push(*pos); },
                        Direction::East => if !self.pos_connectable(&Position { x: pos.x + 1, y: pos.y }, *direction) { disconnected_pipes.push(*pos); },
                    }
                }
            }
            for pos in &disconnected_pipes {
                self.pipes.remove(pos);
            }
            if disconnected_pipes.is_empty() {
                break;
            }
        }
    }

    fn other_direction(&self, pos: &Position, direction: Direction) -> Direction {
        /* get the direction from the other end of the pipe */
        let directions = self.pipes.get(pos).unwrap();
        if directions[0] == direction {
            directions[1]
        } else {
            directions[0]
        }
    }

    fn count_steps(&mut self) -> u32 {
        let mut steps = 0;

        let mut pos = self.start;
        let mut direction = Direction::North;

        /* check for possible connection from starting position */
        if self.pos_connectable(&Position { x: pos.x, y: pos.y - 1 }, Direction::North) { direction = Direction::North; }
        else if self.pos_connectable(&Position { x: pos.x, y: pos.y + 1 }, Direction::South) { direction = Direction::South; }
        else if self.pos_connectable(&Position { x: pos.x - 1, y: pos.y }, Direction::West) { direction = Direction::West; }
        else if self.pos_connectable(&Position { x: pos.x + 1, y: pos.y }, Direction::East) { direction = Direction::East; }

        let mut start_directions = Vec::from([direction]);
        let mut positions_on_path = HashSet::new();

        loop {
            positions_on_path.insert(pos);
            let next_pos = match direction {
                Direction::North => Position { x: pos.x, y: pos.y - 1 },
                Direction::South => Position { x: pos.x, y: pos.y + 1 },
                Direction::West => Position { x: pos.x - 1, y: pos.y },
                Direction::East => Position { x: pos.x + 1, y: pos.y },
            };
            steps += 1;

            if next_pos == self.start {
                start_directions.push(direction.opposite());
                break;
            }

            direction = self.other_direction(&next_pos, direction.opposite());
            pos = next_pos;
        }
        self.pipes.retain(|pos, _| positions_on_path.contains(pos));
        self.pipes.insert(self.start, [start_directions[0], start_directions[1]]);

        steps
    }

    fn enclosed_tiles(&mut self) -> usize {
        let min_y_pos = *self.pipes.keys().min_by_key(|pos| pos.y).unwrap();

        /* start from a known "outside", so that we know where the inside is */
        let mut pos = min_y_pos;
        let directions = self.pipes.get(&pos).unwrap();
        let (mut direction, other_direction) = (directions[0], directions[1]);

        let mut inside_direction = match direction {
            Direction::North => panic!("can't happen"),
            Direction::South => match other_direction {
                Direction::North => panic!("can't happen"),
                Direction::South => panic!("can't happen"),
                Direction::West => Direction::West,
                Direction::East => Direction::East,
            },
            Direction::West => match other_direction {
                Direction::North => panic!("can't happen"),
                Direction::South => Direction::South,
                Direction::West => panic!("can't happen"),
                Direction::East => Direction::South,
            },
            Direction::East => match other_direction {
                Direction::North => panic!("can't happen"),
                Direction::South => Direction::South,
                Direction::West => Direction::South,
                Direction::East => panic!("can't happen"),
            },
        };

        loop {
            let inside_pos = match inside_direction {
                Direction::North => Position { x: pos.x, y: pos.y - 1 },
                Direction::South => Position { x: pos.x, y: pos.y + 1 },
                Direction::West => Position { x: pos.x - 1, y: pos.y },
                Direction::East => Position { x: pos.x + 1, y: pos.y },
            };
            if !self.pipes.contains_key(&inside_pos) {
                self.insides.insert(inside_pos);
            }

            let next_pos = match direction {
                Direction::North => Position { x: pos.x, y: pos.y - 1 },
                Direction::South => Position { x: pos.x, y: pos.y + 1 },
                Direction::West => Position { x: pos.x - 1, y: pos.y },
                Direction::East => Position { x: pos.x + 1, y: pos.y },
            };

            if next_pos == min_y_pos {
                /* back at the beginning */
                break;
            }

            let next_direction = self.other_direction(&next_pos, direction.opposite());

            let mut inside_pos_before_curve = inside_pos;
            inside_direction = match direction {
                Direction::North => match next_direction {
                    Direction::North => inside_direction,
                    Direction::South => panic!("can't happen"),
                    Direction::West => if inside_direction == Direction::West {
                        Direction::South
                    } else {
                        inside_pos_before_curve = Position { x: next_pos.x + 1, y: next_pos.y };
                        Direction::North
                    },
                    Direction::East => if inside_direction == Direction::West {
                        inside_pos_before_curve = Position { x: next_pos.x - 1, y: next_pos.y };
                        Direction::North
                    } else {
                        Direction::South
                    },
                },
                Direction::South => match next_direction {
                    Direction::North => panic!("can't happen"),
                    Direction::South => inside_direction,
                    Direction::West => if inside_direction == Direction::West {
                        Direction::North
                    } else {
                        inside_pos_before_curve = Position { x: next_pos.x + 1, y: next_pos.y };
                        Direction::South
                    },
                    Direction::East => if inside_direction == Direction::West {
                        inside_pos_before_curve = Position { x: next_pos.x - 1, y: next_pos.y };
                        Direction::South
                    } else {
                        Direction::North
                    },
                },
                Direction::West => match next_direction {
                    Direction::North => if inside_direction == Direction::North {
                        Direction::East
                    } else {
                        inside_pos_before_curve = Position { x: next_pos.x, y: next_pos.y + 1 };
                        Direction::West
                    },
                    Direction::South => if inside_direction == Direction::North {
                        inside_pos_before_curve = Position { x: next_pos.x, y: next_pos.y - 1 };
                        Direction::West
                    } else {
                        Direction::East
                    },
                    Direction::West => inside_direction,
                    Direction::East => panic!("can't happen"),
                },
                Direction::East => match next_direction {
                    Direction::North => if inside_direction == Direction::North {
                        Direction::West
                    } else {
                        inside_pos_before_curve = Position { x: next_pos.x, y: next_pos.y + 1 };
                        Direction::East
                    },
                    Direction::South => if inside_direction == Direction::North {
                        inside_pos_before_curve = Position { x: next_pos.x, y: next_pos.y - 1 };
                        Direction::East
                    } else {
                        Direction::West
                    },
                    Direction::West => panic!("can't happen"),
                    Direction::East => inside_direction,
                },
            };
            if !self.pipes.contains_key(&inside_pos_before_curve) {
                self.insides.insert(inside_pos_before_curve);
            }

            pos = next_pos;
            direction = next_direction;
        }

        let mut insides = self.insides.clone();
        for inside in &self.insides {
            self.fill_insides(&mut insides, inside);
        }
        self.insides = insides;

        self.insides.len()
    }

    fn fill_insides(&self, insides: &mut HashSet<Position>, pos: &Position) {
        for (x, y) in [(pos.x-1, pos.y), (pos.x+1, pos.y), (pos.x, pos.y-1), (pos.x, pos.y+1)] {
            let neigh = Position { x, y };
            if self.pipes.contains_key(&neigh) || insides.contains(&neigh) {
                continue;
            }
            insides.insert(neigh);
            self.fill_insides(insides, &neigh);
        }
    }

    fn _print_map(&self) {
        let max_x = self.pipes.keys().map(|pos| pos.x).max().unwrap();
        let max_y = self.pipes.keys().map(|pos| pos.y).max().unwrap();

        for y in 0 ..= max_y {
            for x in 0 ..= max_x {
                let pos = Position { x, y };
                if pos == self.start {
                    print!("S");
                    continue;
                } else if self.insides.contains(&pos) {
                    print!("I");
                    continue;
                }
                if let Some(direction) = self.pipes.get(&pos) {
                    match direction {
                        [Direction::South, Direction::North] => { print!("|"); },
                        [Direction::West, Direction::East] => { print!("-"); },
                        [Direction::North, Direction::East] => { print!("L"); },
                        [Direction::North, Direction::West] => { print!("J"); },
                        [Direction::South, Direction::West] => { print!("7"); },
                        [Direction::South, Direction::East] => { print!("F"); },
                        _ => panic!("unexpected directions"),
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn steps_to_farthest(input: &[String]) -> u32 {
    let mut map = Map::new(input);
    map.remove_disconnected_pipes();
    map.count_steps() / 2
}

fn enclosed_tiles(input: &[String]) -> usize {
    let mut map = Map::new(input);
    map.remove_disconnected_pipes();
    /* cycle through pipes, so that only the main loop remains */
    map.count_steps();
    map.enclosed_tiles()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "-L|F7",
            "7S-7|",
            "L|7||",
            "-L-J|",
            "L|-JF",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(steps_to_farthest(&input), 4);

        let input = [
            "7-F7-",
            ".FJ|7",
            "SJLL7",
            "|F--J",
            "LJ.LJ",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(steps_to_farthest(&input), 8);
    }

    #[test]
    fn test_enclosed() {
        let input = [
            "...........",
            ".S-------7.",
            ".|F-----7|.",
            ".||.....||.",
            ".||.....||.",
            ".|L-7.F-J|.",
            ".|..|.|..|.",
            ".L--J.L--J.",
            "...........",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(enclosed_tiles(&input), 4);

        let input = [
            "..........",
            ".S------7.",
            ".|F----7|.",
            ".||....||.",
            ".||....||.",
            ".|L-7F-J|.",
            ".|..||..|.",
            ".L--JL--J.",
            "..........",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(enclosed_tiles(&input), 4);

        let input = [
            ".F----7F7F7F7F-7....",
            ".|F--7||||||||FJ....",
            ".||.FJ||||||||L7....",
            "FJL7L7LJLJ||LJ.L-7..",
            "L--J.L7...LJS7F-7L7.",
            "....F-J..F7FJ|L7L7L7",
            "....L7.F7||L7|.L7L7|",
            ".....|FJLJ|FJ|F7|.LJ",
            "....FJL-7.||.||||...",
            "....L---J.LJ.LJLJ...",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(enclosed_tiles(&input), 8);

        let input = [
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(enclosed_tiles(&input), 10);
    }
}
