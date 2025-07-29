use std::net::TcpListener;

use crate::{
    least_traffic::{LeastTrafficLoadBalancer, LeastTrafficServerProvider},
    round_robin::RoundRobinLoadBalancer,
};
mod least_traffic;
mod request_handler;
mod round_robin;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Server {
    pub address: String,
    pub number: usize,
}

pub fn start(strategy_input: Option<&String>, servers: Vec<Server>) {
    let strategy = strategy_input
        .and_then(|s| {
            if s.to_lowercase() == "leasttraffic" {
                Some(Strategy::LeastTraffic)
            } else {
                None
            }
        })
        .unwrap_or(Strategy::RoundRobin);

    println!("Load balancing using {:?} between servers:", strategy);
    for server in &servers {
        println!("   {}", server.address);
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    match strategy {
        Strategy::RoundRobin => RoundRobinLoadBalancer::new(servers).execute(listener),
        Strategy::LeastTraffic => {
            LeastTrafficLoadBalancer::new(servers).execute(listener);
        }
    }
}

#[derive(Debug)]
pub enum Strategy {
    RoundRobin,
    LeastTraffic,
}

pub trait LoadBalancer {
    fn execute(&self, listener: TcpListener);
}
