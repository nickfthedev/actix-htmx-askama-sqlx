
use actix_web::{get, web,  HttpResponse, Responder};
use actix_web::http::StatusCode;

use askama_actix::{Template, TemplateToResponse};

use crate::AppState;


#[derive(Template)]
#[template(path = "hello.html")] 
struct HelloTemplate<'a> { 
    name: &'a str, 
}

// Askama Route handler
#[get("/askama")]
async fn askamatest() -> impl Responder {
    let name = "world".to_owned();
    HelloTemplate { name: &name }.to_response()
}

// 404 not found handler
pub async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h3>404 Page not found</h3>"))
}

// Sample with GET Parameter
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

// Root
#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

// Sample which writes a task to DB 
#[get("addtodo")]
async fn addtodotest(data: web::Data<AppState>) -> impl Responder {

    sqlx::query("
        INSERT INTO todo (task, completed)
        VALUES ('Sample todo', false)
    ")
    .execute(&data.pool)
    .await
    .unwrap();

    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}! Todo added") // <- response with app_name
}