use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

#[derive(Debug, Clone)]
pub struct UserRepository<'a> {
  pub db: &'a DatabaseConnection,
}

pub enum UserQueryBy<'a> {
  Id(u32),
  Email(&'a str),
}

impl<'a> UserRepository<'a> {
  async fn has_user(&self, query_by: UserQueryBy<'a>) -> Result<bool, DbErr> {
    let mut select = user::Entity::find();
    match query_by {
      UserQueryBy::Id(id) => select = select.filter(user::Column::Id.eq(id)),
      UserQueryBy::Email(email) => select = select.filter(user::Column::Email.eq(email)),
    }
    let res = select.one(self.db).await?;
    Ok(res.is_some())
  }

  pub async fn has_user_by_id(&self, id: u32) -> Result<bool, DbErr> {
    self.has_user(UserQueryBy::Id(id)).await
  }

  pub async fn has_user_by_email(&self, email: &str) -> Result<bool, DbErr> {
    self.has_user(UserQueryBy::Email(email)).await
  }

  pub async fn is_first_user(&self) -> Result<bool, DbErr> {
    let users = user::Entity::find().all(self.db).await?;
    Ok(users.is_empty())
  }

  pub async fn create_user(&self, active_user: user::ActiveModel) -> Result<user::Model, DbErr> {
    active_user.insert(self.db).await
  }

  pub async fn get_user(&self, username: String) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
      .filter(user::Column::Username.eq(username))
      .one(self.db)
      .await
  }
}
