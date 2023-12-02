use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let sum =
        include_str!("input.txt")
        .par_lines()
        .map(|l| l
            .split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .flat_map(|set| set.split(", "))
            .fold(HashMap::new(), |mut acc, color| {
                let (number, color) = color.trim().split_once(' ').unwrap();
                let number = number.parse::<u32>().unwrap();

                acc
                    .entry(color)
                    .and_modify(|v| {
                        if *v < number {
                            *v = number
                        }
                    })
                    .or_insert(number);

                acc
            })
            .into_values()
            .reduce(|acc, v| acc * v)
            .unwrap()
        )
        .sum::<u32>();

    println!("{}", sum);
}
