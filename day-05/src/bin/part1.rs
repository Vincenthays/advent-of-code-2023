fn main() {
    let mut input = include_str!("input.txt")
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

    let maps = input
        .map(|m| m
            .split_once(":\n")
            .unwrap()
            .1
            .split('\n')
            .map(|l| l
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
            )
            .map(|l| (l[0], l[1], l[2]))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    println!("{seeds:?} {maps:?}");

    let min_location = seeds
        .into_iter()
        .map(|s| get_location(&maps, s, 0))
        .min()
        .unwrap();

    println!("{min_location}");
}
// 7961058 too low

fn get_location(maps: &Vec<Vec<(u32, u32, u32)>>, input: u32, i: usize) -> u32 {
    match maps.get(i) {
        Some(map) => {
            for &(value, start, len) in map {
                for j in 0..len {
                    if j + start == input {
                        return get_location(&maps, j + value, i+1)
                    }
                }
            }
            get_location(&maps, input, i+1)
        },
        _ => input
    }
}
