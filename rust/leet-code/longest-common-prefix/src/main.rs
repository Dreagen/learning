fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut counter = 0usize;
        let first_word = strs.get(0).unwrap();

        loop {
            let current_letter = first_word.chars().nth(counter);

            let result = current_letter.and_then(|_| {
                for s in strs.iter().skip(1) {
                    if s.chars().nth(counter) != current_letter {
                        return None;
                    }
                }
                Some(true)
            });

            match result {
                Some(_) => counter += 1,
                None => break,
            }
        }

        strs[0][..counter].to_string()
    }
}
