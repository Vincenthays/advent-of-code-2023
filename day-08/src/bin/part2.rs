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

    let res = steps_count
        .iter()
        .fold(None, |acc, &x| match acc {
            None => Some(x),
            Some(acc) => Some(lcm(acc, x))
        })
        .unwrap();

    println!("{instructions:?} {network:?} {steps_count:?} {res}");
}

fn get_step_count<'a>(instructions: &Vec<char>, network: &HashMap<&str, (&'a str, &'a str)>, mut node: &'a str) -> Option<u64> {
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

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        (min, max) = (max, min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}
