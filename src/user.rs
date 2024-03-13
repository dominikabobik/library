use std::{collections::HashMap, io::stdin};

use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
}

pub fn login() {
    println!("Login...");
}

pub fn create_account(user_list: &mut HashMap<Uuid, User>) {
    println!("Create an account...");
    println!("Enter your name:");
    let mut name: String = String::new();
    stdin().read_line(&mut name).unwrap();
    println!("Enter your password:");
    let mut password: String = String::new();
    stdin().read_line(&mut password).unwrap();
    let uuid: Uuid = Uuid::new_v4();
    user_list.insert(
        uuid,
        User {
            id: uuid,
            name: name,
            password: password,
        },
    );
}
