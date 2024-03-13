use std::{
    collections::HashMap,
    io::{self, stdin},
};

use uuid::Uuid;

use crate::user::{create_account, login, User};
pub mod user;

fn main() {
    let mut user_list: HashMap<Uuid, User> = HashMap::new();

    while true {
        println!("Hello! Choose one of the following options:");
        println!("(1) Login");
        println!("(2) Create an account");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "1" {
            login();
        } else if input.trim() == "2" {
            create_account(&mut user_list);
            println!("Current user list:");
            for (id, user) in &user_list {
                println!("{id}: {0}", user.name);
            }
        } else {
            println!("Incorrect input. Bye!");
            return;
        }
    }
}
