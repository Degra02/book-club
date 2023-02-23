use crate::database::{Book, MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};


#[post("/book", data="<new_book>")]
pub fn create_book(
    db: &State<MongoRepo>,
    new_book: Json<Book> 
) -> Result<Json<InsertOneResult>, Status> {
   let data = Book {
        id: None,
        title: new_book.title.to_owned(),
        author: new_book.author.to_owned(),
        desc: new_book.desc.to_owned(),
        release_date: new_book.release_date.to_owned() 
   }; 
   let book_detail = db.create_book(data);

   match book_detail {
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(Status::InternalServerError)
   }
}

#[get("/book/<id>")]
pub fn get_book(
    db: &State<MongoRepo>,
    id: String
) -> Result<Json<Book>, Status> {
    
    if id.is_empty() {
        return Err(Status::BadRequest)
    }
    let book_detail = db.get_book(&id);
    match book_detail {
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(Status::InternalServerError),
    }
}
