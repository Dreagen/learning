use std::fmt::Display;

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let wrapper = Wrapper(vec!["hello".to_string(), "world".to_string()]);

    println!("{}", wrapper);
}

type Kilometers = i32;
type Miles = i32;
pub fn type_synonyms() {}

pub fn convert_to_miles(k: Kilometers) -> Miles {
    k * 2
}
