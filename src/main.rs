pub mod handler;
pub mod model;
pub mod utils;
pub mod view;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tracing::info;
use tracing_subscriber;

use handler::common::not_found;
use handler::health;
use handler::todo::{
    add_todo, delete_todo, get_todo, render_update_todo, show_todo, toggle_completed, update_todo,
};
use handler::user::add_user;

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
    app_name: String,
    ip_address: String,
    port: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Logger
    tracing_subscriber::fmt::init();
    // Env
    dotenv::dotenv().ok();

    // Init DB
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    // Run Migrations
    sqlx::migrate!().run(&pool).await.unwrap();
    //
    let app_state = AppState {
        pool: pool.clone(),
        app_name: String::from("Sample Todo App"),
        ip_address: String::from("127.0.0.1"),
        port: 8080,
    };
    info!("🚀 {} --> Server lifting off!", app_state.app_name);
    info!(
        "Listening on http://{}:{}",
        app_state.ip_address, app_state.port
    );
    // Start Server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(Files::new("/public", "./public/").index_file("404.html"))
            .service(health)
            //To-Do Handler
            .service(show_todo)
            .service(get_todo)
            .service(add_todo)
            .service(update_todo)
            .service(render_update_todo)
            .service(delete_todo)
            .service(toggle_completed)
            // User handler
            .service(add_user)
            // 404 handler
            .default_service(
                // Custom 404
                web::route().to(not_found),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
