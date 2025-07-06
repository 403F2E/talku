use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

type ClientSet = Arc<Mutex<HashSet<TcpStream>>>;

#[allow(unused)]
#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub password: String,
    pub listener: TcpListener,
    pub clients: ClientSet,
}

#[allow(unused)]
impl Server {
    pub fn new(name: String) -> Self {
        Self {
            name,
            password: "admin".to_string(),
            listener: TcpListener::bind("127.0.0.1:8808").unwrap(),
            clients: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn broadcast(&mut self, message: &[u8]) {
        let mut clients = self.clients.lock().unwrap();
        for i in 0..clients.len() {
            println!("broadcasting to all users...");
            if let Err(e) = clients[i].write_all(message) {
                eprintln!("failed to write to the socket : err = {:?}", e);
            }
            client.flush().map_err(|_| ());
        }
    }

    pub fn who(&self) {
        for client in self.clients.lock().iter() {
            println!("{:?}", client);
        }
    }

    fn handle_client(mut stream: TcpStream, clients: ClientSet) {
        let peer_addr = stream.peer_addr().unwrap();
        println!("New connection from: {}", peer_addr);

        {
            let mut clients = *clients.lock().unwrap();
            clients.insert(stream.try_clone().unwrap());
        }

        let reader = BufReader::new(stream.try_clone().unwrap());
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => break,
            };

            println!("Received from {}: {}", peer_addr, line);

            let mut clients = clients.lock().unwrap();
            let mut disconnected = Vec::new();

            for client in clients.iter() {
                let mut client = client.try_clone().unwrap();
                if client.peer_addr().unwrap() != peer_addr {
                    if let Err(e) = writeln!(client, "{}: {}", peer_addr, line) {
                        println!("Error writing to client: {}", e);
                        disconnected.push(client.try_clone().unwrap());
                    }
                }
            }

            for dc in disconnected {
                clients.remove(&dc);
            }
        }

        {
            let mut clients = clients.lock().unwrap();
            clients.remove(&stream);
        }

        println!("Client disconnected: {}", peer_addr);
    }

    fn main() -> std::io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:12345")?;
        println!("Server listening on port 12345...");

        let clients: ClientSet = Arc::new(Mutex::new(HashSet::new()));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let clients = Arc::clone(&clients);
                    thread::spawn(move || {
                        handle_client(stream, clients);
                    });
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }
}
