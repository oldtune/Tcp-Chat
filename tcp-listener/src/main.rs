use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::{net::TcpListener, panic::panic_any};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_tcp_request(&mut stream);
            }
            Err(err) => panic_any(err),
        }
    }
}

fn handle_tcp_request(stream: &mut TcpStream) {
    println!("Connection established");
    let mut read_stream = BufReader::new(stream);

    loop {
        let mut string_buffer = String::new();
        let receive_bytes = read_stream
            .read_line(&mut string_buffer)
            .expect("Error when reading stream");

        if receive_bytes > 0 {
            print!("{}", string_buffer);
        } else {
            println!("Connection ended");
            return;
        }
    }
}
