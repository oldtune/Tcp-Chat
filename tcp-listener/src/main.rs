use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::thread;
use std::{net::TcpListener, panic::panic_any};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_read = stream.try_clone().unwrap();
                thread::spawn(move || handle_tcp_read(stream_read));

                let stream_write = stream.try_clone().unwrap();
                thread::spawn(move || handle_tcp_write(stream_write));
            }
            Err(err) => panic_any(err),
        }
    }
}

fn handle_tcp_read(stream: TcpStream) {
    loop {
        let mut string_buffer = String::new();
        let mut buf_reader = BufReader::new(&stream);
        buf_reader.read_line(&mut string_buffer).unwrap();
        print!("{}", string_buffer);
    }
}

fn handle_tcp_write(stream: TcpStream) {
    loop {
        let mut string_buffer = String::new();
        stdin().read_line(&mut string_buffer).unwrap();

        let mut buf_writer = BufWriter::new(&stream);
        buf_writer.write(string_buffer.as_bytes()).unwrap();
    }
}
