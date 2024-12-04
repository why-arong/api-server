mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use db::establish_connection;
use handlers::{health_check, create_user, get_user, update_user, delete_user};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // 데이터베이스 연결 설정
    let pool = establish_connection().await.expect("Failed to create pool.");

    // 호스트와 포트 설정
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    println!("Starting server at {}", address);

    // HTTP 서버 설정
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // 데이터베이스 풀을 애플리케이션 데이터로 추가
            .route("/health", web::get().to(health_check)) // 헬스 체크 엔드포인트
            .service(
                web::scope("/users") // /users 경로 하위의 엔드포인트 설정
                    .route("", web::post().to(create_user)) // POST /users
                    .route("/{id}", web::get().to(get_user)) // GET /users/{id}
                    .route("/{id}", web::put().to(update_user)) // PUT /users/{id}
                    .route("/{id}", web::delete().to(delete_user)), // DELETE /users/{id}
            )
    })
        .bind(address)? // 서버 바인딩
        .run() // 서버 실행
        .await
}
