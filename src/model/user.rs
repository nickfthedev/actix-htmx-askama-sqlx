use serde::Deserialize;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AddUser {
    //pub uuid: Uuid, // Will be generated in handler
    pub name: String,
    //pub username: String, // Will be generated in handler 
    pub email: String,
    pub password: String,
    //pub avatar_path: Option<String>, // Will not be avaiable at registration
}
