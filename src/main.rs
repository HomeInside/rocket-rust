#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};
use rocket::http::RawStr;


mod users{

    use rocket_contrib::{Json, Value};

    // The type to represent the ID of a message.
    type ID = usize;

    #[derive(Serialize, Deserialize)]
    pub struct User {
        description: String,
        complete: bool
    }

    #[get("/user", format = "application/json")]
    pub fn get_user() -> Json<Value> {

        Json(json!({
            "status": "OK",
            "message": "retrieve user list",
            "data": ""
        }))
    }

    #[post("/user", format = "application/json", data = "<user>")]
    pub fn post_user(user: Json<User>) -> Json<Value> {
        Json(json!({
            "status": "OK",
            "message": "user created",
            "data": {"complete": user.complete}
        }))
    }

    #[put("/user/<id>", format = "application/json", data = "<user>")]
    pub fn put_user(id:ID, user: Json<User>) -> Json<Value> {
        Json(json!({
            "status": "OK",
            "message": "user updated!",
            "data": {
                "id": id,
                "description": user.description,
                "complete": user.complete
            }
        }))
    }

    #[delete("/user/<id>", format = "application/json")]
    pub fn del_user(id:ID) -> Json<Value> {
        Json(json!({
            "status": "OK",
            "message": format!("delete user with ID {}!", id),
            "data": ""
        }))
    }
}

#[error(400)]
fn bad_request(req: &rocket::Request) -> Json<Value> {
    Json(json!({
        "status": 400,
        "data": "Bad request",
        "error": format!("{} and error ocurred.", req.uri())
    }))
}

#[error(404)]
fn not_found(req: &rocket::Request) -> Json<Value> {
    Json(json!({
        "status": "Error",
        "data": "",
        "error": format!("Error: {}", req.uri())
    }))
}

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({
        "status": "OK",
        "message": "Hello World, Rust.",
        "data": ""
    }))
}

#[get("/hello/<name>")]
fn hello_world(name: &RawStr) -> Json<Value> {
    Json(json!({
        "status": "OK",
        "message": format!("Hello, {}!", name.as_str()),
        "data": ""
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            index,
            hello_world,

            users::get_user,
            users::post_user,
            users::put_user,
            users::del_user,
        ])
        .catch(errors![not_found, bad_request])
}

fn main() {
    rocket().launch();
}
