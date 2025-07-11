mod client;
mod user_input;

use client::ChatClient;
use user_input::input;

fn authentication() -> Result<(String, String), ()> {
    let name = input("Enter your username : ");
    let mut password: String;
    if name != "server".to_string() {
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
    } else {
        return Err(());
    }
    Ok((name, password))
}

fn main() {
    match authentication() {
        Ok((username, password)) => match ChatClient::connect("127.0.0.1:8088", username, password)
        {
            Ok(mut client) => client.run(),
            Err(e) => println!("Could not connect to server: {}", e),
        },
        Err(e) => println!("An error has occured : {:?}", e),
    }
}
