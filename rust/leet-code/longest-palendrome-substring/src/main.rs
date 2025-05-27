use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut largest_palendrome: (usize, usize) = (0, 0);
        let mut map: HashMap<u8, Vec<usize>> = HashMap::new();

        for (i, c) in s.bytes().enumerate() {
            map.entry(c).or_insert_with(Vec::new).push(i);
        }

        for i in 0..s.len() {
            if s.len() - i <= largest_palendrome.1 - largest_palendrome.0 {
                break;
            }

            for j in i..s.len() {
                if s[i..j + 1].bytes().eq(s[i..j + 1].bytes().rev()) {
                    if largest_palendrome.1 - largest_palendrome.0 < (j + 1 - i) {
                        largest_palendrome = (i, j + 1);
                    }
                }
            }
        }

        s[largest_palendrome.0..largest_palendrome.1].to_string()
    }
}
