use std::{collections::HashMap, thread, time::Duration};

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T, TInput, TResult>
where
    T: Fn(TInput) -> TResult,
{
    map: HashMap<TInput, TResult>,
    calculation: T,
}

impl<T, TInput, TResult> Cacher<T, TInput, TResult>
where
    T: Fn(TInput) -> TResult,
    TInput: Eq,
    TInput: std::hash::Hash,
    TInput: Copy,
    TResult: Copy,
{
    fn new(calculation: T) -> Cacher<T, TInput, TResult> {
        Cacher {
            map: HashMap::new(),
            calculation,
        }
    }

    fn value(&mut self, arg: TInput) -> TResult {
        match self.map.get(&arg) {
            Some(x) => *x,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", cacher.value(intensity));
        println!("Next, do {} situps", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}
