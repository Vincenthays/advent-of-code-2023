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

    let starts = network
        .keys()
        .copied()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    println!("{instructions:?} {network:?} {starts:?}");

    let count = get_step_count(instructions, network, starts);

    println!("{count}");
}

fn get_step_count<'a>(instructions: Vec<char>, network: HashMap<&str, (&'a str, &'a str)>, mut starts: Vec<&'a str>) -> u32 {
    let mut count = 0;
    for i in instructions.iter().cycle() {
        count += 1;
        starts = starts.into_iter().map(|n| match i {
            'L' => network[n].0,
            'R' => network[n].1,
            _ => panic!("No found {n}"),
        }).collect::<Vec<_>>();

        if starts.iter().all(|n| n.ends_with('Z')) {
            return count
        }
    }
    0
}