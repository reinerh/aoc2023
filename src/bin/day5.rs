use std::thread;
use std::sync::Arc;

static DAY: u8 = 5;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", lowest_location(&input));
    println!("{DAY}b: {}", lowest_location2(&input));
}

#[derive(Clone, Copy)]
struct RangeMap {
    dst_start: u64,
    src_start: u64,
    range: u64,
}

impl RangeMap {
    fn new(input: &str) -> RangeMap {
        let values = input.split(' ')
                          .map(|x| x.parse().unwrap())
                          .collect::<Vec<_>>();

        RangeMap { dst_start: values[0], src_start: values[1], range: values[2] }
    }
}

struct RangeCategory {
    ranges: Vec<RangeMap>,
}

impl RangeCategory {
    fn map(&self, val: u64) -> u64 {
        for range in &self.ranges {
            if range.src_start <= val && val < range.src_start + range.range {
                return range.dst_start + val - range.src_start
            }
        }
        val
    }
}

fn read_range_map(input: &[String]) -> Vec<RangeCategory> {
    let mut range_maps = Vec::new();
    let mut current_category = Vec::new();
    for line in input.iter() {
        if line.is_empty() {
            range_maps.push(RangeCategory { ranges: current_category.clone() });
            current_category.clear();
            continue;
        }
        if !line.chars().next().unwrap().is_ascii_digit() {
            /* new category */
            continue;
        }
        current_category.push(RangeMap::new(line));
    }
    range_maps.push(RangeCategory { ranges: current_category.clone() });
    range_maps
}

fn lowest_location(input: &[String]) -> u64 {
    let (_, seeds_str) = input[0].split_once(": ").unwrap();
    let seeds = seeds_str.split(' ')
                         .map(|x| x.parse().unwrap())
                         .collect::<Vec<u64>>();

    let range_maps = read_range_map(&input[2..]);

    let mut locations = Vec::new();
    for seed in seeds {
        let mut next_val = seed;
        for category in &range_maps {
            next_val = category.map(next_val);
        }
        locations.push(next_val);
    }

    *locations.iter().min().unwrap()
}

fn lowest_location2(input: &[String]) -> u64 {
    let (_, seeds_str) = input[0].split_once(": ").unwrap();
    let seeds = seeds_str.split(' ')
                         .map(|x| x.parse().unwrap())
                         .collect::<Vec<u64>>();

    let range_maps = Arc::new(read_range_map(&input[2..]));

    const N_THREADS : u64 = 24;

    let mut lowest_location = u64::max_value();
    for (i, start) in seeds.iter().enumerate().step_by(2) {
        let range = seeds[i+1];

        let mut threads = Vec::new();
        let range_per_thread = range / N_THREADS;
        for t in 0 .. N_THREADS {
            let range_maps = Arc::clone(&range_maps);
            let thread_start = start + (t * range_per_thread);
            let mut thread_end = thread_start + range_per_thread - 1;
            if t == N_THREADS - 1 {
                thread_end += range % N_THREADS;
            }

            threads.push(thread::spawn(move || {
                for seed in thread_start ..= thread_end {
                    let mut next_val = seed;
                    for category in range_maps.iter() {
                        next_val = category.map(next_val);
                    }
                    lowest_location = std::cmp::min(next_val, lowest_location)
                }
                lowest_location
            }));
        }

        for thread in threads {
            lowest_location = std::cmp::min(thread.join().unwrap(), lowest_location);
        }
    }

    lowest_location
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4"
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(lowest_location(&input), 35);
        assert_eq!(lowest_location2(&input), 46);
    }
}
