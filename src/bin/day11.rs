static DAY: u8 = 11;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sum_path_lengths(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

struct GalaxyMap {
    galaxies: Vec<Position>,
    width: isize,
    height: isize,
}

impl GalaxyMap {
    fn new(input: &[String]) -> GalaxyMap {
        let mut galaxies = Vec::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Position { x: x as isize, y: y as isize };
                if c == '#' {
                    galaxies.push(pos);
                }
            }
        }
        let width = input[0].len() as isize;
        let height = input.len() as isize;
        GalaxyMap { galaxies, width, height }
    }

    fn expand_space(&mut self) {
        let mut columns = Vec::new();
        for x in 0 .. self.width {
            if !self.galaxies.iter().any(|pos| pos.x == x) {
                columns.push(x);
            }
        }
        let mut rows = Vec::new();
        for y in 0 .. self.height {
            if !self.galaxies.iter().any(|pos| pos.y == y) {
                rows.push(y);
            }
        }
        for x in columns.iter().rev() {
            for galaxy in self.galaxies.iter_mut().filter(|pos| pos.x > *x) {
                galaxy.x += 1;
            }
            self.width += 1;
        }
        for y in rows.iter().rev() {
            for galaxy in self.galaxies.iter_mut().filter(|pos| pos.y > *y) {
                galaxy.y += 1;
            }
            self.height += 1;
        }
    }

    fn _print_map(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                if self.galaxies.iter().any(|pos| *pos == Position { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn calculate_distances(&self) -> Vec<isize> {
        let mut distances = Vec::new();
        for (i, pos1) in self.galaxies.iter().enumerate() {
            for pos2 in self.galaxies.iter().skip(i+1) {
                let distance = pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y);
                distances.push(distance as isize);
            }
        }
        distances
    }
}

fn sum_path_lengths(input: &[String]) -> isize {
    let mut galaxymap = GalaxyMap::new(input);
    galaxymap.expand_space();
    galaxymap.calculate_distances()
             .iter()
             .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(sum_path_lengths(&input), 374);
    }
}
