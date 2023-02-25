use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;

use super::MongoRepo;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub desc: String,
    pub release_date: String,


    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<String>>
}

impl MongoRepo {
    pub fn create_book(&self, new_book: Book) -> Result<InsertOneResult, Error> {
       let new_doc = Book {
         id: None,
        ..new_book
       };
       let book = self
            .book_col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating Book"); 

        Ok(book)
    } 

    pub fn get_book(&self, id: &String) -> Result<Book, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let book_detail = self
            .book_col
            .find_one(filter, None)
            .ok()
            .expect("Error retrieving Book information");
        Ok(book_detail.unwrap())
    }

    pub fn get_book_title(&self, title: &String) -> Result<Book, Error> {
        let filter = doc! {"title": title};
        let book_detail = self.book_col
            .find_one(filter, None)
            .ok()
            .expect("Error retrieving Book information");
        Ok(book_detail.unwrap())
    }

    pub fn get_all_books(&self) -> Result<Vec<Book>, Error> {
        let cursor = self.book_col
            .find(None, None)
            .ok()
            .expect("Error getting all book");
        let books = cursor.map(|doc| doc.unwrap()).collect();
        Ok(books)
    }
}
