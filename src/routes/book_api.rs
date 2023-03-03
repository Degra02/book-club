use crate::database::{book::Book, MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};
use rocket_dyn_templates::context;
use rocket_include_tera::{EtagIfNoneMatch, TeraContextManager, TeraResponse};

#[post("/book", data = "<new_book>")]
pub fn create_book(
    db: &State<MongoRepo>,
    new_book: Json<Book>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Book {
        id: None,
        title: new_book.title.to_owned(),
        author: new_book.author.to_owned(),
        desc: new_book.desc.to_owned(),
        release_date: new_book.release_date.to_owned(),
        isbn: new_book.isbn.to_owned(),
        labels: new_book.labels.to_owned(),
        comments: new_book.comments.to_owned(),
    };
    let book_detail = db.create_book(data);

    match book_detail {
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(Status::InternalServerError),
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

#[put("/book/<id>", data = "<new_book>")]
pub fn update_book(
    db: &State<MongoRepo>,
    id: String,
    new_book: Json<Book>,
    tera_cm: &State<TeraContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> TeraResponse {
    if id.is_empty() {
        return tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
            status_message: "BadRequest"
        });
    }

    let data = Book {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        title: new_book.title.to_owned(),
        author: new_book.author.to_owned(),
        desc: new_book.desc.to_owned(),
        release_date: new_book.release_date.to_owned(),
        isbn: new_book.isbn.to_owned(),
        labels: new_book.labels.to_owned(),
        comments: new_book.comments.to_owned(),
    };
    let update_result = db.update_book(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_info = db.get_book(&id);
                return match updated_info {
                    Ok(book) => {
                        tera_response!(disable_minify tera_cm, etag_if_none_match, "book_detail", context! {
                            book
                        })
                    }
                    Err(_) => {
                        tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
                            status_message: "InternalServerError"
                        })
                    }
                };
            } else {
                return tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
                    status_message: "Not Found"
                });
            }
        }
        Err(_) => {
            tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
                status_message: "InternalServerError"
            })
        }
    }
}

#[delete("/book/<id>")]
pub fn delete_book(
    db: &State<MongoRepo>,
    id: String,
    tera_cm: &State<TeraContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> TeraResponse {
    if id.is_empty() {
        return tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
            status_message: "BadRequest"
        });
    }

    let result = db.delete_book(id);
    match result {
        Ok(delted) => {
            if delted.deleted_count == 1 {
                return tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
                    msg: "Book successfully deleted"
                });
            } else {
                return tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
                    status_message: "Not Found"
                });
            }
        }
        Err(_) => {
            tera_response!(disable_minify tera_cm, etag_if_none_match, "status_message", context! {
                status_message: "InternalServerError"
            })
        }
    }
}

#[get("/book/<title>?title")]
pub fn get_book_title(db: &State<MongoRepo>, title: String) -> Result<Json<Book>, Status> {
    if title.is_empty() {
        return Err(Status::BadRequest);
    }
    let book_detail = db.get_book_title(&title);
    match book_detail {
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/books")]
pub fn get_all_books(db: &State<MongoRepo>) -> Result<Json<Vec<Book>>, Status> {
    let books = db.get_all_books();
    match books {
        Ok(vec) => Ok(Json(vec)),
        Err(_) => Err(Status::InternalServerError),
    }
}
