use std::collections::HashMap;
use std::sync::Mutex;

pub struct UserMap {
    map: Mutex<HashMap<String, User>>
}

impl UserMap
{
    pub fn new() -> Self {
        UserMap {
            map: Mutex::new(HashMap::new())
        }
    }

    pub fn insert(&self, user: User) -> Option<UserDTO> {
        let dto = UserDTO::from_user(&user);

        let mut map = self.map.lock().expect("Cannot write lock mutex");
        map.insert(user.username(), user);

        Some(dto)
    }

    pub fn get(&self, username: &str) -> Option<UserDTO> {
        let map = self.map.lock().expect("Cannot read lock mutex");

        map.get(username).map(|user: &User| {
            UserDTO::from_user(user)
        })
    }
}

use password_encoder::Hash;

#[derive(Debug)]
pub struct User {
    username: String,
    password: Hash
}

impl User {
    pub fn new(username: String, password: Hash) -> Self {
        User {
            username: username,
            password: password
        }
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn password(&self) -> Hash {
        self.password.clone()
    }
}

#[derive(Serialize,Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String
}

impl UserDTO {
    pub fn from_user(user: &User) -> Self {
        UserDTO {
            username: user.username(),
            password: "".to_string()
        }
    }
}
