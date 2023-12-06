#![feature(iter_next_chunk)]

fn main() {
    let [time, distance] =
        include_str!("input.txt")
            .split('\n')
            .map(|l| l
                .split_once(':')
                .unwrap()
                .1
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
            )
            .next_chunk::<2>()
            .unwrap();

    println!("t={time:?} d={distance:?}");

    let res = get_count_winning(time, distance);

    println!("{res}");
}

fn get_count_winning(t: u64, d: u64) -> u64 {
    (1..t)
        .filter_map(|x| if x * t - x * x > d { Some(x) } else { None })
        .count() as u64
}
