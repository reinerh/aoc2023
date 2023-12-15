static DAY: u8 = 15;

fn main() {
    let input = advent::read_file(DAY);
    println!("{DAY}a: {}", hash_sum(&input));
    println!("{DAY}b: {}", 0);
}

fn hash(input: &str) -> u32 {
    let mut value = 0;

    for c in input.chars() {
        value += c as u32;
        value *= 17;
        value %= 256;
    }

    value
}

fn hash_sum(input: &str) -> u32 {
    let input = input.trim_end();
    input.split(',')
         .map(|x| hash(x))
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash_sum(input), 1320);
    }
}
