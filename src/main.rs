#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket_contrib::Json;
use rocket::response::status::Created;

mod user;
use user::{User, UserDTO, UserMap};

#[get("/user/<username>", format = "application/json")]
fn get_user(username: String, user_map: State<UserMap>) -> Option<Json<UserDTO>> {
    user_map.get(&username).map(|user: UserDTO| {
        Json(user)
    })
}


#[post("/user", format = "application/json", data = "<json>")]
fn post_user(json: Json<UserDTO>, user_map: State<UserMap>) -> Result<Created<Json<UserDTO>>, String> {
    let user_dto = UserDTO {
        username: json.0.username,
        password: json.0.password
    };

    let result_dto = user_map.insert(
        User::new(
            user_dto.username,
            user_dto.password
        )
    ).unwrap();


    Ok(Created(
        format!("/user/{}", result_dto.username),
        Some(Json(result_dto))
    ))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_user, post_user])
        .catch(errors![not_found])
        .manage(UserMap::new())
        .launch();
}

#[error(404)]
fn not_found() -> &'static str {
    "{\"status\": \"error\", \"message\": \"Resource not found.\"}"
}