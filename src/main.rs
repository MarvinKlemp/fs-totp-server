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
use user::{User, ReadableUser, UserMap, ApiKey};

mod api;
use api::{ReturnUser, CreateUser, UsernamePasswordLogin};

mod password_encoder;
use password_encoder::{PasswordEncoder, Sha256PasswordEncoder};

#[post("/login", format = "application/json", data = "<json>")]
fn post_login(json: Json<UsernamePasswordLogin>, user_map: State<UserMap>) -> Option<Json<ReturnUser>> {
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
            return Some(Json(ReturnUser::from_readable_user(&user)));
        }
    }

    None
}

#[post("/user", format = "application/json", data = "<json>")]
fn post_user(json: Json<CreateUser>, user_map: State<UserMap>) -> Result<Created<Json<ReturnUser>>, String> {
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
        Some(Json(ReturnUser::from_readable_user(&result_dto)))
    ))
}

#[get("/user", format = "application/json")]
fn get_user(apikey: ApiKey, user_map: State<UserMap>) -> Option<Json<ReturnUser>> {
    user_map.find_with_apikey(&apikey).map(|user: ReadableUser| {
        Json(ReturnUser::from_readable_user(&user))
    })
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