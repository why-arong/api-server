use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{User, NewUser};
use bcrypt::{hash, DEFAULT_COST};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub async fn create_user(pool: web::Data<PgPool>, item: web::Json<NewUser>) -> impl Responder {
    let new_user = item.into_inner();

    // 비밀번호 해시 생성
    let password_hash = hash(new_user.password, DEFAULT_COST).unwrap();

    // UUID 생성
    let user_id = Uuid::new_v4();

    // 현재 시간
    let now = chrono::Utc::now().naive_utc();

    // SQLx를 사용하여 사용자 삽입
    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, name, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, email, password_hash, created_at, updated_at
        "#,
        user_id,
        new_user.name,
        new_user.email,
        password_hash,
        now,
        now
    )
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(User {
            id: user.id,
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating user: {}", e)),
    }
}

pub async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let user_id = user_id.into_inner();

    let result = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching user: {}", e)),
    }
}

pub async fn update_user(pool: web::Data<PgPool>, user_id: web::Path<Uuid>, item: web::Json<NewUser>) -> impl Responder {
    let user_id = user_id.into_inner();
    let updated_user = item.into_inner();

    // 비밀번호 해시 생성
    let password_hash = hash(updated_user.password, DEFAULT_COST).unwrap();

    let now = chrono::Utc::now().naive_utc();

    let result = sqlx::query!(
        r#"
        UPDATE users
        SET name = $1, email = $2, password_hash = $3, updated_at = $4
        WHERE id = $5
        RETURNING id, name, email, password_hash, created_at, updated_at
        "#,
        updated_user.name,
        updated_user.email,
        password_hash,
        now,
        user_id
    )
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(User {
            id: user.id,
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error updating user: {}", e)),
    }
}

pub async fn delete_user(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let user_id = user_id.into_inner();

    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    )
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                HttpResponse::NotFound().body("User not found")
            } else {
                HttpResponse::Ok().body("User deleted")
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error deleting user: {}", e)),
    }
}