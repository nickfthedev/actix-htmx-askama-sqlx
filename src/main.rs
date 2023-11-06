pub mod handler;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files::Files;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use dotenv;
use pretty_env_logger;

use handler::common::not_found;
use handler::{health, greet,askamatest};
use handler::todo::{addtodotest, show_todo, get_todo, add_todo, update_todo, delete_todo};


struct AppState {
    pool: Pool<Postgres>,
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Logger
    pretty_env_logger::init();
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
    
    // Start Server
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(AppState{
            pool: pool.clone(),
            app_name:String::from("Test App")
        }))
        .service(Files::new("/public", "./public/").index_file("404.html"))
        .service(greet) // Greet Function
        .service(health)
        .service(addtodotest)
        .service(askamatest) 
        //To-Do Handler
        .service(show_todo)
        .service(get_todo)
        .service(add_todo)
        .service(update_todo)
        .service(delete_todo)
        // 404 handler
        .default_service( // Custom 404
            web::route().to(not_found)
        )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}