use dotenv::dotenv;
use mongodb::sync::{Client, Collection};
use std::env;

pub mod book;
pub mod user;

use self::book::Book;
use user::User;

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

        Self {
            book_col,
            users_col,
        }
    }
}
