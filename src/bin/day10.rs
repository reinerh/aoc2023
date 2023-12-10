use std::collections::HashMap;

static DAY: u8 = 10;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", steps_to_farthest(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
        Map { start, pipes }
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

    fn count_steps(&self) -> u32 {
        let mut steps = 0;

        let mut pos = self.start;
        let mut direction = Direction::North;

        /* check for possible connection from starting position */
        if self.pos_connectable(&Position { x: pos.x, y: pos.y - 1 }, Direction::North) { direction = Direction::North; }
        else if self.pos_connectable(&Position { x: pos.x, y: pos.y + 1 }, Direction::South) { direction = Direction::South; }
        else if self.pos_connectable(&Position { x: pos.x - 1, y: pos.y }, Direction::West) { direction = Direction::West; }
        else if self.pos_connectable(&Position { x: pos.x + 1, y: pos.y }, Direction::East) { direction = Direction::East; }

        loop {
            let next_pos = match direction {
                Direction::North => Position { x: pos.x, y: pos.y - 1 },
                Direction::South => Position { x: pos.x, y: pos.y + 1 },
                Direction::West => Position { x: pos.x - 1, y: pos.y },
                Direction::East => Position { x: pos.x + 1, y: pos.y },
            };
            steps += 1;

            if next_pos == self.start {
                break;
            }

            direction = self.other_direction(&next_pos, direction.opposite());
            pos = next_pos;
        }

        steps
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
}
