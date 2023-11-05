pub mod handler;


use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use dotenv;
use pretty_env_logger;

use crate::handler::{greet,index,not_found};
use handler::{addtodotest, askamatest};

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
        .service(greet) // Greet Function
        .service(index)
        .service(addtodotest)
        .service(askamatest) 
        .default_service( // Custom 404
            web::route().to(not_found)
        )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}