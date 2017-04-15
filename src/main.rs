#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate ring;
extern crate rustc_serialize;

pub mod hashing;
pub mod data_access;

//use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::Database;

use data_access::DataAccess;

use std::io::{stdin,stdout,Write};

fn open_chokhmah_db() -> Database {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    client.db("chokhmah")
}

fn add_new_user(data_access: &DataAccess, username: &str, password: &str) {
    if !(data_access.find_username(username)){
        match data_access.insert_new_user(username, password) {
            Ok(_) => println!("User {}, added successfully", username),
            Err(e) => println!("{}", e),
        }
    }else{
        println!("Username already exists!");
    }
}

fn get_username_from_user() -> String {
    let mut input = String::new();
    print!("Username: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid username");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

fn get_password_from_user() -> String {
    let mut input = String::new();
    print!("Password: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid password");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

fn main() {
    let db = open_chokhmah_db();
    let data_access = DataAccess::new(db);
    
    let username = get_username_from_user();

    if data_access.find_username(&username) {
        //let pwd = data_access.get_password(&username);
        //println!("username: {}\npassword: {}", &username, pwd);
    }
    else {
        println!("User does not yet exist. Please enter a password for new user: {}", username);
        let password = get_password_from_user();

        add_new_user(&data_access, &username, &password);
    }
}
