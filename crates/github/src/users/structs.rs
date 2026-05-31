use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserResponse {
    pub login: String,
}
