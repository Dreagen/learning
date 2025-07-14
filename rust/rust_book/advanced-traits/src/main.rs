use std::ops::Add;

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter;

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        return Some(0);
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        return Some(0);
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimetres(u32);

struct Metres(u32);

impl Add<Metres> for Millimetres {
    type Output = Millimetres;

    fn add(self, rhs: Metres) -> Millimetres {
        Millimetres(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("waving arms furiously");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("flying a plane")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
