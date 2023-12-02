use regex::Regex;
use rayon::prelude::*;


fn to_u32(s: &str) -> u32 {
    match s {
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => 0
    }
}

fn main() {
    let re1 = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re2 = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    let sum =
        include_str!("input.txt")
            .par_lines()
            .map(|l| {
                let first = re1.find(l).map(|v| to_u32(v.as_str())).unwrap();
                let l_rev = l.to_string().chars().rev().collect::<String>();
                let last = re2.find(&l_rev).map(|v| to_u32(v.as_str())).unwrap_or(first);

                first * 10 + last
            })
            .sum::<u32>();

    println!("{}", sum);
}
