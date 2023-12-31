use regex::Regex;
use std::collections::HashMap;

fn main() {
    let re = Regex::new(r#"[A-Z]{3}"#).unwrap();
    let mut input = include_str!("input.txt").lines();

    let instructions = input.next().unwrap().chars().collect::<Vec<_>>();
    let network = input
        .filter(|&l| l != "")
        .map(|l| {
            let mut nodes = re.find_iter(l).map(|m| m.as_str());
            (nodes.next().unwrap(), (nodes.next().unwrap(), nodes.next().unwrap()))
        })
        .collect::<HashMap<_, _>>();

    println!("{instructions:?} {network:?}");

    let count = get_step_count(&instructions, &network, "AAA");

    println!("{count}");
}

fn get_step_count<'a>(instructions: &Vec<char>, network: &HashMap<&str, (&'a str, &'a str)>, mut node: &'a str) -> Option<u32> {
    let mut count = 0;
    for i in instructions.iter().cycle() {
        count += 1;
        node = match i {
            'L' => network[node].0,
            'R' => network[node].1,
           _ => panic!("No found {node}"),
        };
        if node == "ZZZ" {
            return Some(count);
        }
    }
    None
}
