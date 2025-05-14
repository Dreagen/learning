fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut largest_palendrome: (usize, usize) = (0, 0);

        for i in 0..s.len() {
            if s.len() - i <= largest_palendrome.1 - largest_palendrome.0 {
                break;
            }

            for j in i..s.len() {
                if s.bytes()
                    .skip(i)
                    .take(j + 1 - i)
                    .eq(s.bytes().skip(i).take(j + 1 - i).rev())
                {
                    if largest_palendrome.1 - largest_palendrome.0 < (j + 1 - i) {
                        largest_palendrome = (i, j + 1);
                    }
                }
            }
        }

        s[largest_palendrome.0..largest_palendrome.1].to_string()
    }
}
