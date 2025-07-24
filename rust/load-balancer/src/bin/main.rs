use load_balancer::{Server, start};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    servers: Vec<String>,
}

fn main() {
    let app_content = fs::read_to_string("app.toml").expect("Failed to read app.toml");

    let config: Config = toml::from_str(&app_content).expect("Failed to parse app.toml");

    start(
        config
            .servers
            .iter()
            .enumerate()
            .map(|(i, addr)| Server {
                address: addr.to_string(),
                number: i,
            })
            .collect(),
    );
}
