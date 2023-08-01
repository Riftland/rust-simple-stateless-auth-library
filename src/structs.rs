use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Payload {
    pub email: String,
    pub username: String,
}
