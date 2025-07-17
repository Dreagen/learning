use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn start(servers: Vec<String>) {
    println!("Load balancing between servers:");
    for server in &servers {
        println!("   {}", server);
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(5)
        .build()
        .unwrap();

    let mut current_server_number = 0;
    pool.scope(|s| {
        for stream in listener.incoming() {
            let servers = &servers;
            s.spawn(move |_| {
                handle_connection(stream, current_server_number, &servers);
            });

            current_server_number += 1;
        }
    })
}

fn handle_connection(
    stream: Result<TcpStream, Error>,
    current_server_number: usize,
    servers: &Vec<String>,
) {
    let (mut tcp_stream, buf) = read_incoming_request(stream.unwrap());

    let mut server_number = current_server_number;
    for i in 0..servers.len() {
        let (server, new_server_number) = get_server(servers, server_number);
        server_number = new_server_number;

        println!("Forwarding request to server {new_server_number}");
        let result = forward_to_server(&buf, &server);

        if let Ok(result_data) = result {
            println!(
                "{} -> Success from server {}! {}",
                GREEN, new_server_number, RESET
            );
            tcp_stream.write(&result_data).unwrap();
            break;
        }

        println!(
            "{} -> Failure from server {}! - {}{}",
            RED,
            new_server_number,
            result.unwrap_err(),
            RESET
        );

        if i == servers.len() - 1 {
            println!(
                "{}\nAll servers failed to respond, request not handled\n{}",
                RED, RESET
            );
        }
    }
}

fn get_server(servers: &Vec<String>, current_server_number: usize) -> (String, usize) {
    let server_number = (current_server_number + 1) % servers.len();
    (servers[server_number].clone(), server_number)
}

fn read_incoming_request(mut stream: TcpStream) -> (TcpStream, Vec<u8>) {
    let mut buf = [0; 1024];

    let amount_read = stream.read(&mut buf).unwrap();

    (stream, buf[..amount_read].to_vec())
}

fn forward_to_server(incoming_data: &Vec<u8>, server: &String) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = [0; 1024];

    let tcp_stream_result = TcpStream::connect(server);

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
