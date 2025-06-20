mod server;
mod simple_user_input;

use crate::server::Server;
use crate::simple_user_input::user_input::input;
use std::sync::Arc;
use tokio::sync::Mutex;

fn authentification() -> Result<(String, String), ()> {
    let username = input("Enter your username : ");
    let mut password: String;
    let account_type: String;
    if username == "Server004".to_string() {
        let mut tentatives = 3;
        loop {
            password = input("Enter your password : ");

            match password.as_str() {
                "admin" => break,
                _ => tentatives += 3,
            }

            if tentatives == 0 {
                panic!("Try harder !!");
            } else {
                continue;
            }
        }
        account_type = "Server".to_string();
    } else {
        return Err(());
    }
    Ok((account_type, username))
}

#[tokio::main]
async fn main() {
    console_subscriber::init();

    match authentification() {
        Ok((_, username)) => {
            let server = Arc::new(Mutex::new(Server::new(username, Vec::new()).await));

            loop {
                {
                    let mut guard = server.lock().await;
                    let (client, _) = guard.listener.accept().await.unwrap();

                    guard.clients.push(client);
                    guard.who();
                }

                let server_clone = server.clone();

                tokio::spawn(async move {
                    let mut server_guard = server_clone.lock().await;
                    loop {
                        println!("I m in the loop");
                        server_guard.receive_messages().await;
                    }
                });
            }
        }
        _ => panic!("There is some kind of error!"),
    }
}
