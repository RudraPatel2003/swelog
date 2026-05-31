use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub login: String,
}
