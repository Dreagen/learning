fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let largest = get_largest(number_list);
    // println!("largest number is {}", largest);
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let largest = get_largest(char_list);
    // println!("largest number is {}", largest);

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for num in number_list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
