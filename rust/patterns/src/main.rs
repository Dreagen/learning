fn main() {}

pub fn match_enum() {
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Hola Mundo!"),
        Language::Russian => println!("Hello in russian!"),
        lang => println!("Unsupported Langague {:?}", lang),
    }
}

pub fn if_let() {
    let autorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = autorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privalidged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }
}

pub fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

pub fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

pub fn let_statement() {
    let x = 5;
    let (x, y, z) = (1, 2, 3);
}

pub fn function_params() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn irrefutable_refutable() {
    // irrefutable - x will always match whatever is on the right
    let x = 5;

    // refutable - Some(x) will only match if x is the Some variant of the option
    let x: Option<i32> = None;
    if let Some(x) = x {
        println!("{}", x);
    }
}
