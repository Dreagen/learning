use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;

use web_server::ThreadPool;

fn main() {
    let args: Vec<String> = env::args().collect();

    let port = args.get(1);
    let default_port = &"7878".to_string();
    let port = port.unwrap_or(&default_port);

    let mut delay: Option<u64> = None;
    if let Some(d) = args.get(2) {
        delay = Some(d.parse().expect("Delay must be a valid integer"));
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream, delay);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, delay: Option<u64>) {
    let mut buf = [0; 1024];

    stream.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        if let Some(sleep_for) = delay {
            std::thread::sleep(Duration::from_millis(sleep_for));
        }
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
