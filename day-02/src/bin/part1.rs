fn main() {
    let sum: u32 =
        include_str!("input.txt")
        .lines()
        .map(|l| {
            let (game, sets) = l.split_once(": ").unwrap();

            let is_valid = sets
                .split("; ")
                .flat_map(|set| set.split(", "))
                .all(|color| {
                    let (number, color) = color.trim().split_once(' ').unwrap();
                    is_row_valid(color, number.parse().unwrap())
                });

            if is_valid {
                let (_, number) = game.trim().split_once(' ').unwrap();
                return number.parse().unwrap();
            }

            0
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