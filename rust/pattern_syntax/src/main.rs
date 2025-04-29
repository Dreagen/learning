fn main() {}

pub fn match_to_literal() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("anything"),
    }
}

pub fn match_to_named_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
}

pub fn match_mulitple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

pub fn match_ranges() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }

    let c = 'c';

    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

pub fn destructure() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub fn match_enums() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 250));

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(h, s, v)) => {
            println!("Red: {}, Green: {}, Blue: {}", h, s, v)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Hue: {}, Saturation: {}, Value: {}", h, s, v)
        }
    }
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

pub fn match_range() {
    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("First: {}, Last: {}", first, last),
    }
}

pub fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than 5: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

pub fn match_shadowing() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
}

enum Message2 {
    Hello { id: i32 },
}

pub fn at_symbol() {
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}
