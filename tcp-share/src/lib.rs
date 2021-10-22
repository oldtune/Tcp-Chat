use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

pub fn read_from_stream(stream: &TcpStream, callback: fn(String)) {
    let mut buffer = String::new();

    let mut buf_reader = BufReader::new(stream);
    buf_reader.read_line(&mut buffer).unwrap();

    callback(buffer);
}

pub fn write_to_stream(stream: &TcpStream, get_string: fn() -> String) {
    let string_buffer = get_string();
    let mut buf_writer = BufWriter::new(stream);
    buf_writer.write(string_buffer.as_bytes()).unwrap();
}
