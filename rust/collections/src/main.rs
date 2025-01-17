use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (k, v) in &map {
        println!("{} appears in sentence {} times", k, v);
    }

    println!("{:?}", map);
}
