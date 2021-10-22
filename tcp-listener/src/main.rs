use std::io::stdin;
use std::thread;
use std::{net::TcpListener, panic::panic_any};
use tcp_share;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_read = stream.try_clone().unwrap();
                thread::spawn(move || loop {
                    tcp_share::read_from_stream(&stream_read, |message| print!("{}", message))
                });

                let stream_write = stream.try_clone().unwrap();
                thread::spawn(move || loop {
                    tcp_share::write_to_stream(&stream_write, read_stdin)
                });
            }
            Err(err) => panic_any(err),
        }
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    buffer
}
