use std::cmp::Ordering;
use std::collections::HashMap;

const LETTER_ORDER: [char; 14] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn main() {
    let mut input = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            let mut values = cards
                .chars()
                .filter(|&c| c != 'J')
                .fold(HashMap::new(), |mut acc, x| {
                    *acc.entry(x).or_insert(0) += 1;
                    acc
                })
                .into_values()
                .collect::<Vec<u32>>();

            values.sort();
            match  cards.chars().filter(|&c| c == 'J').count() {
                0 => {},
                5 => { values = vec![5] }
                x => { *values.last_mut().unwrap() += x as u32 }
            }

            (get_score(values), cards, bid.trim().parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    input.sort_by(|(score1, cards1, _), (score2, cards2, _)|
        match score1.cmp(score2) {
            Ordering::Equal => get_ordering_cards(cards1, cards2),
            order => order,
        });

    println!("{input:?}");

    let res = input
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i as u32 + 1))
        .sum::<u32>();

    println!("{res}");
}

fn get_score(mut values: Vec<u32>) -> u32 {
    while values.iter().sum::<u32>() < 5 {
        values.push(1);
    }
    values.sort();
    match values[..] {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        [1, 1, 1, 1, 1] => 0,
        _ => panic!("not found {values:?}")
    }
}

fn get_ordering_cards(cards1: &str, cards2: &str) -> Ordering {
    for (c1, c2) in cards1.chars().zip(cards2.chars()) {
        if c1 == c2 { continue }

        let c1 = LETTER_ORDER.iter().position(|&c| c == c1).unwrap();
        let c2 = LETTER_ORDER.iter().position(|&c| c == c2).unwrap();

        return c1.cmp(&c2).reverse()
    }
    Ordering::Equal
}

// 248,618,050 to low
// 248,618,050
// 248,751,336 to high
