// binary - Vec<i32>
// binary represents characters, e.g. ABC a=0, b=10, c=11
// binary ends with 0 (could be a or b)
// return true if binary ends with single 0 false if ends with 10

fn main() {
    println!("Hello, world!");
}

fn execute(input: Vec<i32>) -> bool {
    let mut target = false;
    let mut inside = false;

    for i in input {
        if i == 0 {
            if !inside {
                target = true;
                continue;
            }

            target = false;
            inside = false;
            continue;
        }

        if i == 1 {
            if !inside {
                inside = true;
                continue;
            }

            target = false;
            inside = false;
        }
    }

    target
}

#[cfg(test)]
mod tests {
    use crate::execute;

    #[test]
    fn ends_with_0() {
        assert!(execute(vec![0]))
    }

    #[test]
    fn ends_with_0_1() {
        assert!(execute(vec![0, 1, 0, 1, 1, 0]))
    }

    #[test]
    fn not_end_with_10() {
        assert!(!execute(vec![1, 0]));
    }

    #[test]
    fn test() {
        assert!(execute(vec![1, 1, 0]));
    }
}
