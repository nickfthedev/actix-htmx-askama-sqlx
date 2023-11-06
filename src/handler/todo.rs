use actix_web::{get, post, patch, delete, web ,Responder};
use askama_actix::{Template, TemplateToResponse};
use serde::Deserialize;
use crate::AppState;

///
/// Template Structs
/// 
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "todo_index.html")]
pub struct TodoList<'a> {
    #[template(escape = "none")]
    todo: Vec<TodoItem<'a>>,
}

pub struct TodoItem<'a> {
    task: &'a str,
    completed: bool,
}
///
/// Handler functions
/// 

#[get("/")]
async fn show_todo(data: web::Data<AppState>) -> impl Responder {
    let items = sqlx::query_as::<_, (String, bool)>("SELECT task, completed FROM todo ORDER BY id")
        .fetch_all(&data.pool)
        .await
        .unwrap();

        let items: Vec<TodoItem> = items.iter().map(|(task, completed)| TodoItem { task, completed: *completed }).collect();
        TodoList { todo: items }.to_response()
}

#[get("/todo")]
async fn get_todo(data: web::Data<AppState>) -> impl Responder {
    let items = sqlx::query_as::<_, (String, bool)>("SELECT task, completed FROM todo ORDER BY id")
        .fetch_all(&data.pool)
        .await
        .unwrap();

        let items: Vec<TodoItem> = items.iter().map(|(task, completed)| TodoItem { task, completed: *completed }).collect();
        TodoList { todo: items }.to_response()
}


#[allow(dead_code)]
#[derive(Deserialize)]
struct AddTodo {
    task: String,
}

#[post("/todo")]
async fn add_todo(data: web::Data<AppState>, web::Form(form): web::Form<AddTodo>) -> impl Responder {
    let new_task = form.task;
    let add_query = sqlx::query!("INSERT INTO todo (task) VALUES ($1)", new_task);
    add_query.execute(&data.pool).await.unwrap();
    let items = sqlx::query_as::<_, (String, bool)>("SELECT task, completed FROM todo ORDER BY id")
        .fetch_all(&data.pool)
        .await
        .unwrap();

        let items: Vec<TodoItem> = items.iter().map(|(task, completed)| TodoItem { task, completed: *completed }).collect();
        TodoList { todo: items }.to_response()
}

#[patch("/todo")]
async fn update_todo() -> impl Responder {
    "Hello world!"
}

#[delete("/todo")]
async fn delete_todo() -> impl Responder {
    "Hello world!"
}

// Sample which writes a task to DB 
#[get("/addtodo")]
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