use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();

    let sum =
        include_str!("input.txt")
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(": ").unwrap();
            let (winning, have) = l.split_once(" | ").unwrap();

            let winning = re.split(winning.trim()).map(|n| n.parse().unwrap()).collect::<Vec<u32>>();
            let have = re.split(have.trim()).map(|n| n.parse().unwrap()).collect::<Vec<u32>>();

            have
                .into_iter()
                .filter(|n| winning.contains(&n))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum::<u32>();

    println!("{:?}", sum);
}
