use regex::Regex;

const RE_SYMBOL: Regex = Regex::new(r"[^(\d|\\.)]").unwrap();

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let input = include_str!("input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    for i in 0..input.len() {
        /// line loop
        let m = re
            .find_iter(input[i])
            .filter(|m| {
                let has_diagonal_neighbors =
                    (m.start()..m.end()+1)
                    .any(|j| check_diagonal_neighbors(&input, i, j));

                if has_diagonal_neighbors {
                    return true
                }

                /// before || after
                if check_symbol(input[i].chars().nth(m.start()-1))
                    || check_symbol(input[i].chars().nth(m.end()+1)) {
                    return true
                }

                return false
            })
            .map(|m| {
                println!("{:?}", m);
                m
            })
            .collect::<Vec<_>>();

        println!("{:?}", m);
    }
}

fn check_diagonal_neighbors(input: &Vec<&str>, i: usize, j: usize) -> bool {
    /// line up
    if let Some(line) = input.get(i-1) {
        /// up left || up right
        if check_symbol(line.chars().nth(j-1)) || check_symbol(line.chars().nth(j+1)) {
            return true;
        }
    }

    /// line down
    if let Some(line) = input.get(i+1) {
        /// down left || down right
        if check_symbol(line.chars().nth(j-1)) || check_symbol(line.chars().nth(j+1)) {
            return true;
        }
    }

    return false
}

fn check_symbol(c: Option<char>) -> bool {
    if let Some(c) = c {
        return RE_SYMBOL.is_match(&c.to_string());
    }
    return false;
}
