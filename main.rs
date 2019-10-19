#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::json::Json;
use serde_json::{Value, json};

mod post;
use post::{Post, NewPost, IgnoreNoneFieldsUpdatePost};

pub mod schema;
pub mod driver;

#[post("/", data = "<post>")]
fn create(post: Json<NewPost>, connection: driver::Connection) -> Json<Post> {
    let insert = NewPost { ..post.into_inner() };
    Json(Post::create(insert, &connection))
}

#[get("/")]
fn read(connection: driver::Connection) -> Json<Value> {
    Json(json!(Post::read(&connection)))
}

#[put("/<id>", data = "<post>")]
fn update(id: i32, post: Json<IgnoreNoneFieldsUpdatePost>, connection: driver::Connection) -> Json<Value> {
    let update = IgnoreNoneFieldsUpdatePost { ..post.into_inner() };
    Json(json!({
        "success": Post::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: driver::Connection) -> Json<Value> {
    Json(json!({
        "success": Post::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .mount("/post", routes![create, update, delete])
        .mount("/posts", routes![read])
        .manage(driver::connect())
        .launch();
}