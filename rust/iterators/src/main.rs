pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];

    let mapped: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(mapped, vec![2, 3, 4]);
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got {}", value)
    }
}
