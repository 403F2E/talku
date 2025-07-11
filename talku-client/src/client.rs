use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::thread;

#[allow(unused)]
pub struct ChatClient {
    username: String,
    password: String,
    stream: TcpStream,
}

impl ChatClient {
    pub fn connect(address: &str, username: String, password: String) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(ChatClient {
            stream,
            username,
            password,
        })
    }

    pub fn run(&mut self) {
        println!("Connected to server. Type your messages below (Ctrl+C to exit):");

        let mut receive_stream = self.stream.try_clone().unwrap();
        let username = self.username.clone();
        thread::spawn(move || {
            Self::receive_messages(&mut receive_stream, username);
        });

        self.send_messages();
    }

    fn receive_messages(stream: &mut TcpStream, username: String) {
        let mut reader = io::BufReader::new(stream);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    println!("\nDisconnected from server");
                    std::process::exit(0);
                }
                Ok(_) => {
                    print!("\r{}\n", buffer.trim());
                    print!("{} : ", username);
                    io::stdout().flush().unwrap();
                }
                Err(e) => {
                    println!("\nError receiving message: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    fn send_messages(&mut self) {
        let mut input = String::new();
        loop {
            print!("{} : ", self.username.as_str());
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let message: String = format!("{} : {}", self.username, input);

            if let Err(e) = self.stream.write_all(message.as_bytes()) {
                println!("Error sending message: {}", e);
                break;
            }
        }
    }
}
