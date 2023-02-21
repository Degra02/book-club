#[macro_use] extern crate rocket;

mod routes;

use routes::homepage::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
