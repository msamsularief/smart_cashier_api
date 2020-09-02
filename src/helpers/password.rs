extern crate bcrypt;

use bcrypt::DEFAULT_COST;

/// Generate passhash bcrypt
pub fn generate_passhash(password: &str) -> String {
    return bcrypt::hash(password, DEFAULT_COST).unwrap();
}

/// Matching password with hash from generate_passhash result
pub fn password_match(password: &str, hashed: &str) -> bool {
    return bcrypt::verify(password, hashed).unwrap();
}
