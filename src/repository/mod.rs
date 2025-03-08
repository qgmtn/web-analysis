mod event;
mod user;
mod website;

use event::EventRepository;
use sea_orm::DatabaseConnection;

pub use user::UserRepository;
pub use website::WebsiteRepository;

#[derive(Debug, Clone)]
pub struct RepositoryManager {
  pub db: DatabaseConnection,
}

impl RepositoryManager {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db }
  }

  pub fn user(&self) -> UserRepository {
    UserRepository { db: &self.db }
  }

  pub fn website(&self) -> WebsiteRepository {
    WebsiteRepository { db: &self.db }
  }

  pub fn event(&self) -> EventRepository {
    EventRepository { db: &self.db }
  }
}
