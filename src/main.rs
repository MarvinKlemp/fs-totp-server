#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate ring;
extern crate rand;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket_contrib::Json;
use rocket::response::status::Created;

mod user;
use user::{User, ReadableUser, UserMap};

mod api;
use api::{UserDTO, LoginDTO};

mod password_encoder;
use password_encoder::{PasswordEncoder, Sha256PasswordEncoder};

#[post("/login", format = "application/json", data = "<json>")]
fn post_login(json: Json<LoginDTO>, user_map: State<UserMap>) -> Option<Json<UserDTO>> {
    let password_encoder = Sha256PasswordEncoder::new(1001);

    let username: String = json.0.username;
    let password: String = json.0.password;

    if let Some(user) = user_map.get(&username) {
        let secured = password_encoder.is_password_valid(
            &user.password,
            &password,
            &username
        );


        if secured {
            return Some(Json(UserDTO::from_readable_user(&user)));
        }
    }

    None
}

#[get("/user/<username>", format = "application/json")]
fn get_user(username: String, user_map: State<UserMap>) -> Option<Json<UserDTO>> {
    user_map.get(&username).map(|user: ReadableUser| {
        Json(UserDTO::from_readable_user(&user))
    })
}

#[post("/user", format = "application/json", data = "<json>")]
fn post_user(json: Json<UserDTO>, user_map: State<UserMap>) -> Result<Created<Json<UserDTO>>, String> {
    let username: String = json.0.username;
    let password: String = json.0.password;

    let password_encoder = Sha256PasswordEncoder::new(1001);
    let encoded_password = password_encoder.encode(
        &password,
        &username
    );

    let result_dto = user_map.insert(
        User::new(
            username,
            encoded_password
        )
    ).unwrap();

    Ok(Created(
        format!("/user/{}", result_dto.username),
        Some(Json(UserDTO::from_readable_user(&result_dto)))
    ))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![post_login, get_user, post_user])
        .catch(errors![not_found])
        .manage(UserMap::new())
        .launch();
}

#[error(404)]
fn not_found() -> &'static str {
    "{\"status\": \"error\", \"message\": \"Resource not found.\"}"
}