use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:9827").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(error) => { println!("Error: {}", error) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer;
    loop {
        buffer = [0; 512];
        let _ = match stream.read(&mut buffer) {
            Err(e) => panic!("Error: {}", e),
            Ok(message) => {
                if message == 0 {
                    break;
                }
                message
            },
        };

        match stream.write(&buffer) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}
