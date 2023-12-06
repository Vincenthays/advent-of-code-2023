#![feature(iter_next_chunk)]

use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();
    let [time, distance] = include_str!("input_test.txt")
        .split('\n')
        .map(|l|
            re.split(l)
            .filter_map(|v| v.parse().ok())
            .collect::<Vec<u32>>()
        )
        .next_chunk::<2>()
        .unwrap();

    println!("t={time:?} d={distance:?}");
}
