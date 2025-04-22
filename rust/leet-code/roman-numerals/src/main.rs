fn main() {}

#[warn(dead_code)]
fn calculate_roman_numerals(input: String) -> u32 {
    let mut total = 0;

    let mut found_i = false;
    let mut found_x = false;
    let mut found_c = false;

    for char in input.chars().into_iter() {
        total += match char {
            'I' => {
                found_i = !found_i;
                1
            }
            'V' => {
                if found_i {
                    found_i = false;
                    3
                } else {
                    5
                }
            }
            'X' => {
                found_x = !found_x;
                if found_i {
                    found_i = false;
                    8
                } else {
                    10
                }
            }
            'L' => {
                if found_x {
                    found_x = false;
                    30
                } else {
                    50
                }
            }
            'C' => {
                found_c = !found_c;
                if found_x {
                    found_x = false;
                    80
                } else {
                    100
                }
            }
            'D' => {
                if found_c {
                    found_c = false;
                    300
                } else {
                    500
                }
            }
            'M' => {
                if found_c {
                    found_c = false;
                    800
                } else {
                    1000
                }
            }
            _ => panic!("not recognised roman numeral"),
        };
        println!("Running total {}", total);
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::calculate_roman_numerals;

    #[test]
    fn three() {
        assert_eq!(calculate_roman_numerals("III".to_string()), 3);
    }

    #[test]
    fn fifty_eight() {
        assert_eq!(calculate_roman_numerals("LVIII".to_string()), 58);
    }

    #[test]
    fn one_thousand_nine_hundred_and_ninty_four() {
        assert_eq!(calculate_roman_numerals("MCMXCIV".to_string()), 1994);
    }
}
