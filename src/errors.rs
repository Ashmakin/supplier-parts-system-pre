// src/errors.rs

use actix_web::{error::BlockingError, HttpResponse, ResponseError};
use thiserror::Error;
use actix_multipart::MultipartError;
use std::io;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database Error: {0}")]
    DBError(#[from] sqlx::Error),

    #[error("Authentication Failed")]
    AuthError,

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    // --- 新增的错误变体 ---
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),

    #[error("Multipart Error: {0}")]
    MultipartError(#[from] MultipartError),

    #[error("Blocking Error: {0}")]
    BlockingError(#[from] BlockingError),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DBError(e) => {
                log::error!("Database Error: {:?}", e);
                HttpResponse::InternalServerError().json("An internal database error occurred")
            }
            AppError::AuthError => HttpResponse::Unauthorized().json("Authentication Failed"),
            AppError::BadRequest(message) => HttpResponse::BadRequest().json(message),
            AppError::InternalServerError(message) => {
                log::error!("Internal Server Error: {}", message);
                HttpResponse::InternalServerError().json("An unexpected error occurred")
            }
            // --- 新增的错误响应 ---
            AppError::IoError(e) => {
                log::error!("IO Error: {:?}", e);
                HttpResponse::InternalServerError().json("File system error")
            }
            AppError::MultipartError(e) => {
                log::error!("Multipart Error: {:?}", e);
                HttpResponse::BadRequest().json(format!("Invalid file upload: {}", e))
            }
            AppError::BlockingError(e) => {
                log::error!("Blocking operation failed: {:?}", e);
                HttpResponse::InternalServerError().json("An internal operation failed")
            }
        }
    }
}