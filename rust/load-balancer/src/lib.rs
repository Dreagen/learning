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

        let handle_result = handle_connection(tcp_stream, &servers[server_number]);

        // TODO: don't drop request if it fails, try all other servers
        if handle_result.is_err() {
            println!(
                "Failed to connecting to server {} with error: {}",
                server_number,
                handle_result.unwrap_err()
            );
        }

        current_server += 1;
    }
}

fn handle_connection(
    mut incoming_stream: TcpStream,
    server: &String,
) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];

    let amount_read = incoming_stream.read(&mut buf).unwrap();
    let tcp_stream_result = TcpStream::connect(server);

    match tcp_stream_result {
        Ok(mut tcp_stream) => {
            tcp_stream.write(&buf[..amount_read]).unwrap();
            let amount_read = tcp_stream.read(&mut buf).unwrap();

            incoming_stream.write(&buf[..amount_read]).unwrap();
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}
