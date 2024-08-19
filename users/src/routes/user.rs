use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Deserialize)]
struct RegisterRequest {
    email:    String,
    password: String,
}

#[post("/register")]
async fn register(
    pool: web::Data<PgPool>,
    form: web::Json<RegisterRequest>,
) -> impl Responder {
    let hashed_password = hash(&from.password, DEFAULT_COST).unwrap();
    let user_id = Uuid::new_v4();

    let result = sqlx::query!(
        "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)",
        user_id,
        form.email,
        hashed_password
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok() => HttpResponse::Ok().json("User registered successfully"),
        Err() => HttpResponse::InternalServerError().json("Failed to register user"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}