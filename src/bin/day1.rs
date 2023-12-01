static DAY: u8 = 1;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sum_digits(&input));
    println!("{DAY}b: {}", 0);
}

fn sum_digits(input: &[String]) -> u32 {
    let mut sum = 0;
    for line in input {
        let left_pos = line.find(|c: char| c.is_digit(10)).expect("line contains digit");
        let right_pos = line.rfind(|c: char| c.is_digit(10)).expect("line contains digit");

        let line = line.as_bytes();
        let left_digit = (line[left_pos] as char).to_digit(10).unwrap();
        let right_digit = (line[right_pos] as char).to_digit(10).unwrap();
        sum += (left_digit * 10 + right_digit) as u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(sum_digits(&input), 142);
    }
}
