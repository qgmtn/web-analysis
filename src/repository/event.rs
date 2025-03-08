use entity::event;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
  QueryFilter, QuerySelect,
};

#[derive(Debug, Clone)]
pub struct EventRepository<'a> {
  pub db: &'a DatabaseConnection,
}

pub enum EventQueryBy<'a> {
  Id(u32),
  Email(&'a str),
}

impl<'a> EventRepository<'a> {
  async fn has_event(&self, query_by: EventQueryBy<'a>) -> Result<bool, DbErr> {
    let mut select = event::Entity::find();
    match query_by {
      EventQueryBy::Id(id) => select = select.filter(event::Column::Id.eq(id)),
      EventQueryBy::Email(email) => select = select.filter(event::Column::VisitorId.eq(email)),
    }
    let res = select.one(self.db).await?;
    Ok(res.is_some())
  }

  pub async fn has_event_by_id(&self, id: u32) -> Result<bool, DbErr> {
    self.has_event(EventQueryBy::Id(id)).await
  }

  pub async fn has_event_by_email(&self, email: &str) -> Result<bool, DbErr> {
    self.has_event(EventQueryBy::Email(email)).await
  }

  pub async fn create_event(
    &self,
    active_event: event::ActiveModel,
  ) -> Result<event::Model, DbErr> {
    active_event.insert(self.db).await
  }

  pub async fn stats(&self, website_id: u32) -> Result<(u64, u64), DbErr> {
    let visitors = event::Entity::find()
      .select_only()
      .filter(event::Column::WebsiteId.eq(website_id))
      .column(event::Column::VisitorId)
      .distinct()
      .count(self.db)
      .await?;
    let page_view = event::Entity::find()
      .filter(event::Column::WebsiteId.eq(website_id))
      .count(self.db)
      .await?;
    Ok((visitors, page_view))
  }
}
