use base64::{Engine, prelude::BASE64_STANDARD};
use cookie::Key;

fn main() {
    let key = Key::generate();
    let encoded_key = BASE64_STANDARD.encode(key.master());

    println!("Cookie key:\n{encoded_key:?}\n\nCopy and paste this value into the `.env` file.")
}
