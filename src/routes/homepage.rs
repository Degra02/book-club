use rocket::State;
use rocket_dyn_templates::context;
use rocket_include_tera::{EtagIfNoneMatch, TeraContextManager, TeraResponse};

#[get("/")]
pub fn homepage(
    tera_cm: &State<TeraContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> TeraResponse {
    tera_response!(disable_minify tera_cm, etag_if_none_match, "base", context! {
        title: "BookClub"
    })
}
