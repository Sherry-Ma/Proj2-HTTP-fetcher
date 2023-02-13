use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let url = "http://www.google.com";

    let host = url.split("//").nth(1).unwrap();
    let address = format!("{host}:80");

    let mut stream = TcpStream::connect(address.as_str()).unwrap();

    let request = format!("GET / HTTP/1.1\r\nHost: {host}\r\n\r\n");
    let bytes_written = stream.write(request.as_bytes()).unwrap();

    if bytes_written != request.len() {
        println!("Error: Not all bytes were written to the stream");
        return;
    }

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    println!("{response}");
}
