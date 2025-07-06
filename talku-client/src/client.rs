use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Client {
    pub id: u16,
    pub username: String,
    pub password: String,
    pub stream: TcpStream,
}

#[allow(unused)]
impl Client {
    pub fn new(username: String, password: String) -> Self {
        let mut stream = TcpStream::connect("127.0.0.1:8808").unwrap();
        Self {
            id: 1,
            username,
            password,
            stream,
        }
    }

    pub fn send_message(&mut self, message: &str) {
        if let Err(e) = self.stream.write_all(message.as_bytes()) {
            eprintln!("failed to write to the socket : err = {:?}", e);
        }
        self.stream.flush().map_err(|_| ());
    }

    pub fn receive_message(&mut self) {
        println!("I here to receive");
        let mut buf = [0; 1024];
        match self.stream.read(&mut buf) {
            Ok(0) => {
                println!("The connection with the server got closed!!");
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buf[..n]);
                println!("received...\n{}", received);
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
            }
            _ => {
                println!("no message was sent");
            }
        };
    }
}
