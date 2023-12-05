use std::collections::HashMap;

fn main() {
    let mut input = include_str!("input_test.txt")
        .split("\n\n");

    let seeds = input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let maps = input.map(|m| m
            .split_once(":\n")
            .unwrap()
            .1
            .split('\n')
            .map(|l| l
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
            )
            .collect::<Vec<_>>()
        )
        .map(expand_map)
        .collect::<Vec<_>>();

    println!("{seeds:?} {maps:?}");
}

fn expand_map(maps: Vec<Vec<u32>>) -> HashMap<u32, u32> {
    let init = (0..100).map(|i| (i, i)).collect::<HashMap<u32, u32>>();
    maps.into_iter().fold(init, |mut acc, m| {
        let [value, start, len] = m[..] else { panic!("No 3 values") };
        acc.extend((0..len).map(|i| (start+i, value+i)));
        acc
    })
}
