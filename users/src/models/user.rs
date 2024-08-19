use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id:            Uuid,
    pub email:         String,
    pub password_hash: String,
    pub is_verified:   bool,
}