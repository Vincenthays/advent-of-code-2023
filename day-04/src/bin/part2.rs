use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();

    let winning_count =
        include_str!("input_test.txt")
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(": ").unwrap();
            let (winning, have) = l.split_once(" | ").unwrap();

            let winning = re.split(winning.trim()).map(|n| n.parse().unwrap()).collect::<Vec<u32>>();
            let have = re.split(have.trim()).map(|n| n.parse().unwrap()).collect::<Vec<u32>>();

            let winning_count = have
                .into_iter()
                .filter_map(|n| if winning.contains(&n) { Some(n) } else { None })
                .count();

            (winning_count, (0..winning_count).fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 }))
        })
        .collect::<Vec<_>>();

    println!("{winning_count:?}");
}
