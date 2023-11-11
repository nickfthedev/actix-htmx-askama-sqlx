//
use crate::AppState;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use askama_actix::TemplateToResponse;

use crate::model::todo::{AddTodo, TodoItem, UpdateTodo};
use crate::model::user::AddUser;
use crate::utils::view::{create_toast_header, ToastType};
use crate::view::todo::{TodoIndex, TodoList, TodoListItemEdit};
// Add new user
//
#[post("/user")]
async fn add_user(
    data: web::Data<AppState>,
    web::Form(user): web::Form<AddUser>,
) -> impl Responder {
    // If Task is empty
    if user.email == "" {
        return HttpResponse::BadRequest()
            .insert_header((
                "hx-trigger",
                create_toast_header(ToastType::Error, "Missing field email"),
            ))
            .body("Empty task");
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
