//
use crate::AppState;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use askama_actix::TemplateToResponse;

use crate::model::todo::{AddTodo, TodoItem, UpdateTodo};
use crate::model::user::AddUser;

use crate::utils::view::{create_toast_header, ToastType};
use crate::utils::form::verify_email;

use crate::view::todo::{TodoIndex, TodoList, TodoListItemEdit};

// Add new user
//
#[post("/user")]
async fn add_user(
    data: web::Data<AppState>,
    web::Form(user): web::Form<AddUser>,
) -> impl Responder {
    // If Task is empty
    if !verify_email(user.email.as_str()) {
        return HttpResponse::BadRequest()
            .insert_header((
                "hx-trigger",
                create_toast_header(ToastType::Error, "Missing or invalid email address"),
            ))
            .body("Missing or invalid email address");
    }
    // Query Insert
    let add_query = sqlx::query!(r#"INSERT INTO "user" (email) VALUES ($1)"#, user.email);
    add_query.execute(&data.pool).await.unwrap();

    return HttpResponse::Accepted()
        .insert_header((
            "hx-trigger",
            create_toast_header(ToastType::Success, "User created"),
        ))
        .body("User added");
}
