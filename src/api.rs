use user::ReadableUser;

#[derive(Serialize,Deserialize)]
pub struct ReturnUser {
    pub username: String,
    pub apikey: String
}

impl ReturnUser {
    pub fn from_readable_user(user: &ReadableUser) -> Self {
        ReturnUser {
            username: user.username.clone(),
            apikey: user.apikey.to_string()
        }
    }
}

#[derive(Serialize,Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String
}

#[derive(Serialize,Deserialize)]
pub struct UsernamePasswordLogin {
    pub username: String,
    pub password: String
}

#[derive(Serialize,Deserialize)]
pub struct ApiKeyLogin {
    pub apikey: String
}

use user::ApiKey;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        return Outcome::Success(ApiKey(keys[0].to_string()));
    }
}
