mod client;
mod simple_user_input;

use crate::client::Client;
use crate::simple_user_input::user_input::input;
use std::{process::exit, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    signal,
    sync::Mutex,
};

fn authentification() -> Result<(String, String, String), ()> {
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
    Ok((account_type, username, password))
}

#[tokio::main]
async fn main() {
    console_subscriber::init();

    match authentification() {
        Ok((_, username, password)) => {
            let client = Arc::new(Mutex::new(Client::new(username, password).await));

            let client_send = client.clone();

            tokio::spawn(async move {
                let mut guard_send = client_send.lock().await;

                println!("sending task is spawned!");

                loop {
                    let mut message = guard_send.username.clone();
                    message.push_str(" : ");
                    let payload = input(&message.as_str());
                    message.push_str(payload.as_str());
                    if let Err(_) = guard_send.send_message(&message).await {
                        break;
                    } else {
                        println!("message is sent at this stage");
                        println!("{:?}", message);
                    }
                    println!("I m still at the loop");
                }
            });

            println!("references count are : {}", Arc::strong_count(&client));
            let client_read = client.clone();

            tokio::spawn(async move {
                println!("the reading task is spawned");
                let mut guard_read = client_read.lock().await;

                loop {
                    let mut buf = [0; 1024];
                    println!("commance reading from the stream");
                    let n = match guard_read.stream.read(&mut buf).await {
                        Ok(0) => {
                            println!("The connection with the server got closed!!");
                            break;
                        }
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            continue;
                        }
                    };
                    println!("the data received is : {:?}", &n.to_ne_bytes());
                }
            });

            tokio::spawn(async move {
                if let Ok(_) = signal::ctrl_c().await {
                    exit({
                        client.lock().await.stream.shutdown().await.unwrap();
                        0
                    });
                }
            });
        }
        _ => panic!("There is some kind of error!"),
    }
}
