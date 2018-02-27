#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket_contrib::Json;

mod user;
use user::UserDTO;
use user::UserMap;


#[get("/user/<username>", format = "application/json")]
fn get_user(username: String, user_map: State<UserMap>) -> Option<Json<UserDTO>> {
    user_map.get(&username).map(|user: UserDTO| {
        Json(user)
    })
}

fn main() {
    use user::User;

    let user_map = UserMap::new();
    user_map.insert(User::new());


    rocket::ignite()
        .mount("/", routes![get_user])
        .catch(errors![not_found])
        .manage(user_map)
        .launch();
}

#[error(404)]
fn not_found() -> &'static str {
    "{status: \"error\", message: \"Resource not found.\"}"
}