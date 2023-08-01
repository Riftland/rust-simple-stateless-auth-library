use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::Error};
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
    )?.claims;

    Ok(decoded_jwt)
}
