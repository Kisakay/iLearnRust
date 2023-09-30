use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:888").unwrap();
    loop {
        for stream in listener.incoming() {
            println!("{:?}", listener);
            handle_client(stream.unwrap());
        }

    }
    
}
