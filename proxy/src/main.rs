use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    loop {
        let packet: Vec<u8> = Vec::new();
        loop {
            let raw_usize_read_data = stream.read(&mut [0; 128]).unwrap();
            let raw_u8_read_data: &[u8] = &raw_usize_read_data.to_ne_bytes()[..];
            let utf8_read_data = std::str::from_utf8(raw_u8_read_data);
            if utf8_read_data.is_ok() {
                println!("{:?}", utf8_read_data.unwrap());
                if utf8_read_data.unwrap().contains("\0") {
                    break;
                }
            }
        }
        //stream.write(a);
        //println!("{:?}", a);
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:16384").unwrap();
    loop {
        match listener.accept() {
            Ok((_socket, addr)) => {
                println!("new client: {addr:?}");
                thread::spawn(|| {
                    handle_client(_socket);
                });
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}
