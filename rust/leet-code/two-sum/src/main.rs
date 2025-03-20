use std::{collections::HashMap, usize};

fn main() {
    println!("Hello, world!");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let value = nums.get(i).unwrap();
        let difference = target - value;
        if let Some(index) = num_map.get(&difference) {
            return vec![*index as i32, i as i32];
        }

        num_map.insert(*value, i);
    }

    panic!("didn't find a match");
}

#[test]
fn test_1() {
    let input = vec![2, 7, 11, 15];
    let target = 9;

    assert_eq!(vec![0, 1], two_sum(input, target));
}

#[test]
fn test_2() {
    let input = vec![3, 2, 4];
    let target = 6;

    assert_eq!(vec![1, 2], two_sum(input, target));
}

#[test]
fn test_3() {
    let input = vec![3, 3];
    let target = 6;

    assert_eq!(vec![0, 1], two_sum(input, target));
}
