use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn is_valid(s: String) -> bool {
    let mut parenthesis_map = HashMap::new();

    let open_round = '(';
    let close_round = ')';

    let open_curly = '{';
    let close_curly = '}';

    let open_square = '[';
    let close_square = ']';

    let count_round = 0;
    let count = 0;

    let parenthesis_list = [];

    for c in s.chars() {}
    false
}

#[test]
fn single_bracket_set_returns_true() {
    assert_eq!(true, is_valid("()".to_string()));
}
