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

            (winning_count as u8,
             (0..winning_count).fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 }) as u64)
        })
        .collect::<Vec<_>>();

    let scores = (0..winning_count.len() as u32)
        .map(|i| line_score(&winning_count, i))
        .collect::<Vec<_>>();

    println!("{winning_count:?} {scores:?}");
}

fn line_score(winning_count: &Vec<(u8, u64)>, i: u32) -> u64 {
    match winning_count.get(i as usize) {
        Some(&(count, score)) if count > 0 => {
            score + (1..1 + count as u32)
                .map(|j| line_score(&winning_count, i+j))
                .sum::<u64>()
        },
        _ => 0
    }
}
