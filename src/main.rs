#![allow(warnings)]
use rocket::{form::validate::contains, serde::json::*, *};

mod libs;

use crate::libs::index_app::helpers::index;
use crate::libs::index_app::schema::IndexResponse;

#[get("/")]
fn index_controller() -> Json<IndexResponse> {
    let resp = IndexResponse::create(index());
    Json(resp)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_controller])
}
