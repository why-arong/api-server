// use actix_web::{HttpResponse, ResponseError};
// use thiserror::Error;
//
// #[derive(Debug, Error)]
// pub enum MyError {
//     #[error("Database error: {0}")]
//     DatabaseError(#[from] sqlx::Error),
//
//     #[error("Internal server error")]
//     InternalError,
// }
//
// impl ResponseError for MyError {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             MyError::DatabaseError(_) => {
//                 HttpResponse::InternalServerError().body("Database error occurred.")
//             }
//             MyError::InternalError => HttpResponse::InternalServerError().body("Internal server error."),
//         }
//     }
// }
