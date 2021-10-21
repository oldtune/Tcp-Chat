use std::{
    io::{stdin, BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
    thread,
};

fn main() {
    let tcp_stream =
        TcpStream::connect("localhost:8080").expect("Failed to connect to localhost on port 8080");
    let tcp_stream_for_read = tcp_stream.try_clone().unwrap();
    let _ = thread::spawn(move || {
        read_from_stream(&tcp_stream_for_read);
    });

    loop {
        let mut buf_writer = BufWriter::new(&tcp_stream);
        let mut string_buffer = String::new();
        stdin().read_line(&mut string_buffer).unwrap();
        buf_writer.write(string_buffer.as_bytes()).unwrap();
    }
}

fn read_from_stream(tcp_stream: &TcpStream) {
    loop {
        let mut string_buffer = String::new();
        let mut buf_reader = BufReader::new(tcp_stream);
        buf_reader.read_line(&mut string_buffer).unwrap();
        print!("{}", string_buffer);
    }
}
