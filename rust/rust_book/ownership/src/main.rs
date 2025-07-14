fn main() {
    let s = String::from("something");
    takes_ownership(s);
    println!("{}!", s);

    let i = 5;
    makes_copy(i);
    println!("{}!", i);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s1 = String::from("some string");
    let s2 = takes_and_gives_back(s1);
    println!("{}", s2);

    let check_length = String::from("check my length");
    let length = calculate_length(&check_length);
    println!("{} has length {}", check_length, length);

    let mut s1 = String::from("hello");
    change(&mut s1);
}

fn takes_ownership(some_string: String) {
    println!("{}!", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}!", some_integer);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
