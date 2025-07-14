fn main() {
    let rect = Rectangle {
        height: 30,
        width: 50,
    };

    let rect_smaller = Rectangle {
        height: 20,
        width: 40,
    };

    let rect_bigger = Rectangle {
        height: 31,
        width: 51,
    };

    let square = Rectangle::new_square(10);

    println!("Can hold rect_smaller: {}", rect.can_hold(&rect_smaller));
    println!("Can hold rect_bigger: {}", rect.can_hold(&rect_bigger));
    println!("Can hold square: {}", rect.can_hold(&square));

    println!("Rectangle: {:#?}", rect);

    println!("The area of the rectange is {} square pixels", rect.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn new_square(side_length: u32) -> Rectangle {
        Rectangle {
            width: side_length,
            height: side_length,
        }
    }
}
