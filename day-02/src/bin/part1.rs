use rayon::prelude::*;

fn main() {
    let sum =
        include_str!("input.txt")
        .par_lines()
        .map(|l| {
            let (game, sets) = l.split_once(": ").unwrap();

            let is_valid = sets
                .split("; ")
                .flat_map(|set| set.split(", "))
                .all(|color| {
                    let (number, color) = color.trim().split_once(' ').unwrap();
                    is_row_valid(color, number.parse().unwrap())
                });

            if !is_valid {
                return 0;
            }

            game
                .trim()
                .split_once(' ')
                .unwrap()
                .1
                .parse()
                .unwrap()

        })
        .sum::<u32>();

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