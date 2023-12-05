use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();

    let count_winning =
        include_str!("input.txt")
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(": ").unwrap();
            let (winning, have) = l.split_once(" | ").unwrap();

            let winning = re.split(winning.trim()).map(|n| n.parse().unwrap()).collect::<Vec<u32>>();
            let have = re.split(have.trim()).map(|n| n.parse().unwrap()).collect::<Vec<u32>>();

            have
                .into_iter()
                .filter_map(|n| if winning.contains(&n) { Some(n) } else { None })
                .count() as u32
        })
        .collect::<Vec<_>>();

    let res = (0..count_winning.len())
        .map(|i| line_score(&count_winning, i))
        .sum::<u64>();

    println!("{res}");
}

fn line_score(count_winning: &Vec<u32>, i: usize) -> u64 {
    if let Some(&count) = count_winning.get(i) {
        1 + (i+1..i+1+count as usize)
            .map(|j| line_score(count_winning, j))
            .sum::<u64>()
    } else {
        0
    }
}
