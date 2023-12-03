use regex::Regex;

fn main() {
    let re = Regex::new(r"\*").unwrap();
    let input = include_str!("input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let sum =
        (0..input.len())
        .map(|i| re
            .find_iter(input[i])
            .map(|m| find_neighbors(&input, i, m.end()))
            .filter(|n| n.len() == 2)
            .map(|n| n[0] * n[1])
            .sum::<u32>()
        ).sum::<u32>();

    println!("{:?}", sum);
}

fn find_neighbors(input: &Vec<&str>, i: usize, j: usize) -> Vec<u32> {
    return [
        check_line_pos(input, i+1, j),
        check_line_pos(input, i, j),
        check_line_pos(input, i-1, j),
    ].concat()
}

fn check_line_pos(input: &Vec<&str>, line_index: usize, char_index: usize) -> Vec<u32> {
    if let Some(line) = input.get(line_index) {
        return Regex::new(r"\d+")
            .unwrap()
            .find_iter(line)
            .filter(|m| {
                for i in m.start()-1..m.end()+2 {
                    if i == char_index {
                        return true
                    }
                }
                return false
            })
            .map(|m| m.as_str().parse().unwrap())
            .collect()
    }
    return vec![];
}
