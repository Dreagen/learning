use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
};

use crate::{least_traffic::LeastTrafficLoadBalancer, round_robin::RoundRobinLoadBalancer};
mod least_traffic;
mod request_handler;
mod round_robin;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Server {
    pub address: String,
    pub number: usize,
}

pub struct Log {
    pub server_calls: HashMap<Server, u32>,
}

impl Log {
    fn log_call(&mut self, server: Server) {
        *self.server_calls.entry(server).or_insert(0) += 1;
    }
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

    let log = Arc::new(Mutex::new(Log {
        server_calls: HashMap::new(),
    }));

    match strategy {
        Strategy::RoundRobin => RoundRobinLoadBalancer::new(servers).execute(log, listener),
        Strategy::LeastTraffic => {
            LeastTrafficLoadBalancer::new(servers).execute(log, listener);
        }
    }
}

#[derive(Debug)]
pub enum Strategy {
    RoundRobin,
    LeastTraffic,
}

pub trait LoadBalancer {
    fn execute(&self, log: Arc<Mutex<Log>>, listener: TcpListener);
}
