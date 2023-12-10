use std::collections::HashMap;
use regex::Regex;

fn main() {
    let re = Regex::new(r#"[A-Z]{3}"#).unwrap();
    let mut input = include_str!("input_test.txt").lines();

    let instructions = input.next().unwrap().chars().collect::<Vec<_>>();
    let network = input
        .filter(|&l| l != "")
        .map(|l| {
            let mut nodes = re.find_iter(l).map(|m| m.as_str());
            (nodes.next().unwrap(), nodes.collect::<Vec<_>>())
        })
        .collect::<HashMap<_, Vec<_>>>();

    println!("{instructions:?} {network:?}");
}