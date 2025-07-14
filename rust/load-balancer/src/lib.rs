use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn start(servers: Vec<String>) {
    for server in &servers {
        println!("   {}", server);
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let server_count = servers.len();
    let mut current_server = 0;
    for stream in listener.incoming() {
        let tcp_stream = stream.unwrap();

        let server_number = current_server % server_count;
        println!("Forwarding request to server {server_number}");

        handle_connection(tcp_stream, &servers[server_number]);
        current_server += 1;
    }
}

fn handle_connection(mut incoming_stream: TcpStream, server: &String) {
    let mut buf = [0; 1024];

    incoming_stream.read(&mut buf).unwrap();

    let mut tcp_stream = TcpStream::connect(server).unwrap();

    tcp_stream.write(&buf).unwrap();
    tcp_stream.read(&mut buf).unwrap();

    incoming_stream.write(&buf).unwrap();
}
