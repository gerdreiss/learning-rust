pub mod commands;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schema;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::Error;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), Error> {
    PasswordHash::new(hash)
        .and_then(|phash| Argon2::default().verify_password(password.as_bytes(), &phash))
}

pub fn generation_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}
