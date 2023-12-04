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
        .fold(vec![0; count_winning.len()], |mut acc, i| {
            line_score(&count_winning, &mut acc, i);
            acc
        })
        .into_iter()
        .sum::<u64>();

    println!("{res}");
}

fn line_score(count_winning: &Vec<u32>, res: &mut Vec<u64>, i: usize) {
    if let Some(&count) = count_winning.get(i) {
        res[i] += 1;
        for j in i+1..i+1+count as usize {
            line_score(count_winning, res, j)
        }
    }
}
