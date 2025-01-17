pub use self::front_of_house::hosting;

use rand::{CryptoRng, Rng};
use std::io::*;

mod front_of_house;

fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
