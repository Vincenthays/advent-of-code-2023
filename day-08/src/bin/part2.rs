use regex::Regex;
use std::collections::HashMap;

fn main() {
    let re = Regex::new(r"[A-Z\d]{3}").unwrap();
    let mut input = include_str!("input.txt").lines().filter(|&l| l != "");

    let instructions = input.next().unwrap().chars().collect::<Vec<_>>();
    let network = input
        .map(|l| {
            let mut nodes = re.find_iter(l).map(|m| m.as_str());
            (nodes.next().unwrap(), (nodes.next().unwrap(), nodes.next().unwrap()))
        })
        .collect::<HashMap<_, _>>();

    let steps_count = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|n| get_step_count(&instructions, &network, n).unwrap())
        .collect::<Vec<_>>();

    println!("{instructions:?} {network:?} {steps_count:?}");
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
        if node.ends_with('Z') {
            return Some(count);
        }
    }
    None
}

fn lcm(mut a: u32, b: u32) -> u32 {
    while a % b != 0 {
        a += b
    }
    a
}
