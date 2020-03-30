use serde::{Deserialize, Serialize};
use std::fmt;
//use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Serialize, Deserialize)]
pub struct UserSignup {
    pub email: String,
    pub password: String,
}

impl fmt::Display for UserSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Email for user: {}", self.email)
    }
}

//fn hash_password(password: String) -> Result<String, dyn std::error::Error> {
//    hash(password, DEFAULT_COST).unwrap()
//}
//
//fn verify_password(password: String, hashed: String) -> bool {
//    verify(password, &hashed).unwrap()
//}
