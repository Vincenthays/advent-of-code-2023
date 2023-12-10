use std::cmp::Ordering;
use std::collections::HashMap;

const LETTER_ORDER: [char; 14] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn main() {
    let mut input = include_str!("input_test.txt")
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            let mut values = cards
                .chars()
                .filter_map(|c| if c == 'J' { None } else { Some(c) })
                .fold(HashMap::new(), |mut acc, x| {
                    *acc.entry(x).or_insert(0) += 1;
                    acc
                })
                .into_values()
                .collect::<Vec<u32>>();

            values.sort();
            let nb_joker = cards.chars().filter(|c| *c == 'J').count() as u32;
            match nb_joker {
                0 => {},
                5 => { values = vec![5] }
                x => { *values.last_mut().unwrap() += x }
            }

            (get_score(values), cards, bid.trim().parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    input.sort_by(|(score1, cards1, _), (score2, cards2, _)|
        match get_ordering(score1, score2) {
            Ordering::Equal => get_ordering_cards(cards1, cards2),
            order => order,
        });

    println!("{input:?}");

    let res = input
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i as u32 +1))
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
        _ => 0
    }
}

fn get_ordering(score1: &u32, score2: &u32) -> Ordering {
    if score1 < score2 { Ordering::Greater }
    else if score1 > score2 { Ordering::Less }
    else { Ordering::Equal }
}

fn get_ordering_cards(cards1: &str, cards2: &str) -> Ordering {
    for (c1, c2) in cards1.chars().zip(cards2.chars()) {
        if c1 == c2 { continue }

        let c1 = LETTER_ORDER.iter().position(|&c| c == c1).unwrap();
        let c2 = LETTER_ORDER.iter().position(|&c| c == c2).unwrap();

        return if c1 < c2 { Ordering::Greater } else { Ordering::Less }
    }
    Ordering::Equal
}

// 248,618,050 to low
// 248,751,336 to high
