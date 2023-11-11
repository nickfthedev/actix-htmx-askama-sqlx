pub mod common;
pub mod todo;
pub mod user;

use actix_web::{get, web, Responder};

use askama_actix::{Template, TemplateToResponse};

// Root
#[get("/health")]
async fn health() -> impl Responder {
    "OK!"
}
