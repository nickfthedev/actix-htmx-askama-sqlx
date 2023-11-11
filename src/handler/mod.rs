pub mod common;
pub mod todo;
pub mod user;

use actix_web::{get, Responder};

// Root
#[get("/health")]
async fn health() -> impl Responder {
    "OK!"
}
