use std::{io::stdin, net::TcpStream, thread};
use tcp_share;

fn main() {
    let tcp_stream =
        TcpStream::connect("localhost:8080").expect("Failed to connect to localhost on port 8080");
    let tcp_stream_for_read = tcp_stream.try_clone().unwrap();
    let _ = thread::spawn(move || loop {
        tcp_share::read_from_stream(&tcp_stream_for_read, |message| print!("{}", message));
    });

    loop {
        tcp_share::write_to_stream(&tcp_stream, read_from_stdin);
    }
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    buffer
}
