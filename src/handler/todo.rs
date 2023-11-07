use actix_web::{get, post, patch, put, delete, web ,Responder, HttpResponse};
use crate::AppState;
use tracing::info;

use askama_actix::TemplateToResponse;

use crate::view::todo::{TodoIndex, TodoList,TodoListItemEdit};
use crate::model::todo::{TodoItem, AddTodo, UpdateTodo};



///
/// Handler functions
/// 

#[get("/")]
async fn show_todo()-> impl Responder {
        TodoIndex{}.to_response()
}

#[get("/todo")]
async fn get_todo(data: web::Data<AppState>) -> impl Responder {
    let items = sqlx::query_as::<_, (i32, String, bool)>("SELECT id, task, completed FROM todo ORDER BY id")
        .fetch_all(&data.pool)
        .await
        .unwrap();

        let items: Vec<TodoItem> = items.iter().map(|(id, task, completed)| TodoItem { id, task, completed: *completed }).collect();
        TodoList { todo: items }.to_response()
}

#[post("/todo")]
async fn add_todo(data: web::Data<AppState>, web::Form(form): web::Form<AddTodo>) -> impl Responder {
    if form.task == "" {
        return HttpResponse::BadRequest().body("Empty task")
    }
    let add_query = sqlx::query!("INSERT INTO todo (task) VALUES ($1)", form.task);
    add_query.execute(&data.pool).await.unwrap();

    let items = sqlx::query_as::<_, (i32, String, bool)>("SELECT id, task, completed FROM todo ORDER BY id")
        .fetch_all(&data.pool)
        .await
        .unwrap();

        let items: Vec<TodoItem> = items.iter().map(|(id, task, completed)| TodoItem { id, task, completed: *completed }).collect();
        TodoList { todo: items }.to_response()
}

#[put("/todo/{id}")]
async fn toggle_completed(id: web::Path<String>, 
    data: web::Data<AppState>) -> impl Responder {
    // Attempt to parse the `id` string into an i32 integer.
    let id_int = match id.parse::<i32>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };
        
    let qry = sqlx::query!("UPDATE todo SET completed = NOT completed WHERE id=($1)", id_int);
    qry.execute(&data.pool).await.unwrap();
    // Append header to show success message on website
    return HttpResponse::Accepted().append_header(("hx-trigger", "taskUpdated")).body("Ok");
    }
// Renders the Update <li> item instead of the normal one
#[patch("/todo/edit/{id}")]
async fn render_update_todo(id: web::Path<i32>, web::Form(form): web::Form<UpdateTodo>) -> impl Responder {
    TodoListItemEdit { id: &id, task: &form.task, completed: &form.get_completed() }.to_response()
}
// Updates the Todolist Item
#[patch("/todo/{id}")]
async fn update_todo(id: web::Path<String>,
    web::Form(form): web::Form<UpdateTodo>,
    data: web::Data<AppState>,
    ) -> impl Responder {

    info!("Completed: {}", form.get_completed());
    // Attempt to parse the `id` string into an i32 integer.
    let id_int = match id.parse::<i32>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };
    if form.task == "" {
        return HttpResponse::BadRequest().body("Empty task")
    }
    let qry = sqlx::query!("UPDATE todo SET task=($1),completed=($2) WHERE id=($3)", form.task, form.get_completed(), id_int);
    qry.execute(&data.pool).await.unwrap();

    let items = sqlx::query_as::<_, (i32, String, bool)>("SELECT id, task, completed FROM todo ORDER BY id")
        .fetch_all(&data.pool)
        .await
        .unwrap();

        let items: Vec<TodoItem> = items.iter().map(|(id, task, completed)| TodoItem { id, task, completed: *completed }).collect();
        TodoList { todo: items }.to_response()
}

#[delete("/todo/{id}")]
async fn delete_todo(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    // Attempt to parse the `id` string into an i32 integer.
    let id_int = match id.parse::<i32>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };

    let qry = sqlx::query!("DELETE FROM todo WHERE id=($1)", id_int);
    qry.execute(&data.pool).await.unwrap();

    let items = sqlx::query_as::<_, (i32, String, bool)>("SELECT id, task, completed FROM todo ORDER BY id")
        .fetch_all(&data.pool)
        .await
        .unwrap();

        let items: Vec<TodoItem> = items.iter().map(|(id, task, completed)| TodoItem { id, task, completed: *completed }).collect();
        TodoList { todo: items }.to_response()
}
