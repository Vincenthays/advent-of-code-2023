use std::collections::HashMap;

fn main() {
    let sum: u32 =
        include_str!("input.txt")
            .lines()
            .map(|l| {
                let (game, sets) = l.split_once(": ").unwrap();

                let acc = sets
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
                    .unwrap();

                println!("{:?}", &acc);

                acc
            }).sum();

    println!("{}", sum);
}

fn is_row_valid(color: &str, count: u8) -> bool {
    match color {
        "red" if count <= 12 => true,
        "green" if count <= 13 => true,
        "blue" if count <= 14 => true,
        _ => false
    }
}