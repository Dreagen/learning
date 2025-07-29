use std::{
    cmp,
    net::TcpListener,
    sync::{Arc, Mutex},
};

use crate::{
    LoadBalancer, Log, Server,
    request_handler::{self},
};

pub struct RoundRobinLoadBalancer {
    servers: Vec<Server>,
}

impl RoundRobinLoadBalancer {
    pub fn new(servers: Vec<Server>) -> Self {
        RoundRobinLoadBalancer { servers }
    }
}

impl LoadBalancer for RoundRobinLoadBalancer {
    fn execute(&self, log: Arc<Mutex<Log>>, listener: TcpListener) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cmp::min(5, self.servers.len() as usize))
            .build()
            .unwrap();

        let mut server_provider = RoundRobinServerProvider {
            servers: self.servers.clone(),
            next_server_index: 0,
        };

        pool.scope(|s| {
            for stream in listener.incoming() {
                let server = server_provider.get_next_server();
                s.spawn(|_| {
                    request_handler::handle_connection(log.clone(), stream, server);
                });
            }
        })
    }
}

pub struct RoundRobinServerProvider {
    pub servers: Vec<Server>,
    pub next_server_index: usize,
}

impl RoundRobinServerProvider {
    fn get_next_server(&mut self) -> Server {
        let next_server = self.servers[self.next_server_index].clone();
        self.next_server_index = (self.next_server_index + 1) % self.servers.len();

        next_server
    }
}
