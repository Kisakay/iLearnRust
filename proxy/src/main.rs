use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:16384").unwrap();
    loop {
        for stream in listener.incoming() {
            println!("{:?}", listener);
            thread::spawn(||{
                handle_client(stream.unwrap())
            });
        }
    }
}
