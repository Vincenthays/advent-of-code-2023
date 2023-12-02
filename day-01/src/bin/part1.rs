use regex::Regex;
use rayon::prelude::*;

fn main() {
    let re = Regex::new(r"\D").unwrap();

    let sum: u32 =
        include_str!("input.txt")
        .par_lines()
        .map(|l| {
            let clean = re.replace_all(l, "");
            let mut chars = clean.chars();
            let first = chars.next().unwrap();
            let last = chars.next_back().unwrap_or(first);

            let first = first.to_digit(10).unwrap();
            let last = last.to_digit(10).unwrap();

            first * 10 + last
        }).sum();

    println!("{}", sum);
}