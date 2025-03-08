mod analytics;
mod assets;
mod config;
mod error;
mod handler;
mod helper;
mod locales;
mod migration;
mod repository;
mod response;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web::Data};
use config::EnvConfig;
use error::AppError;
use migration::migrate;
use repository::RepositoryManager;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone)]
pub struct AppState {
  pub repo: RepositoryManager,
  pub jwt_key: String,
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
  let target_filter = filter::Targets::new()
    .with_default(LevelFilter::TRACE)
    .with_target("sqlx::query", LevelFilter::DEBUG)
    .with_target("sqlx::query", LevelFilter::OFF)
    .with_target("rustls", LevelFilter::OFF);
  let env_filter = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("info"))
    .unwrap();
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_timer(fmt::time::LocalTime::rfc_3339()))
    .with(target_filter)
    .with(env_filter)
    .init();
  let EnvConfig {
    workers,
    host,
    port,
    database_url,
    jwt_key,
    ..
  } = EnvConfig::load_env()?;
  let db = migrate(&database_url).await?;
  let state = AppState {
    jwt_key,
    repo: RepositoryManager::new(db),
  };
  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(state.clone()))
      .wrap(middleware::Logger::default())
      .wrap(Cors::permissive())
      .service(handler::dist)
      .service(handler::index)
      .service(handler::track_data)
      .service(handler::add_site)
      .service(handler::add_user)
      .service(handler::get_stats)
      .service(handler::get_token)
      .service(handler::get_websites_list)
  })
  .bind((host, port))?
  .workers(workers)
  .run()
  .await?;
  Ok(())
}
