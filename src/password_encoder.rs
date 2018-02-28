use ring::{digest, pbkdf2};

pub type Hash = [u8; digest::SHA256_OUTPUT_LEN];

pub trait PasswordEncoder {
    fn encode(&self, password: &str, salt: &str) -> Hash;

    fn is_password_valid(&self, hash: &Hash, password: &str, salt: &str) -> bool;
}

pub struct Sha256PasswordEncoder {
    iterations: u32
}

impl Sha256PasswordEncoder {
    pub fn new(iterations: u32) -> Self {
        Sha256PasswordEncoder {
            iterations
        }
    }
}

impl PasswordEncoder for Sha256PasswordEncoder {
    fn encode(&self, password: &str, salt: &str) -> Hash {
        let mut encoded = [0u8; digest::SHA256_OUTPUT_LEN];

        pbkdf2::derive(
            &digest::SHA256,
            self.iterations,
            salt.as_bytes(),
            password.as_bytes(),
            &mut encoded
        );

        encoded
    }

    fn is_password_valid(&self, hash: &Hash, password: &str, salt: &str) -> bool {
        let result = pbkdf2::verify(
            &digest::SHA256,
            self.iterations,
            salt.as_bytes(),
            password.as_bytes(),
            hash
        );

        match result {
            Ok(_) => true,
            Err(_) => false
        }
    }
}