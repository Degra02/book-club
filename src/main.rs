#[macro_use] extern crate rocket;

mod routes;
mod database;

use database::MongoRepo;
use routes::homepage::*;
use routes::book_api::*;


#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![index, files])
        .mount("/", routes![create_book, get_book])
}
