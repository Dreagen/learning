use std::{
    cmp,
    collections::{HashMap, VecDeque},
    net::{TcpListener, TcpStream},
};

use crate::{
    LoadBalancer, Server,
    request_handler::{self, ServerProvider},
};

pub struct LeastTrafficLoadBalancer {
    servers: HashMap<Server, VecDeque<TcpStream>>,
}

impl LeastTrafficLoadBalancer {
    pub fn new(servers: Vec<Server>) -> Self {
        let servers = servers
            .iter()
            .map(|s| (s.clone(), VecDeque::new()))
            .collect::<HashMap<_, _>>();

        LeastTrafficLoadBalancer { servers }
    }
}

impl LoadBalancer for LeastTrafficLoadBalancer {
    fn execute(&self, listener: TcpListener) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cmp::min(5, self.servers.len() as usize))
            .build()
            .unwrap();

        // let mut server_provider = RoundRobinServerProvider {
        //     servers: vec![Server {
        //         address: "Blah".to_string(),
        //         number: 0,
        //     }],
        //     next_server_index: 0,
        // };

        // pool.scope(|s| {
        //     for stream in self.listener.incoming() {
        //         let server = server_provider.get_next_server();
        //         s.spawn(|_| {
        //             request_handler::handle_connection(stream, server);
        //         });
        //     }
        // })
    }
}
