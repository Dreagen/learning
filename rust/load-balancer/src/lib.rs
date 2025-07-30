use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex, mpsc::channel},
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

    handle_close(log.clone());

    match strategy {
        Strategy::RoundRobin => RoundRobinLoadBalancer::new(servers).execute(log, listener),
        Strategy::LeastTraffic => {
            LeastTrafficLoadBalancer::new(servers).execute(log, listener);
        }
    }
}

fn handle_close(log: Arc<Mutex<Log>>) {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("could not send close log"))
        .expect("Error setting ctrl-c handler");

    std::thread::spawn(move || {
        rx.recv().expect("Failed to receive signal");

        println!("\n\nShutting down load balancer: Statistics:\n");

        let log = log.lock().unwrap();
        let mut sorted: Vec<_> = log.server_calls.iter().collect();
        sorted.sort_by(|(server1, _), (server2, _)| server1.number.cmp(&server2.number));

        for (server, calls) in sorted {
            println!("Server {} received {} requests", server.number, calls);
        }

        std::process::exit(0);
    });
}

#[derive(Debug)]
pub enum Strategy {
    RoundRobin,
    LeastTraffic,
}

pub trait LoadBalancer {
    fn execute(&self, log: Arc<Mutex<Log>>, listener: TcpListener);
}
