#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate ring;

pub mod data_access;

//use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::Database;

use data_access::DataAccess;

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

fn main() {
    let db = open_chokhmah_db();
    let data_access = DataAccess::new(db);
    
    let username = "kgarrison";

    if data_access.find_username(username) {
        let pwd = data_access.get_password(username);
        println!("username: {}\npassword: {}", username, pwd);
    }
    else {
        println!("User does not yet exist. Please enter a password for new user: {}", username);
        let password = "whatever for now";

        add_new_user(&data_access, username, password);       
    }
}
