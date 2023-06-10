#![allow(warnings)]
use dotenv::dotenv;
use rocket::{form::validate::contains, serde::json::*, *};

mod libs;

use crate::libs::index_app::helpers::index;
use crate::libs::index_app::schema::{DummyRequest, DummyResponse, IndexResponse};

#[get("/")]
fn get_index_controller() -> Json<IndexResponse> {
    let resp = IndexResponse::create(index());
    Json(resp)
}

#[post("/", data = "<req>")]
fn post_index_controller(req: Json<DummyRequest>) -> Json<DummyResponse> {
    let resp = DummyResponse::create(req.text.clone(), Some(req.msg.clone()));
    Json(resp)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![get_index_controller, post_index_controller])
}
