static DAY: u8 = 1;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sum_digits(&input, false));
    println!("{DAY}b: {}", sum_digits(&input, true));
}

fn starts_with_number(word: &str, consider_words: bool) -> Option<u32> {
    if let Some(digit) = word.chars().next().unwrap().to_digit(10) {
        return Some(digit);
    }
    if !consider_words {
        return None
    }
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (pos, digit) in digits.iter().enumerate() {
        if word.starts_with(digit) {
            return Some(pos as u32 + 1);
        }
    }
    None
}

fn sum_digits(input: &[String], consider_words: bool) -> u32 {
    let mut sum = 0;
    for line in input {
        let mut line_number = 0;
        for pos in 0 .. line.len() {
            if let Some(number) = starts_with_number(&line[pos..], consider_words) {
                line_number += 10 * number;
                break;
            }
        }
        for pos in (0 .. line.len()).rev() {
            if let Some(number) = starts_with_number(&line[pos..], consider_words) {
                line_number += number;
                break;
            }
        }
        sum += line_number;
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
        assert_eq!(sum_digits(&input, false), 142);

        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(sum_digits(&input, true), 281);
    }
}
