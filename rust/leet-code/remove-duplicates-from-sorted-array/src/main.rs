fn main() {
    println!(
        "{}",
        remove_duplicates(&mut vec![1, 2, 2, 2, 3, 4, 4, 5, 6, 6])
    );
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut unique = 1;

    let mut current = nums[0];
    for i in 1..nums.len() {
        let next = nums[i];
        if next != current {
            unique += 1;
            nums[unique - 1] = next;
            current = next;
        }
    }

    return unique as i32;
}
