pub mod common;
pub mod todo;

use actix_web::{get, web,   Responder};

use askama_actix::{Template, TemplateToResponse};

///
/// Template Structs
/// 
#[derive(Template)]
#[template(path = "hello.html")] 
pub struct HelloTemplate<'a> { 
    name: &'a str, 
}

///
/// Handler functions
/// 

// Askama Route handler
#[get("/askama")]
async fn askamatest() -> impl Responder {
    let name = "world".to_owned();
    HelloTemplate { name: &name }.to_response()
}


// Sample with GET Parameter
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

// Root
#[get("/health")]
async fn health() -> impl Responder {
    "OK!"
}

