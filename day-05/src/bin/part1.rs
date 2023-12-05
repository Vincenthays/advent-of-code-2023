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
    ).collect::<Vec<_>>();

    println!("{seeds:?} {maps:?}");
}
