mod server;
mod simple_user_input;

use crate::server::Server;
use crate::simple_user_input::user_input::input;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn authentification() -> Result<(String, String), ()> {
    let username = input("Enter your username : ");
    let mut password: String;
    let account_type: String;
    if username == "server".to_string() {
        let mut tentatives = 3;
        loop {
            password = input("Enter your password : ");

            match password.as_str() {
                "admin" => break,
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
    Ok((account_type, username))
}

fn main() {
    // console_subscriber::init();

    match authentification() {
        Ok((_, username)) => {
            println!("Server starting at || IP:localhost port:8808");
            println!("Server is listening...");

            let server = Arc::new(Mutex::new(Server::new(username)));

            let server_recv = server.clone();

            for stream in server_recv.lock().unwrap().listener.incoming() {
                let server_handle = server.clone();
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            server_handle.lock().unwrap().receive_messages(stream);
                        });
                    }
                    Err(e) => {
                        println!("Connection failed: {}", e);
                    }
                }
            }
        }
        _ => panic!("There is some kind of error!"),
    }
}
