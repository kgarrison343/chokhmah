use bson::Bson;
use mongodb::db::{Database, ThreadedDatabase};

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

    pub fn get_password(&self, username: &str) -> String {
        let user_doc = doc! {"username" => username};

        let mut cursor = self.db.collection("users")
            .find(Some(user_doc.clone()), None)
            .ok().expect("failed to execute find");

        let item = cursor.next();

        let doc = match item {
            Some(x) => x.ok().expect("failed to execute"),
            _ => panic!("Password not found"),
        };
        return match doc.get("password") {
            Some(&Bson::String(ref x)) => x.to_string(),
            _ => panic!("Password was expected to be a string!"),
        };
    }

    pub fn insert_new_user(&self, username: &str, password: &str)
                           -> Result<(), String> {
        let insert_doc = doc!("username" => username,
                             "password" => password);

        let coll = self.db.collection("users");
        let result = coll.insert_one(insert_doc, None);

        return match result {
            Ok(_) => Ok(()),
            Err(_) => Err("An error occurred".to_string()),
        }
    }
}
