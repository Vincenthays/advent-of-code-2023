use regex::Regex;
use std::collections::HashMap;

fn main() {
    let re = Regex::new(r#"[A-Z]{3}"#).unwrap();
    let mut input = include_str!("input_test.txt").lines();

    let instructions = input.next().unwrap().chars().collect::<Vec<_>>();
    let network = input
        .filter(|&l| l != "")
        .map(|l| {
            let mut nodes = re.find_iter(l).map(|m| m.as_str());
            (nodes.next().unwrap(), (nodes.next().unwrap(), nodes.next().unwrap()))
        })
        .collect::<HashMap<_, _>>();

    println!("{instructions:?} {network:?}");
}

fn get_step_nb(instructions: Vec<char>, network: HashMap<&str, (&str, &str)>) -> u32 {
    todo!()
}