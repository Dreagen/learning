fn main() {
    println!(
        "{}",
        remove_duplicates(&mut vec![1, 2, 2, 3, 4, 4, 5, 6, 6])
    );
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut unique = 0;

    if nums.is_empty() {
        return unique;
    }

    let mut nums2 = vec![0 as i32; nums.len()];

    for (i, ele) in nums.into_iter().enumerate() {
        if i == 0 {
            nums2[i] = *ele;
            unique += 1;
            continue;
        }

        match nums2.get(i - 1) {
            Some(value) => {
                if value != ele {
                    nums2[i] = *ele;
                    unique += 1;
                }
            }
            None => {}
        }
    }

    nums2.iter().for_each(|n| println!("{n}"));

    for (i, n) in nums2.iter().enumerate() {
        nums[i] = *n;
    }

    return unique;
}
