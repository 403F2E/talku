mod client;
mod simple_user_input;

use crate::client::Client;
use crate::simple_user_input::user_input::input;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn authentification() -> Result<(String, String, String), ()> {
    let username = input("Enter your username : ");
    let mut password: String;
    let account_type: String;
    if username != "Server004".to_string() {
        let mut tentatives = 3;
        loop {
            password = input("Enter your password : ");

            match password.as_str() {
                "guest" => break,
                _ => tentatives -= 1,
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

fn main() {
    match authentification() {
        Ok((_, username, password)) => {
            let client = Arc::new(Mutex::new(Client::new(username, password)));
            println!(
                "You have established a connection with the server. Type ('quit' or 'exit') to close this connection."
            );

            let recv_client = Arc::clone(&client);

            thread::spawn(move || {
                loop {
                    recv_client.lock().unwrap().receive_message();
                }
            });

            let mut guard = client.lock().unwrap();
            loop {
                let mut message = guard.username.clone();
                message.push_str(" : ");
                let payload = input(&message.as_str());
                message.push_str(payload.as_str());

                guard.send_message(&message);
            }
        }
        _ => panic!("There is some kind of error!"),
    }
}
