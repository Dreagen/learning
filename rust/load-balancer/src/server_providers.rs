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
            current_index: 0,
        })),
        Strategy::LeastTraffic => panic!("Least traffic strategy has not been implemented"),
    }
}

pub struct RoundRobinServerProvider {
    pub servers: Vec<Server>,
    pub current_index: usize,
}

impl ServerProvider for RoundRobinServerProvider {
    fn get_next_server(&mut self) -> Server {
        let next_server = self.servers[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.get_server_count();

        next_server
    }

    fn get_server_count(&self) -> usize {
        self.servers.len()
    }
}
