use crate::database::{book::Book, MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use rocket_dyn_templates::context;
use rocket_include_tera::{TeraResponse, TeraContextManager, EtagIfNoneMatch};


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
        release_date: new_book.release_date.to_owned(),
        isbn: new_book.isbn.to_owned(),
        labels: new_book.labels.to_owned(),
        comments: new_book.comments.to_owned()
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
    id: String,
    tera_cm: &State<TeraContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> TeraResponse {
    
    if id.is_empty() {
        return tera_response!(disable_minify tera_cm, etag_if_none_match, "book_detail", context! {
            book_title: "Not found" 
        });
    }
    let book_detail = db.get_book(&id).unwrap();
    

    tera_response!(disable_minify tera_cm, etag_if_none_match, "book_detail", context! {
        book: book_detail
    })
}


#[get("/book/title?<title>")]
pub fn get_book_title(
    db: &State<MongoRepo>,
    title: String
) -> Result<Json<Book>, Status> {
    
    if title.is_empty() {
        return Err(Status::BadRequest)
    }
    let book_detail = db.get_book_title(&title);
    match book_detail {
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/books")]
pub fn get_all_books(
    db: &State<MongoRepo>
    ) -> Result<Json<Vec<Book>>, Status> {
    let books = db.get_all_books();
    match books {
        Ok(vec) => Ok(Json(vec)),
        Err(_) => Err(Status::InternalServerError)
    }
}
