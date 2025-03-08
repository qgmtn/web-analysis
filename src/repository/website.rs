use entity::website;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

#[derive(Debug, Clone)]
pub struct WebsiteRepository<'a> {
  pub db: &'a DatabaseConnection,
}

pub enum WebsiteQueryBy {
  Id(u32),
  Name(String),
}

impl<'a> WebsiteRepository<'a> {
  async fn has_website(&self, query_by: WebsiteQueryBy) -> Result<bool, DbErr> {
    let mut select = website::Entity::find();
    match query_by {
      WebsiteQueryBy::Id(id) => select = select.filter(website::Column::Id.eq(id)),
      WebsiteQueryBy::Name(name) => select = select.filter(website::Column::Name.eq(name)),
    }
    let res = select.one(self.db).await?;
    Ok(res.is_some())
  }

  pub async fn has_website_by_id(&self, id: u32) -> Result<bool, DbErr> {
    self.has_website(WebsiteQueryBy::Id(id)).await
  }

  pub async fn has_website_by_name(&self, name: &str) -> Result<bool, DbErr> {
    self
      .has_website(WebsiteQueryBy::Name(name.to_string()))
      .await
  }

  pub async fn create_website(
    &self,
    active_website: website::ActiveModel,
  ) -> Result<website::Model, DbErr> {
    active_website.insert(self.db).await
  }

  pub async fn get_websites(&self, user_id: u32) -> Result<Vec<website::Model>, DbErr> {
    website::Entity::find()
      .filter(website::Column::UserId.eq(user_id))
      .all(self.db)
      .await
  }
}
