use core::panic;
use std::sync::{Arc, Mutex};

use crate::Server;

pub trait ServerProvider: Send {
    fn get_next_server(&mut self) -> Server;

    fn get_server_count(&self) -> usize;
}

pub enum Strategy {
    RoundRobin,
    LeastTraffic,
}

pub fn create(strategy: Strategy, servers: Vec<Server>) -> Arc<Mutex<dyn ServerProvider>> {
    match strategy {
        Strategy::RoundRobin => Arc::new(Mutex::new(RoundRobinServerProvider {
            servers,
            next_server_index: 0,
        })),
        Strategy::LeastTraffic => panic!("Least traffic strategy has not been implemented"),
    }
}

pub struct RoundRobinServerProvider {
    pub servers: Vec<Server>,
    pub next_server_index: usize,
}

impl ServerProvider for RoundRobinServerProvider {
    fn get_next_server(&mut self) -> Server {
        let next_server = self.servers[self.next_server_index].clone();
        self.next_server_index = (self.next_server_index + 1) % self.get_server_count();

        next_server
    }

    fn get_server_count(&self) -> usize {
        self.servers.len()
    }
}
