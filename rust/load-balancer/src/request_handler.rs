use std::{
    io::{Error, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{Log, Server};

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn handle_connection(log: Arc<Mutex<Log>>, stream: Result<TcpStream, Error>, server: Server) {
    log.lock().unwrap().log_call(server.clone());

    let (mut tcp_stream, buf) = read_incoming_request(stream.unwrap());

    let server_number = server.number;
    let result = forward_to_server(&buf, server);

    if let Ok(result_data) = result {
        println!(
            "{} -> Success from server {}! {}",
            GREEN, server_number, RESET
        );

        tcp_stream.write(&result_data).unwrap();
    } else {
        println!(
            "{} -> Failure from server {}! - {}{}",
            RED,
            server_number,
            result.unwrap_err(),
            RESET
        );
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
