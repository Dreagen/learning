use std::collections::HashMap;

fn main() {
    let input = "(){}}{".to_string();
    println!(
        "checking:{input}: has valid parenthesis {}",
        is_valid(input.clone())
    );
}

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let mut parenthesis_map = HashMap::new();
    parenthesis_map.insert(')', '(');
    parenthesis_map.insert(']', '[');
    parenthesis_map.insert('}', '{');

    for c in s.chars() {
        if stack.len() == 0 && parenthesis_map.keys().any(|closing| &c == closing) {
            return false;
        }

        if parenthesis_map.values().any(|opening| &c == opening) {
            stack.push(c);
            continue;
        }

        if parenthesis_map.keys().any(|closing| &c == closing) {
            let popped = stack.pop_if(|item| *item == parenthesis_map[&c]);

            if popped == None {
                return false;
            }
        }
    }

    if stack.is_empty() {
        return true;
    }
    false
}

#[test]
fn single_bracket_set_returns_true() {
    assert_eq!(true, is_valid("()".to_string()));
}
