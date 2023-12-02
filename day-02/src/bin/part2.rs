use std::collections::HashMap;

fn main() {
    let sum: u32 =
        include_str!("input.txt")
            .lines()
            .map(|l| l
                .split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .flat_map(|set| set.split(", "))
                .fold(HashMap::new(), |mut acc, color| {
                    let (number, color) = color.trim().split_once(' ').unwrap();
                    let number: u32 = number.parse().unwrap();

                    if acc.entry(color).or_insert(number).to_owned() < number {
                        acc.insert(color, number);
                    }

                    acc
                })
                .into_values()
                .reduce(|acc, v| acc * v)
                .unwrap()
            )
            .sum();

    println!("{}", sum);
}
