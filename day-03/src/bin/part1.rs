use regex::Regex;

const RE_SYMBOL: Regex = Regex::new(r"[^(\d|\\.)]").expect("");

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
    return check_line_pos(input, i+1, j) /// line up
        || check_line_pos(input, i, j)   /// same line
        || check_line_pos(input, i-1, j) /// line down
}

fn check_line_pos(input: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    if let Some(line) = input.get(line_index) {
        return check_symbol(line.chars().nth(char_index-1)) /// before
            || check_symbol(line.chars().nth(char_index+1)) /// after
    }
    return false
}

fn check_symbol(c: Option<char>) -> bool {
    if let Some(c) = c {
        return RE_SYMBOL.is_match(&c.to_string());
    }
    return false;
}
