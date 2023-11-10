use crate::AppState;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use askama_actix::TemplateToResponse;

use crate::model::todo::{AddTodo, TodoItem, UpdateTodo};
use crate::view::todo::{TodoIndex, TodoList, TodoListItemEdit};
use crate::utils::view::{ToastType, create_toast_header}; 
    
///
/// Helper functions
///

async fn query_todo(pool: Pool<Postgres>) -> Vec<(i32, String, bool)> {
    let items = sqlx::query_as::<_, (i32, String, bool)>(
        "SELECT id, task, completed FROM todo ORDER BY id",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    return items;
}

fn transform_todo(items: Vec<(i32, String, bool)>) -> Vec<TodoItem> {
    let i: Vec<TodoItem> = items
        .iter()
        .map(|(id, task, completed)| TodoItem {
            id: id.clone(),
            task: task.clone(),
            completed: *completed,
        })
        .collect();

    i
}

///
/// Handler functions
/// 
#[get("/")]
async fn show_todo() -> impl Responder {
    TodoIndex {}.to_response()
}

//
// Get all todos from DB
//
#[get("/todo")]
async fn get_todo(data: web::Data<AppState>) -> impl Responder {
    let items = query_todo(data.pool.clone()).await;
    TodoList { todo: transform_todo(items) }.to_response()
}

//
// Add new Todo
//
#[post("/todo")]
async fn add_todo(
    data: web::Data<AppState>,
    web::Form(form): web::Form<AddTodo>,
) -> impl Responder {
    // If Task is empty
    if form.task == "" {
        return HttpResponse::BadRequest()
            .insert_header(("hx-trigger", create_toast_header(ToastType::Error, "Task cannot be empty!")))
            .body("Empty task");
    }
    // Query Insert
    let add_query = sqlx::query!("INSERT INTO todo (task) VALUES ($1)", form.task);
    add_query.execute(&data.pool).await.unwrap();

    // Load new Data
    let items = query_todo(data.pool.clone()).await;
    let mut response = TodoList { todo: transform_todo(items)}.to_response();
    let toast_header = create_toast_header(ToastType::Success, "Task added!");
    response.headers_mut().append(HeaderName::from_static("hx-trigger"), HeaderValue::from_str(&toast_header).unwrap());
    return response;
}

//
// Toggle done/undone
//
#[put("/todo/{id}")]
async fn toggle_completed(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    // Attempt to parse the `id` string into an i32 integer.
    let id_int = match id.parse::<i32>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };
    // Query UPDATE
    let qry = sqlx::query!(
        "UPDATE todo SET completed = NOT completed WHERE id=($1)",
        id_int
    );
    qry.execute(&data.pool).await.unwrap();
    // Send Success header
    return HttpResponse::Accepted()
        .append_header(("hx-trigger", create_toast_header(ToastType::Success, "Task updated successfully!")))
        .body("Ok");
}

//
// Renders the Update <li> item instead of the normal one
//
#[patch("/todo/edit/{id}")]
async fn render_update_todo(
    id: web::Path<i32>,
    web::Form(form): web::Form<UpdateTodo>,
) -> impl Responder {
    // Renders the edit form into the list
    TodoListItemEdit {
        id: &id,
        task: &form.task,
        completed: &form.get_completed(),
    }
    .to_response()
}

//
// Updates the Todolist Item
//
#[patch("/todo/{id}")]
async fn update_todo(
    id: web::Path<String>,
    web::Form(form): web::Form<UpdateTodo>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Attempt to parse the `id` string into an i32 integer.
    let id_int = match id.parse::<i32>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };
    // Error if task is empty
    if form.task == "" {
        return HttpResponse::BadRequest()
        .append_header(("hx-trigger", create_toast_header(ToastType::Warning, "Task cannot be empty!")))
        .body("Empty task");
    }
    // Query UPDATE
    let qry = sqlx::query!(
        "UPDATE todo SET task=($1),completed=($2) WHERE id=($3)",
        form.task,
        form.get_completed(),
        id_int
    );
    qry.execute(&data.pool).await.unwrap();

    // Query new list from DB and return with success headers
    let items = query_todo(data.pool.clone()).await;
    let mut response = TodoList { todo: transform_todo(items) }.to_response();
    let toast_header = create_toast_header(ToastType::Success, "Task updated!");
    response.headers_mut().append(HeaderName::from_static("hx-trigger"), HeaderValue::from_str(&toast_header).unwrap());
    return response;
}

//
// Delete Task
//
#[delete("/todo/{id}")]
async fn delete_todo(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    // Attempt to parse the `id` string into an i32 integer.
    let id_int = match id.parse::<i32>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };
    // Query DELETE
    let qry = sqlx::query!("DELETE FROM todo WHERE id=($1)", id_int);
    qry.execute(&data.pool).await.unwrap();
    // Query new todolist from DB
    let items = query_todo(data.pool.clone()).await;
    let mut response = TodoList { todo: transform_todo(items) }.to_response();
    let toast_header = create_toast_header(ToastType::Info, "Task deleted!");
    response.headers_mut().append(HeaderName::from_static("hx-trigger"), HeaderValue::from_str(&toast_header).unwrap());
    return response;
}
