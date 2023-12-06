use rayon::prelude::*;

fn main() {
    let input = include_str!("input_test.txt")
        .split_once('\n')
        .unwrap();

    println!("{time} {distance}");
}
