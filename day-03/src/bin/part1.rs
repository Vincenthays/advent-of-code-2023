use regex::Regex;

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let input = include_str!("input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let sum = (0..input.len())
        .map(|i| re
            .find_iter(input[i])
            .filter(|m| {
                let has_diagonal_neighbors = (m.start()..=m.end())
                    .any(|j| check_diagonal_neighbors(&input, i, j));

                return has_diagonal_neighbors
                    || check_symbol(input[i].chars().nth(m.start()-1))
                    || check_symbol(input[i].chars().nth(m.end()+1))
            })
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .sum::<u32>()
        )
        .sum::<u32>();

    println!("{}", sum);
}

fn check_diagonal_neighbors(input: &Vec<&str>, i: usize, j: usize) -> bool {
    return check_line_pos(input, i+1, j) // line up
        || check_line_pos(input, i-1, j) // line down
}

fn check_line_pos(input: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    if let Some(line) = input.get(line_index) {
        return check_symbol(line.chars().nth(char_index-1)) // before
            || check_symbol(line.chars().nth(char_index+1)) // after
    }
    return false
}

fn check_symbol(c: Option<char>) -> bool {
    if let Some(c) = c {
        return Regex::new(r"[^(\d|\\.)]")
            .unwrap()
            .is_match(&c.to_string())
    }
    return false
}
