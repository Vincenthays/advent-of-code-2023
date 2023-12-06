#![feature(iter_next_chunk)]
use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();
    let [time, distance] =
        include_str!("input_test.txt")
        .split('\n')
        .map(|l|
            re.split(l)
            .filter_map(|v| v.parse().ok())
            .collect::<Vec<u32>>()
        )
        .next_chunk::<2>()
        .unwrap();

    println!("t={time:?} d={distance:?}");

    let res = time
        .into_iter()
        .zip(distance)
        .map(|(t, d)| get_count_winning(t, d))
        .fold(1, |acc, x| acc * x);

    println!("{res}");
}

fn get_count_winning(t: u32, d: u32) -> u32 {
    (1..t)
        .filter_map(|x| match x * t - x * x {
            res if res > d => Some(x),
            _ => None
        })
        .count() as u32
}
