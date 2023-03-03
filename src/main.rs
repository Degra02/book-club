#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_include_tera;

use rocket_include_tera::TeraResponse;

mod database;
mod routes;

use database::MongoRepo;
use routes::book_api::*;
use routes::homepage::*;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .attach(TeraResponse::fairing(|tera| {
            tera_resources_initialize!(
                tera,
                "base" => "templates/base.html",
                "book_detail" => "templates/book_detail.html",
                "status_message" => "templates/status_message.html"
            )
        }))
        .manage(db)
        .mount("/", routes![homepage])
        .mount(
            "/",
            routes![
                create_book,
                get_book,
                update_book,
                delete_book,
                get_book_title,
                get_all_books
            ],
        )
}
