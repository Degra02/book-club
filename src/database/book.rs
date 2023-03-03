use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use mongodb::bson::extjson::de::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};

use super::MongoRepo;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub desc: String,
    pub release_date: String,

    pub isbn: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<String>>,
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
        let filter = doc! {"_id": obj_id};
        let book_detail = self
            .book_col
            .find_one(filter, None)
            .ok()
            .expect("Error retrieving Book information");
        Ok(book_detail.unwrap())
    }

    pub fn update_book(&self, id: &String, new_book: Book) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id };
        let new_doc = doc! {
            "$set": {
                "id": new_book.id,
                "title": new_book.title,
                "author": new_book.author,
                "release_date": new_book.release_date,
                "isbn": new_book.isbn,
                "labels": new_book.labels,
                "comments": new_book.comments
            }
        };
        let updated_doc = self
            .book_col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating searched book");
        Ok(updated_doc)
    }

    pub fn delete_book(&self, id: String) -> Result<DeleteResult, Error> {
        let book_id = ObjectId::parse_str(&id).unwrap();
        let filter = doc! {"_id": book_id };
        let delete_result = self
            .book_col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting the selected Book");
        Ok(delete_result)
    }

    pub fn get_book_title(&self, title: &String) -> Result<Book, Error> {
        let filter = doc! { "title": title };
        let book_detail = self
            .book_col
            .find_one(filter, None)
            .ok()
            .expect("Error retrieving Book information");
        Ok(book_detail.unwrap())
    }

    pub fn get_all_books(&self) -> Result<Vec<Book>, Error> {
        let cursor = self
            .book_col
            .find(None, None)
            .ok()
            .expect("Error getting all book");
        let books = cursor.map(|doc| doc.unwrap()).collect();
        Ok(books)
    }
}
