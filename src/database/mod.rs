use std::env;
use mongodb::sync::{Collection, Client};
use dotenv::dotenv;

pub mod user;
pub mod book;

use user::User;
use self::book::Book;

pub struct MongoRepo {
    book_col: Collection<Book>,
    users_col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
       dotenv().ok(); 
       let uri = match env::var("MONGOURI") {
            Ok(val) => val.to_string(),
            Err(err) => format!("Error loading environment variable: {}", err),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("bookclubDB");
        let book_col: Collection<Book> = db.collection("Book");
        let users_col: Collection<User> = db.collection("User");

        Self { book_col, users_col }
    }

}
