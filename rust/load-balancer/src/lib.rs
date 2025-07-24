use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::server_providers::ServerProvider;
mod server_providers;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[derive(Clone, Debug)]
pub struct Server {
    pub address: String,
    pub number: usize,
}

pub fn start(servers: Vec<Server>) {
    println!("Load balancing between servers:");
    for server in &servers {
        println!("   {}", server.address);
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(5)
        .build()
        .unwrap();

    let server_provider = server_providers::create(server_providers::Strategy::RoundRobin, servers);

    pool.scope(|s| {
        for stream in listener.incoming() {
            let sp = server_provider.clone();
            s.spawn(move |_| {
                handle_connection(stream, sp);
            });
        }
    })
}

fn handle_connection(
    stream: Result<TcpStream, Error>,
    server_provider: Arc<Mutex<dyn ServerProvider>>,
) {
    let (mut tcp_stream, buf) = read_incoming_request(stream.unwrap());

    let mut sp = server_provider.lock().unwrap();
    let server_count = sp.get_server_count();

    for i in 0..server_count {
        let new_server = sp.get_next_server();
        let server_number = new_server.number;

        let result = forward_to_server(&buf, new_server);

        if let Ok(result_data) = result {
            println!(
                "{} -> Success from server {}! {}",
                GREEN, server_number, RESET
            );
            tcp_stream.write(&result_data).unwrap();
            break;
        }

        println!(
            "{} -> Failure from server {}! - {}{}",
            RED,
            server_number,
            result.unwrap_err(),
            RESET
        );

        if i == server_count - 1 {
            println!(
                "{}\nAll servers failed to respond, request not handled\n{}",
                RED, RESET
            );
        }
    }
}

fn read_incoming_request(mut stream: TcpStream) -> (TcpStream, Vec<u8>) {
    let mut buf = [0; 1024];

    let amount_read = stream.read(&mut buf).unwrap();

    (stream, buf[..amount_read].to_vec())
}

fn forward_to_server(incoming_data: &Vec<u8>, server: Server) -> Result<Vec<u8>, std::io::Error> {
    println!("Forwarding request to server {}", server.number);

    let mut buf = [0; 1024];
    let tcp_stream_result = TcpStream::connect(server.address);

    match tcp_stream_result {
        Ok(mut tcp_stream) => {
            tcp_stream.write(&incoming_data).unwrap();
            let amount_read = tcp_stream.read(&mut buf).unwrap();

            return Ok(buf[..amount_read].to_vec());
        }
        Err(e) => {
            return Err(e);
        }
    }
}
