use std::{
    io::{stdin, Read, Write},
    net::TcpStream,
};

fn main() {
    let mut tcp_stream = TcpStream::connect("localhost:8080").unwrap();
    loop {
        let mut buffer = vec![0; 1024];
        stdin()
            .read(&mut buffer)
            .expect("Can't seem to read from stdin");

        tcp_stream.write(&buffer).unwrap();
    }
}
