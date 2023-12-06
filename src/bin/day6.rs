static DAY: u8 = 6;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", possible_ways(&input));
    println!("{DAY}b: {}", 0);
}

fn possible_ways(input: &[String]) -> u32 {
    let get_numbers = |x: &str| {
        x.split_once(':').unwrap().1.to_string()
         .split(' ')
         .filter(|s| !s.is_empty())
         .map(|s| s.parse().unwrap())
         .collect::<Vec<u32>>()
    };
    let times = get_numbers(&input[0]);
    let distances = get_numbers(&input[1]);

    let mut ways_list = Vec::new();
    for (time, record) in times.iter().zip(distances.iter()) {
        let mut ways = 0;
        for hold_button in 1 .. *time {
            let moving_time = time - hold_button;
            let boat_distance = moving_time * hold_button;
            if boat_distance > *record {
                ways += 1;
            }
        }
        ways_list.push(ways);
    }
    ways_list.iter()
             .product()
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
    }
}
