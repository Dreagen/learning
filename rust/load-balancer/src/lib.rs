use std::{
    io::{Read, Write},
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

    let server_count = servers.len();
    let mut current_server = 0;
    for stream in listener.incoming() {
        let tcp_stream = stream.unwrap();

        let server_number = current_server % server_count;

        let (mut tcp_stream, buf) = read_stream(tcp_stream);
        for i in 0..server_count {
            let server_number = (server_number + i) % server_count;
            print!("Forwarding request to server {server_number}");

            let result_data = forward_to_server(&buf, &servers[server_number]);
            if let Ok(result) = result_data {
                println!("{} -> Success! {}", GREEN, RESET);
                tcp_stream.write(&result).unwrap();
                break;
            }

            println!("{} -> Failed! - {}{}", RED, result_data.unwrap_err(), RESET);

            if i == server_count - 1 {
                println!(
                    "{}\nAll servers failed to respond, request not handled\n{}",
                    RED, RESET
                );
            }
        }

        current_server += 1;
    }
}

fn read_stream(mut stream: TcpStream) -> (TcpStream, Vec<u8>) {
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
