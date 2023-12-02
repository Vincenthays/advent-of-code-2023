use regex::Regex;
use rayon::prelude::*;

const DATA: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;


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
    let regex_str = "one|two|three|four|five|six|seven|eight|nine";
    let re1 = Regex::new(&format!(r"(\d|{regex_str})")).unwrap();
    let re2 = Regex::new(&format!(r"(\d|{})", regex_str.chars().rev().collect::<String>())).unwrap();

    let sum: u32 =
        include_str!("input1.txt")
        // DATA
        .par_lines()
        .map(|l| {
            let first = re1.find(l).map(|v| to_u32(v.as_str())).unwrap();
            let l_rev = l.to_string().chars().rev().collect::<String>();
            let last = re2.find(&l_rev).map(|v| to_u32(v.as_str())).unwrap_or(first);

            first * 10 + last
        }).sum();

    println!("{}", sum);
}
