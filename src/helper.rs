use actix_web::HttpRequest;

use crate::error::AppError;

pub fn extract_token(req: &HttpRequest) -> Result<String, AppError> {
  let auth_header = req
    .headers()
    .get("Authorization")
    .ok_or(AppError::Authorization)?
    .to_str()?;
  if !auth_header.starts_with("Bearer ") {
    return Err(AppError::Authorization);
  }
  Ok(auth_header[7..].to_string()) // Skip "Bearer " prefix
}
