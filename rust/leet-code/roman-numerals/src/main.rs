fn main() {}

pub fn calculate_roman_numerals(s: String) -> u32 {
    let mut total = 0;
    let mut marker: u8 = 0;

    for char in s.bytes() {
        total += match char {
            b'I' => {
                if marker == 1 {
                    marker = 0;
                } else {
                    marker = 1;
                }
                1
            }
            b'V' => {
                if marker == 1 {
                    marker = 0;
                    3
                } else {
                    5
                }
            }
            b'X' => {
                let result = if marker == 1 { 8 } else { 10 };
                marker = 2;

                result
            }
            b'L' => {
                let result = if marker == 2 { 30 } else { 50 };
                marker = 0;

                result
            }
            b'C' => {
                let result = if marker == 2 { 80 } else { 100 };
                marker = 3;

                result
            }
            b'D' => {
                let result = if marker == 3 { 300 } else { 500 };
                marker = 0;

                result
            }
            b'M' => {
                let result = if marker == 3 { 800 } else { 1000 };
                marker = 0;

                result
            }
            _ => panic!("not recognised roman numeral"),
        };
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
