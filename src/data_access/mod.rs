use bson::Bson;
use mongodb::db::{Database, ThreadedDatabase};
use rustc_serialize::base64::*;
use hashing;

pub struct DataAccess {
    /// A reference to the database to be operated on.
    pub db: Database,
}

impl DataAccess {
    pub fn new(db: Database) -> DataAccess {
        DataAccess{
            db: db.clone(),
        }
    }

    pub fn find_username(&self, username: &str) -> bool {
        let user_doc = doc! {"username" => username};

        let mut cursor = self.db.collection("users").find(Some(user_doc.clone()), None).expect("failed to execute find");
        
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

    pub fn get_password(&self, username: &str) -> Vec<u8> {
        let user_doc = doc! {"username" => username};

        let mut cursor = self.db.collection("users")
            .find(Some(user_doc.clone()), None)
            .expect("failed to execute find");

        let item = cursor.next();

        let doc = match item {
            Some(x) => x.expect("failed to execute"),
            _ => panic!("Password not found"),
        };

        match doc.get("password") {
            Some(&Bson::String(ref x)) => x.from_base64().expect("Invalid password hash"),
            _ => panic!("Password was expected to be a string!"),
        }
    }

    pub fn insert_new_user(&self, username: &str, password: &str)
                           -> Result<(), String> {
        let hashed_password = hashing::hash_password(username, password)
            .to_base64(Config {
                char_set: CharacterSet::Standard,
                newline: Newline::LF,
                pad: true,
                line_length: None,
            });
        
        let insert_doc = doc!("username" => username,
                              "password" => hashed_password);

        let coll = self.db.collection("users");
        let result = coll.insert_one(insert_doc, None);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("An error occurred".to_string()),
        }
    }
}
