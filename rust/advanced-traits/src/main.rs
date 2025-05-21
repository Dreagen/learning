pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;

impl Iterator for Counter {
}

fn main() {
    println!("Hello, world!");
}
