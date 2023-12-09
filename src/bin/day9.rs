static DAY: u8 = 9;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sum_extrapolations(&input, false));
    println!("{DAY}b: {}", sum_extrapolations(&input, true));
}

fn extrapolate(input: &[i32], previous: bool) -> i32 {
    if input.iter().all(|&n| n == 0) {
        /* all zeroes */
        return 0;
    }

    let diffs = input.iter()
                     .as_slice()
                     .windows(2)
                     .map(|x| x[1] - x[0])
                     .collect::<Vec<_>>();
    if previous {
        input.first().unwrap() - extrapolate(&diffs, previous)
    } else {
        input.last().unwrap() + extrapolate(&diffs, previous)
    }
}

fn sum_extrapolations(input: &[String], previous: bool) -> i32 {
    input.iter()
         .map(|x| x.split(' ').map(|n| n.parse().unwrap()).collect::<Vec<_>>())
         .map(|numbers| extrapolate(&numbers, previous))
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "0 3 6 9 12 15",
            "1 3 6 10 15 21",
            "10 13 16 21 30 45",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(sum_extrapolations(&input, false), 114);
        assert_eq!(sum_extrapolations(&input, true), 2);
    }
}
