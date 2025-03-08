use actix_web::{
  HttpRequest, HttpResponse, get, post,
  web::{self, Data, Json, Path},
};
use entity::*;
use helpers::{hash, jwt, time::utc_now};
use mime_guess::from_path;
use sea_orm::{ActiveValue::Set, FromQueryResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AppState, assets::Asset, error::AppError, helper::extract_token, response::Response};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceData {
  user_agent: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageData {
  path: String,
  domian: String,
  title: String,
  url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackData {
  website_id: u32,
  r#type: String,
  visitor_id: String,
  device_data: DeviceData,
  page_data: PageData,
}

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => HttpResponse::Ok()
      .content_type(from_path(path).first_or_octet_stream())
      .body(content.data.into_owned()),
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

#[actix_web::get("/js/{_:.*}")]
async fn dist(path: web::Path<String>) -> HttpResponse {
  handle_embedded_file(&path)
}

#[actix_web::get("/")]
async fn index() -> HttpResponse {
  handle_embedded_file("index.html")
}

#[post("/api/track")]
async fn track_data(
  state: Data<AppState>,
  body: Json<TrackData>,
) -> Result<HttpResponse, AppError> {
  let TrackData {
    r#type,
    visitor_id,
    device_data,
    page_data,
    website_id,
  } = body.0;
  if !state.repo.website().has_website_by_id(website_id).await? {
    return Err(AppError::Error);
  }
  let active_event = entity::event::ActiveModel {
    website_id: Set(website_id),
    visitor_id: Set(visitor_id),
    r#type: Set(r#type),
    tag: Set("".to_string()),
    url_path: Set(page_data.path),
    url_query: Set(page_data.url),
    page_title: Set(page_data.title),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  state.repo.event().create_event(active_event).await?;
  Response::success(Some(()))
}

#[derive(Debug, Deserialize)]
struct AddUserData {
  username: String,
  password: String,
  email: String,
  display_name: String,
  avatar: String,
}

#[post("/api/user")]
pub async fn add_user(
  state: Data<AppState>,
  body: Json<AddUserData>,
) -> Result<HttpResponse, AppError> {
  let AddUserData {
    username,
    password,
    email,
    display_name,
    avatar,
  } = body.0;
  if state.repo.user().has_user_by_email(&email).await? {
    return Response::<()>::error(AppError::Error);
  }

  let role = if state.repo.user().is_first_user().await? {
    "admin".to_string()
  } else {
    "normal".to_string()
  };
  let hashed = hash::bcrypt(&password)?;
  let active_user = user::ActiveModel {
    username: Set(username),
    password: Set(hashed),
    role: Set(role),
    email: Set(email),
    display_name: Set(display_name),
    avatar: Set(avatar),
    created_at: Set(utc_now()),
    updated_at: Set(Some(utc_now())),
    ..Default::default()
  };
  state.repo.user().create_user(active_user).await?;
  Response::success(Some(()))
}

#[derive(Debug, Deserialize)]
struct GetTokenData {
  username: String,
  password: String,
}

#[post("/api/user/token")]
pub async fn get_token(
  state: Data<AppState>,
  body: Json<GetTokenData>,
) -> Result<HttpResponse, AppError> {
  let GetTokenData { username, password } = body.0;
  if let Some(user) = state.repo.user().get_user(username).await? {
    let matches = hash::verify_bcrypt(&password, &user.password)?;
    if matches {
      let token = jwt::sign(user.id, &state.jwt_key, 1000000)?;
      Response::success(Some(token))
    } else {
      Response::<()>::error(AppError::Error)
    }
  } else {
    Response::<()>::error(AppError::UserNotFound)
  }
}

#[derive(Debug, Deserialize)]
struct AddSiteData {
  name: String,
  domian: String,
}

#[post("/api/site")]
pub async fn add_site(
  req: HttpRequest,
  state: Data<AppState>,
  body: Json<AddSiteData>,
) -> Result<HttpResponse, AppError> {
  let AddSiteData { name, domian } = body.0;
  let repo = &state.repo;
  let user_id = jwt::verify(&extract_token(&req)?, &state.jwt_key)?
    .claims
    .data;
  if repo.website().has_website_by_name(&name).await? {
    return Response::<()>::error(AppError::WebsiteExist);
  }

  let active_website = website::ActiveModel {
    user_id: Set(user_id),
    name: Set(name),
    domain: Set(domian),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  repo.website().create_website(active_website).await?;
  Response::success(Some(()))
}

#[derive(Debug, Serialize, FromQueryResult)]
struct VisitorCount {
  visitor_count: u64,
}

#[get("/api/websites/{website_id}/stats")]
async fn get_stats(path: Path<u32>, state: Data<AppState>) -> Result<HttpResponse, AppError> {
  let website_id = path.into_inner();
  let (visitors, page_view) = state.repo.event().stats(website_id).await?;
  Response::success(Some(json!({ "visitors": visitors, "pageview": page_view })))
}

#[get("/api/websites/list")]
async fn get_websites_list(
  req: HttpRequest,
  state: Data<AppState>,
) -> Result<HttpResponse, AppError> {
  let token = extract_token(&req)?;
  let user_id = jwt::verify(&token, &state.jwt_key)?.claims.data;
  let websites_list = state.repo.website().get_websites(user_id).await?;
  Response::success(Some(websites_list))
}
