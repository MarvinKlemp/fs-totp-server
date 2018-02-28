use std;
use std::collections::HashMap;
use std::sync::Mutex;

use rand::{thread_rng, Rng};

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

    pub fn insert(&self, user: User) -> Option<ReadableUser> {
        let readable = ReadableUser::from_user(&user);

        let mut map = self.map.lock().expect("Cannot write lock mutex");
        map.insert(user.username(), user);

        Some(readable)
    }

    pub fn get(&self, username: &str) -> Option<ReadableUser> {
        let map = self.map.lock().expect("Cannot read lock mutex");

        map.get(username).map(|user: &User| {
            ReadableUser::from_user(user)
        })
    }
}

use password_encoder::Hash;

#[derive(Clone, Debug)]
pub struct ApiKey {
    apikey: String
}

impl ApiKey {
    fn new() -> Self {
        let key: String = thread_rng().gen_ascii_chars().take(30).collect();

        ApiKey {
            apikey: key
        }
    }
}

impl std::string::ToString for ApiKey {
    fn to_string(&self) -> String {
        self.apikey.clone()
    }
}

#[derive(Debug)]
pub struct User {
    username: String,
    password: Hash,
    apikey: ApiKey
}

impl User {
    pub fn new(username: String, password: Hash) -> Self {
        User {
            username: username,
            password: password,
            apikey: ApiKey::new()
        }
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn password(&self) -> Hash {
        self.password.clone()
    }

    pub fn apikey(&self) -> ApiKey {
        self.apikey.clone()
    }
}

pub struct ReadableUser {
    pub username: String,
    pub password: Hash,
    pub apikey: ApiKey
}

impl ReadableUser {
    pub fn from_user(user: &User) -> Self {
        ReadableUser {
            username: user.username(),
            password: user.password(),
            apikey: user.apikey()
        }
    }
}
