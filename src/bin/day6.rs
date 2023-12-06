static DAY: u8 = 6;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", possible_ways(&input));
    println!("{DAY}b: {}", possible_ways_single_race(&input));
}

fn count_ways(time: u64, record: u64) -> u64 {
    let mut ways = 0;
    for hold_button in 1 .. time {
        let moving_time = time - hold_button;
        let boat_distance = moving_time * hold_button;
        if boat_distance > record {
            ways += 1;
        }
    }
    ways
}

fn possible_ways(input: &[String]) -> u64 {
    let get_numbers = |x: &str| {
        x.split_once(':').unwrap().1.to_string()
         .split(' ')
         .filter(|s| !s.is_empty())
         .map(|s| s.parse().unwrap())
         .collect::<Vec<u64>>()
    };
    let times = get_numbers(&input[0]);
    let distances = get_numbers(&input[1]);

    times.iter()
         .zip(distances.iter())
         .map(|(time, distance)| count_ways(*time, *distance))
         .product()
}

fn possible_ways_single_race(input: &[String]) -> u64 {
    let get_number = |mut x: String| -> u64 {
        x.retain(|c| c.is_ascii_digit());
        x.parse().unwrap()
    };
    let time = get_number(input[0].clone());
    let distance = get_number(input[1].clone());

    count_ways(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "Time:      7  15   30",
            "Distance:  9  40  200",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(possible_ways(&input), 288);
        assert_eq!(possible_ways_single_race(&input), 71503);
    }
}
