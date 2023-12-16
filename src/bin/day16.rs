use std::collections::HashMap;

static DAY: u8 = 16;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", energized_tiles(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Mirror {
    Backward,
    Forward,
}

impl Mirror {
    fn next_direction(&self, direction: Direction) -> Direction {
        match self {
            Mirror::Backward => match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            Mirror::Forward => match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
        }
    }
}

enum Splitter {
    Horizontal,
    Vertical,
}

impl Splitter {
    fn new_directions(&self, direction: Direction) -> Vec<Direction> {
        let mut directions = Vec::new();
        match self {
            Splitter::Horizontal => match direction {
                Direction::Up => { directions.push(Direction::Left); directions.push(Direction::Right); },
                Direction::Down => { directions.push(Direction::Left); directions.push(Direction::Right); },
                Direction::Left => { directions.push(direction); },
                Direction::Right => { directions.push(direction); },
            },
            Splitter::Vertical => match direction {
                Direction::Up => { directions.push(direction); },
                Direction::Down => { directions.push(direction); },
                Direction::Left => { directions.push(Direction::Up); directions.push(Direction::Down); },
                Direction::Right => { directions.push(Direction::Up); directions.push(Direction::Down); },
            },
        }
        directions
    }
}

enum Object {
    Mirror(Mirror),
    Splitter(Splitter),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: Position,
    direction: Direction,
}

impl Object {
    fn from(c: char) -> Option<Object> {
        match c {
            '\\' => Some(Object::Mirror(Mirror::Backward)),
            '/' => Some(Object::Mirror(Mirror::Forward)),
            '-' => Some(Object::Splitter(Splitter::Horizontal)),
            '|' => Some(Object::Splitter(Splitter::Vertical)),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn next_pos(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position { x: self.x, y: self.y - 1 },
            Direction::Down => Position { x: self.x, y: self.y + 1 },
            Direction::Left => Position { x: self.x - 1, y: self.y },
            Direction::Right => Position { x: self.x + 1, y: self.y },
        }
    }
}

struct Map {
    map: HashMap<Position, Object>,
    energized: HashMap<Position, Vec<Direction>>,
    width: isize,
    height: isize,
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut map = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Position { x: x as isize, y: y as isize };
                if let Some(obj) = Object::from(c) {
                    map.insert(pos, obj);
                }
            }
        }
        Map {
            map,
            energized: HashMap::new(),
            width: input[0].len() as isize,
            height: input.len() as isize,
        }
    }

    fn outside_map(&self, pos: Position) -> bool {
        pos.x < 0 || pos.x >= self.width || pos.y < 0 || pos.y >= self.height
    }

    fn energize(&mut self, beam: Beam) {
        let mut beams = Vec::from([beam]);
        while !beams.is_empty() {
            let mut beam_heads = Vec::new();
            for beam in beams {
                if self.outside_map(beam.pos) {
                    continue;
                }
                let energized = self.energized.entry(beam.pos).or_default();
                if energized.contains(&beam.direction) {
                    /* identical beam already exists; current beam is part of a loop */
                    continue;
                } else {
                    energized.push(beam.direction);
                }

                let mut beam = beam;
                if let Some(obj) = self.map.get(&beam.pos) {
                    match obj {
                        Object::Mirror(mirror) => {
                            beam.direction = mirror.next_direction(beam.direction);
                            beam.pos = beam.pos.next_pos(beam.direction);
                            beam_heads.push(beam);
                        },
                        Object::Splitter(splitter) => {
                            for direction in splitter.new_directions(beam.direction) {
                                beam_heads.push(Beam { pos: beam.pos.next_pos(direction), direction });
                            }
                        },
                    }
                } else {
                    beam.pos = beam.pos.next_pos(beam.direction);
                    beam_heads.push(beam);
                }
            }
            beams = beam_heads;
        }
    }

    fn tiles_energized(&self) -> usize {
        self.energized.keys().len()
    }

    fn _print_map(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                if let Some(directions) = self.energized.get(&Position { x, y }) {
                    if directions.len() == 1 {
                        match directions[0] {
                            Direction::Up => print!("^"),
                            Direction::Down => print!("v"),
                            Direction::Left => print!("<"),
                            Direction::Right => print!(">"),
                        }
                    } else {
                        print!("{}", directions.len());
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn energized_tiles(input: &[String]) -> usize {
    let mut map = Map::new(input);
    map.energize(Beam { pos: Position { x: 0, y: 0 }, direction: Direction::Right });
    map.tiles_energized()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            r".|...\....",
            r"|.-.\.....",
            r".....|-...",
            r"........|.",
            r"..........",
            r".........\",
            r"..../.\\..",
            r".-.-/..|..",
            r".|....-|.\",
            r"..//.|....",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(energized_tiles(&input), 46);
    }
}
