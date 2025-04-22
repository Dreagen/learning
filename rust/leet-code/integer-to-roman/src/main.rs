fn main() {
    println!("Hello, world!");
}

pub fn interger_to_roman(num: i32) -> String {
    let digit1 = num / 1000 % 10;
    let digit2 = num / 100 % 10;
    let digit3 = num / 10 % 10;
    let digit4 = num % 10;

    let digit1 = match digit1 {
        0 => "",
        1 => "M",
        2 => "MM",
        3 => "MMM",
        _ => panic!(),
    };

    let digit2 = match digit2 {
        0 => "",
        1 => "C",
        2 => "CC",
        3 => "CCC",
        4 => "CD",
        5 => "D",
        6 => "DC",
        7 => "DCC",
        8 => "DCCC",
        9 => "CM",
        _ => panic!(),
    };

    let digit3 = match digit3 {
        0 => "",
        1 => "X",
        2 => "XX",
        3 => "XXX",
        4 => "XL",
        5 => "L",
        6 => "LX",
        7 => "LXX",
        8 => "LXXX",
        9 => "XC",
        _ => panic!(),
    };

    let digit4 = match digit4 {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        _ => panic!(),
    };

    format!("{}{}{}{}", digit1, digit2, digit3, digit4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(interger_to_roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn test2() {
        assert_eq!(interger_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn test3() {
        assert_eq!(interger_to_roman(58), "LVIII");
    }
}
