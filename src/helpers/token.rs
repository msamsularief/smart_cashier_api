extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use rand::Rng;

/// Generate random value u64
pub fn generate_randval() -> u64 {
    let mut rng = rand::thread_rng();
    let val = rng.gen::<u64>();

    val
}

/// Generate access_token
pub fn generate_access_token() -> String {
    let mut hasher = Sha256::new();
    let token_val = generate_randval();
    // Merubah u64 menjadi karakter string
    let token_val_string: String = token_val.to_string();

    hasher.input_str(&token_val_string);
    let hex = hasher.result_str();

    hex
}
