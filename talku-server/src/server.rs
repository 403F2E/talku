use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[allow(unused)]
#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub password: String,
    pub listener: TcpListener,
    pub clients: Vec<TcpStream>,
    pub payload: String,
}

#[allow(unused)]
impl Server {
    pub async fn new(name: String, users: Vec<u16>) -> Self {
        Self {
            name,
            password: "admin".to_string(),
            listener: TcpListener::bind("127.0.0.1:8808").await.unwrap(),
            clients: Vec::new(),
            payload: "Hello message".into(),
        }
    }

    pub async fn broadcast(&mut self, message: &[u8]) {
        println!("now I arrived to the broadcast function.");
        for client in self.clients.iter_mut() {
            if let Err(e) = client.write_all(message).await {
                eprintln!("failed to write to the socket : err = {:?}", e);
            }
        }
        println!("after I v finished to the broadcast function.");
    }

    pub fn who(&self) {
        for client in self.clients.iter() {
            println!("{:?}", client);
        }
    }

    pub async fn receive_messages(&mut self) {
        println!("I v arrived to the receive_messages function");

        let mut buf = [0; 1024];

        if self.clients.len() == 0 {
            println!("the list of clients is empty");
            return;
        }

        for i in 0..self.clients.len() {
            match self.clients[i].read(&mut buf).await {
                Ok(0) => {
                    println!("Client disconnected.");
                    self.clients.swap_remove(i);
                }
                Ok(n) => {
                    println!("Received {} bytes", n);
                    self.broadcast(&n.to_ne_bytes()).await;
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {:?}", e);
                    self.clients.clear();
                }
                _ => continue,
            }
        }

        println!("I v finished to the receive_messages function");
    }
}
