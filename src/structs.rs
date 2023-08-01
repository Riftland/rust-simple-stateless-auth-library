use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub email: String,
    pub username: String,
}
