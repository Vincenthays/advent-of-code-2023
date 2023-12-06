#![feature(iter_next_chunk)]

use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();
    let [time, distance] = include_str!("input_test.txt")
        .split('\n')
        .map(|l| l.split_once(':').unwrap().1.trim())
        .map(|l|
            re.split(l)
            .map(|v| v.parse().unwrap())
            .collect::<Vec<u32>>()
        )
        .next_chunk::<2>()
        .unwrap();

    println!("t={time:?} d={distance:?}");
}
