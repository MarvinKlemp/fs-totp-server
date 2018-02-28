use user::ReadableUser;

#[derive(Serialize,Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
    pub apikey: Option<String>
}

impl UserDTO {
    pub fn from_readable_user(user: &ReadableUser) -> Self {
        UserDTO {
            username: user.username.clone(),
            password: "".to_string(),
            apikey: Some(user.apikey.to_string())
        }
    }
}

#[derive(Serialize,Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String
}