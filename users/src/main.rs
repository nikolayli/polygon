use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod routes;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url).await.expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
