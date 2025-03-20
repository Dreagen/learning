use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn is_anagram(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut char_map = HashMap::new();

    let chars = word1.bytes();
    for c in chars {
        *char_map.entry(c).or_insert(0) += 1;
    }

    for c in word2.bytes() {
        if let Some(count) = char_map.get_mut(&c) {
            if *count == 1 {
                char_map.remove(&c);
            } else {
                char_map.entry(c).and_modify(|x| *x -= 1);
            }
        } else {
            return false;
        }
    }

    true
}

#[test]
fn test_1() {
    let word1 = String::from("anagram");
    let word2 = String::from("nagaram");

    assert!(is_anagram(word1, word2));
}

#[test]
fn test_2() {
    let word1 = String::from("rat");
    let word2 = String::from("car");

    assert!(!is_anagram(word1, word2));
}

#[test]
fn test_3() {
    let word1 = String::from("ab");
    let word2 = String::from("a");

    assert!(!is_anagram(word1, word2));
}
