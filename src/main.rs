#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

pub mod orchestrator;

//use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::Database;

use orchestrator::Orchestrator;

fn open_chokhmah_db() -> Database {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    client.db("chokhmah")
}

fn main() {
    let db = open_chokhmah_db();
    let orchestrator = Orchestrator::new(db);
    
    let username = "kgarrison343";

    if orchestrator.find_username(username) {
        let pwd = orchestrator.get_password(username);
        println!("username: {}\npassword: {}", username, pwd);
    }
    else {
        println!("failure!");
    }
}
