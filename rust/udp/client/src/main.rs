use std::{io::{self, BufRead}, net::UdpSocket};

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0");
    if socket.is_err() {
        println!("Failed to bind to local port");
    }

    let socket = socket.unwrap();
    socket.connect("127.0.0.1:8080").expect("failed to connect to server");

    println!("\nSend messages to UDP echo server\n");

    for line in io::stdin().lock().lines() {
        socket.send(&line.unwrap().into_bytes()).expect("failed to send message");

        let mut buf = [0u8; 1024];
        let recv = socket.recv(&mut buf).expect("failed to recieve data from socket");

        let data = &buf[..recv];

        println!("{}{}{}", GREEN, String::from_utf8_lossy(data), RESET);
    }

}
