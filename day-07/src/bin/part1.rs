use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut input = include_str!("input_test.txt")
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            let mut values = cards
                .chars()
                .fold(HashMap::new(), |mut acc, x| {
                    *acc.entry(x).or_insert(0) += 1;
                    acc
                })
                .into_values()
                .collect::<Vec<u32>>();

            values.sort();

            (values, cards, bid.trim().parse::<u32>().unwrap())
        })
        .map(|(values, cards, bid)| {
            let score = if values.contains(&5) { 5 }
                else if values.contains(&4) { 4 }
                else if values.contains(&3) { 3 }
                else if values[..] == [1, 2, 2] { 2 }
                else if values.contains(&2) { 1 }
                else { 0 };

            (score, cards, bid)
        })
        .collect::<Vec<_>>();

    input.sort_by(|(score1, cards1, _), (score2, cards2, _)|
        match score1.cmp(score2) {
            Ordering::Equal => cards2.cmp(cards1),
            o => o
        });

    let res = input
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i as u32 +1))
        .sum::<u32>();

    println!("{input:?}\n{res}");
}