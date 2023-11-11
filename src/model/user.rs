use serde::Deserialize;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AddUser {
    pub uuid: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar_path: Option<String>, // Assuming avatar_path is optional
}
