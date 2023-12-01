use std::str::FromStr;
use std::fmt::Debug;

pub fn read_file(day: u8) -> String {
    let filename = format!("inputs/day{}", day);
    std::fs::read_to_string(filename).unwrap()
}

pub fn read_lines(day: u8) -> Vec<String> {
    read_file(day).split_terminator('\n')
                  .map(String::from)
                  .collect()
}

pub fn read_numbers<T: FromStr>(day: u8) -> Vec<T> where <T as FromStr>::Err: Debug {
    read_lines(day).iter()
                   .map(|n| n.parse::<T>().unwrap())
                   .collect::<Vec<T>>()
}
