use std::env;

use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;
use mongodb::sync::{Collection, Client};
use mongodb::bson::{Document, doc};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub desc: String,
    pub release_date: String,
}

pub struct MongoRepo {
    col: Collection<Book>
}

impl MongoRepo {
    pub fn init() -> Self {
       dotenv().ok(); 
       let uri = match env::var("MONGURI") {
        Ok(val) => val.to_string(),
        Err(err) => format!("Error loading environment variable: {}", err),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("bookclubDB");
        let col: Collection<Book> = db.collection("Book");

        Self { col }
    }

    pub fn create_book(&self, new_book: Book) -> Result<InsertOneResult, Error> {
       let new_doc = Book {
        id: None,
        ..new_book
       };
       let book = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating Book"); 

        Ok(book)
    } 

    pub fn get_book(&self, id: &String) -> Result<Book, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id:": obj_id};
        let book_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error retrieving Book information");
        Ok(book_detail.unwrap())
    }
}
