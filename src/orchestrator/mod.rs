//use bson::Bson;
//use mongodb::{Client, ThreadedClient};
use mongodb::db::{Database, ThreadedDatabase};

pub struct Orchestrator {
    // A reference to the database to be operated on.
    pub db: Database,
}

impl Orchestrator {
    pub fn new(db: Database) -> Orchestrator {
        Orchestrator{
            db: db.clone(),
        }
    }

    pub fn find_username(&self, username: &str) -> bool {
        let user_doc = doc! {"username" => username};

        let mut cursor = self.db.collection("users").find(Some(user_doc.clone()), None).ok().expect("failed to execute find");
        
        match  cursor.next() {
            Some(Ok(_)) => {
                true
            }
            Some(Err(e)) => {
                panic!("{}", e);
            }
            None => {
                false
            }
        }
    }
}
