use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use std::collections::HashSet;

use crate::structs::Payload;

pub fn sign(payload: Payload, secret: &str) -> Result<String, Error> {
    let encoded_jwt = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(encoded_jwt)
}

//In this first approach we gon'avoid to check if exp time is correct
pub fn verify(token: &str, secret: &str) -> Result<Payload, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.required_spec_claims = HashSet::new();
    validation.validate_exp = false;

    let decoded_jwt = decode::<Payload>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?
    .claims;

    Ok(decoded_jwt)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_payload() -> Payload {
        let email = String::from("example@mail.com");
        let username = String::from("User");
        Payload { email, username }
    }

    #[test]
    fn jwt_creation() {
        let header = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9";

        assert!(sign(get_payload(), "SUPER_SECRET")
            .unwrap()
            .contains(header));
    }

    #[test]
    fn correct_payload() {
        let secret = "SUPER_SECRET";
        let encoded_jwt = sign(get_payload(), secret).unwrap();
        let payload = verify(&encoded_jwt, secret);

        assert_eq!(payload, Ok(get_payload()));
    }
}
