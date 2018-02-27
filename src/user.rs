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

    pub fn insert(&self, user: User) -> () {
        let mut map = self.map.lock().expect("Cannot write lock mutex");

        map.insert(user.username(), user);
    }

    pub fn get(&self, username: &str) -> Option<UserDTO> {
        let map = self.map.lock().expect("Cannot read lock mutex");

        map.get(username).map(|user: &User| {
            UserDTO::from_user(user)
        })
    }
}

pub struct User {
    name: String,
    password: String
}

impl User {
    pub fn new() -> Self {
        User {
            name: "marvin".to_string(),
            password: "password".to_string()
        }
    }

    pub fn username(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize,Deserialize)]
pub struct UserDTO {
    pub name: String
}

impl UserDTO {
    pub fn from_user(user: &User) -> Self {
        UserDTO {
            name: user.username()
        }
    }
}
