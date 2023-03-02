#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_include_tera;

use rocket_include_tera::TeraResponse;

mod routes;
mod database;

use database::MongoRepo;
use routes::homepage::*;
use routes::book_api::*;


#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .attach(TeraResponse::fairing(|tera| {
            tera_resources_initialize!(
                tera,
                "base" => "templates/base.html"
            )
        }))
        .manage(db)
        .mount("/", routes![homepage])
        .mount("/", routes![create_book, get_book, get_book_title, get_all_books])
}
