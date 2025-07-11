mod server;
mod user_input;

use server::ChatServer;
use user_input::input;

fn authentication() -> Result<(String, String), ()> {
    let name = input("Enter your username : ");
    let mut password: String;
    if name == "server".to_string() {
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
    } else {
        return Err(());
    }
    Ok((name, password))
}

fn main() {
    match authentication() {
        Ok((name, password)) => {
            let mut server = ChatServer::new("127.0.0.1:8088", name, password).unwrap();
            server.run();
        }
        Err(e) => println!("An erro has occured : {:?}", e),
    }
}
