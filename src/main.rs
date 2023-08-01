mod utils {
    pub mod jwt;
}
pub mod structs;

use std::error::Error;

use structs::Payload;
use utils::jwt;

fn get_sign_example() -> (Payload, String) {
    let payload = Payload {
        email: "example@gmail.com".to_string(),
        username: "User".to_string(),
    };
    let secret = String::from("SUPER_SECRET");

    (payload, secret)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (payload, secret) = get_sign_example();
    let encoded_jwt = utils::jwt::sign(payload, &secret)?;

    println!("> jwt: {encoded_jwt}");

    match jwt::verify(&encoded_jwt, &secret) {
        Ok(v) => println!("> payload from token: {:?}", v),
        Err(err) => println!("> debug error: {:?}", err),
    }

    Ok(())
}
