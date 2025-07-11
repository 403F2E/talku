use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

struct Client {
    id: usize,
    stream: TcpStream,
}

impl Client {
    fn new(id: usize, stream: TcpStream) -> Self {
        Client { id, stream }
    }
}

#[allow(unused)]
pub struct ChatServer {
    name: String,
    password: String,
    listener: TcpListener,
    clients: Arc<Mutex<HashMap<usize, TcpStream>>>,
    next_id: usize,
}

impl ChatServer {
    pub fn new(address: &str, name: String, password: String) -> std::io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        Ok(ChatServer {
            name,
            password,
            listener,
            clients: Arc::new(Mutex::new(HashMap::new())),
            next_id: 0,
        })
    }

    pub fn run(&mut self) {
        println!(
            "Server {} started on {:?}. Waiting for connections...",
            self.name,
            self.listener.local_addr().unwrap()
        );

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.next_id += 1;
                    self.handle_new_connection(stream);
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }

    fn handle_new_connection(&self, stream: TcpStream) {
        let client = Client::new(self.next_id, stream.try_clone().unwrap());
        let clients = Arc::clone(&self.clients);

        clients
            .lock()
            .unwrap()
            .insert(client.id, client.stream.try_clone().unwrap());

        thread::spawn(move || {
            Self::handle_client(client, clients);
        });
    }

    fn handle_client(client: Client, clients: Arc<Mutex<HashMap<usize, TcpStream>>>) {
        let addr = client.stream.peer_addr().unwrap();
        println!("New connection from {}", addr);
        Self::broadcast_message(&clients, "A new client has joined!!\n", client.id);

        let mut reader = BufReader::new(client.stream.try_clone().unwrap());

        loop {
            let mut message = String::new();
            match reader.read_line(&mut message) {
                Ok(0) => break,
                Ok(_) => {
                    print!("Received from {}: {}", addr, message);
                    Self::broadcast_message(&clients, &message, client.id);
                }
                Err(e) => {
                    println!("Error reading from {}: {}", addr, e);
                    break;
                }
            }
        }

        println!("Connection closed with {}", addr);
        clients.lock().unwrap().remove(&client.id);
    }

    fn broadcast_message(
        clients: &Arc<Mutex<HashMap<usize, TcpStream>>>,
        message: &str,
        sender_id: usize,
    ) {
        let clients = clients.lock().unwrap();
        for (client_id, client_stream) in clients.iter() {
            if *client_id != sender_id {
                let mut stream = client_stream.try_clone().unwrap();
                stream.write_all(message.as_bytes()).unwrap();
            }
        }
    }
}
