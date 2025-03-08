use actix_web::{HttpResponse, ResponseError};

use crate::response::Response;

#[derive(Debug)]
pub enum AppError {
  Error = 1000,
  Database = 1001,
  Env = 1002,
  Authorization = 1003,
  UserNotFound = 1004,
  WebsiteExist = 1005,
}

impl AppError {
  pub fn code(&self) -> i32 {
    match self {
      Self::Error => 1000,
      Self::Database => 1001,
      Self::Env => 1002,
      Self::Authorization => 1003,
      Self::UserNotFound => 1004,
      Self::WebsiteExist => 1005,
    }
  }
  pub fn message(&self) -> String {
    match self {
      Self::Error => "".to_string(),
      Self::Database => "".to_string(),
      Self::Env => "Env error".to_string(),
      Self::Authorization => "Authorization error".to_string(),
      Self::UserNotFound => "UserNotFound error".to_string(),
      Self::WebsiteExist => "WebsiteExist error".to_string(),
    }
  }
}

impl From<sea_orm::DbErr> for AppError {
  fn from(err: sea_orm::DbErr) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Database
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<envy::Error> for AppError {
  fn from(err: envy::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Env
  }
}

impl From<dotenvy::Error> for AppError {
  fn from(err: dotenvy::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Env
  }
}

impl From<helpers::jwt::Error> for AppError {
  fn from(err: helpers::jwt::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<helpers::hash::BcryptError> for AppError {
  fn from(err: helpers::hash::BcryptError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<actix_web::http::header::ToStrError> for AppError {
  fn from(err: actix_web::http::header::ToStrError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl ResponseError for AppError {
  fn error_response(&self) -> HttpResponse {
    tracing::error!("{:#?}", self.message());
    HttpResponse::Ok().json(Response::<()> {
      data: None,
      code: self.code(),
      msg: self.message(),
    })
  }
}

impl std::fmt::Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
  }
}
