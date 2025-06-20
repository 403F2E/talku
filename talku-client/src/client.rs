use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[allow(unused)]
#[derive(Debug)]
pub struct Client {
    pub id: u16,
    pub username: String,
    pub password: String,
    pub payload: String,
    pub stream: TcpStream,
}

#[allow(unused)]
impl Client {
    pub async fn new(username: String, password: String) -> Self {
        let mut stream = TcpStream::connect("127.0.0.1:8808").await.unwrap();
        Self {
            id: 1,
            username,
            password,
            payload: "Hello message".into(),
            stream,
        }
    }

    pub async fn send_message(&mut self, message: &String) -> Result<(), std::io::Error> {
        let buf = [0; 1024];
        if let Err(e) = self.stream.write_all(&buf).await {
            eprintln!("failed to write to the socket : err = {:?}", e);
            return Err(e);
        }
        Ok(())
    }
}
