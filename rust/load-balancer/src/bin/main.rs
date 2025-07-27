use load_balancer::{Server, start};
use serde::Deserialize;
use std::{env, fs};

#[derive(Deserialize)]
struct Config {
    servers: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let strategy = args.get(1);

    let app_content = fs::read_to_string("app.toml").expect("Failed to read app.toml");

    let config: Config = toml::from_str(&app_content).expect("Failed to parse app.toml");

    start(
        strategy,
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
