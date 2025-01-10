fn main() {
    let x = 5;
    let y: Option<i8> = None;

    let sum = x + y.unwrap_or(0);
    println!("Sum: {}", sum);
}
