use std::{
    collections::{HashMap, VecDeque},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::{
    LoadBalancer, Log, Server,
    request_handler::{self},
};

pub struct LeastTrafficLoadBalancer {
    server_queues: Arc<HashMap<Server, Arc<Mutex<VecDeque<TcpStream>>>>>,
}

impl LeastTrafficLoadBalancer {
    pub fn new(servers: Vec<Server>) -> Self {
        let server_queues = servers
            .iter()
            .map(|s| (s.clone(), Arc::new(Mutex::new(VecDeque::new()))))
            .collect::<HashMap<_, _>>();

        LeastTrafficLoadBalancer {
            server_queues: Arc::new(server_queues),
        }
    }
}

impl LoadBalancer for LeastTrafficLoadBalancer {
    fn execute(&self, log: Arc<Mutex<Log>>, listener: TcpListener) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.server_queues.len() + 1 as usize)
            .build()
            .unwrap();

        let server_map = self.server_queues.clone();

        pool.scope(|s| {
            for (server, queue) in server_map.iter() {
                s.spawn(|_| {
                    println!("Starting worker thread for server: {}", server.address);
                    loop {
                        let mut queue_guard = queue.lock().unwrap();
                        if !queue_guard.is_empty() {
                            if let Some(stream) = queue_guard.pop_front() {
                                let queue_size = queue_guard.len();
                                drop(queue_guard);
                                println!(
                                    "Handling request for server: {} with queue size: {}",
                                    server.address, queue_size
                                );
                                request_handler::handle_connection(
                                    log.clone(),
                                    Ok(stream),
                                    server.clone(),
                                );
                            }
                        } else {
                            drop(queue_guard);
                            std::thread::sleep(std::time::Duration::from_millis(10));
                        }
                    }
                });
            }

            s.spawn(|_| {
                println!("Load balancer is now listening for incoming requests...");

                let mut server_provider = LeastTrafficServerProvider {
                    server_queues: self.server_queues.clone(),
                };

                for stream in listener.incoming() {
                    println!("New incoming request received");
                    server_provider.accept_new_request(stream.unwrap());
                }
            });
        });
    }
}

pub struct LeastTrafficServerProvider {
    pub server_queues: Arc<HashMap<Server, Arc<Mutex<VecDeque<TcpStream>>>>>,
}

impl LeastTrafficServerProvider {
    fn accept_new_request(&mut self, stream: TcpStream) {
        let mut server_with_least_traffic = None;
        let mut current_queue_size = usize::MAX;

        for (server, queue) in self.server_queues.iter() {
            let mutex_guard = queue.lock().unwrap();
            if mutex_guard.len() < current_queue_size {
                server_with_least_traffic = Some(server);
                current_queue_size = mutex_guard.len();
            }
        }

        let server = server_with_least_traffic.unwrap().clone();

        println!(
            "Assigning request to server: {} with queue size: {}",
            server.address, current_queue_size
        );

        self.server_queues
            .get(&server)
            .unwrap()
            .lock()
            .unwrap()
            .push_back(stream);
    }
}
