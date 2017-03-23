#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

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

fn main() {
    let db = open_chokhmah_db();
    let data_access = DataAccess::new(db);
    
    let username = "kgarrison343";

    if data_access.find_username(username) {
        let pwd = data_access.get_password(username);
        println!("username: {}\npassword: {}", username, pwd);
    }
    else {
        println!("failure!");
    }
}
