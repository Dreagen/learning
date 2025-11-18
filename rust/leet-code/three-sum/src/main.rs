use std::collections::{HashMap, HashSet};

fn main() {
    let input = vec![-1, 0, 1, 2, -1, -4];
    three_sum(input);
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = nums
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut acc, n| {
            *acc.entry(n).or_insert_with(|| 0) += 1;
            acc
        });

    let mut hash_set = HashSet::<i32>::new();
    let mut matching_triplets = vec![];
    loop {
        let next_value = get_next_value(&mut map);

        if let Some(next_value) = next_value {
            add_matching_triplets(next_value, &hash_set, &map, &mut matching_triplets);
            hash_set.insert(next_value);
        } else {
            return matching_triplets;
        }
    }
}

fn add_matching_triplets(
    next_value: i32,
    hash_set: &HashSet<i32>,
    map: &HashMap<i32, i32>,
    matching_triplets: &mut Vec<Vec<i32>>,
) {
    for n in hash_set {
        let value_to_find = (next_value + n) * -1;
        if map.contains_key(&value_to_find) {
            matching_triplets.push(vec![*n, next_value, value_to_find]);
        }
    }
}

fn get_next_value(map: &mut HashMap<i32, i32>) -> Option<i32> {
    let next_value: Option<i32> = {
        let next = map.iter_mut().next();

        if let Some((key, value)) = next {
            let key = *key;
            if *value == 1 {
                map.remove(&key);
            } else {
                *value -= 1;
            }

            Some(key)
        } else {
            None
        }
    };
    next_value
}

#[cfg(test)]
mod tests {
    use crate::three_sum;

    #[test]
    fn example() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        assert_eq!(expected, three_sum(input));
    }
}
