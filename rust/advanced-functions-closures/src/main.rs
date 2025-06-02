pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn do_twice<T>(f: T, x: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(f(x))
}

fn int_to_string(i: &i32) -> String {
    i.to_string()
}

pub fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    println!("{}", do_twice(add_one, 5));

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(int_to_string).collect();

    println!("{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
    }

    let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_status);
}
