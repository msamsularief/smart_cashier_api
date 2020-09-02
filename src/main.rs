// use dotenv::dotenv;

mod db;
mod helpers;
use crate::helpers::{password, token};

fn main() {
    // Hanya test saja belum sepenuhnya jadi
    let token = token::generate_access_token();
    let password = password::generate_passhash("Ngawur123");
    let valid = password::password_match("Ngawur123", &password);

    println!(
        "Access token: {} Password: {} Match: {}",
        token, password, valid
    );
}
