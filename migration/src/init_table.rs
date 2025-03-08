use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(pk_auto(User::Id).unsigned())
          .col(string(User::Username))
          .col(string(User::Password))
          .col(string(User::Email))
          .col(string(User::DisplayName))
          .col(string(User::Avatar))
          .col(string(User::Role))
          .col(timestamp(User::CreatedAt).default(Expr::current_timestamp()))
          .col(timestamp(User::UpdatedAt).null())
          .to_owned(),
      )
      .await?;
    manager
      .create_table(
        Table::create()
          .table(Website::Table)
          .if_not_exists()
          .col(pk_auto(Website::Id).unsigned())
          .col(unsigned(Website::UserId))
          .col(string(Website::Name))
          .col(string(Website::Domain))
          .col(timestamp(Website::CreatedAt).default(Expr::current_timestamp()))
          .to_owned(),
      )
      .await?;
    manager
      .create_table(
        Table::create()
          .table(Event::Table)
          .if_not_exists()
          .col(pk_auto(Event::Id).unsigned())
          .col(unsigned(Event::WebsiteId))
          .col(string(Event::VisitorId))
          .col(string(Event::Type))
          .col(string(Event::Tag))
          .col(string(Event::UrlPath))
          .col(string(Event::UrlQuery))
          .col(string(Event::PageTitle))
          .col(timestamp(Event::CreatedAt).default(Expr::current_timestamp()))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Website::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum User {
  Table,
  Id,
  Username,
  Password,
  Email,
  DisplayName,
  Avatar,
  Role,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum Website {
  Table,
  Id,
  UserId,
  Name,
  Domain,
  CreatedAt,
}

#[derive(DeriveIden)]
enum Event {
  Table,
  Id,
  WebsiteId,
  VisitorId,
  Type,
  Tag,
  UrlPath,
  UrlQuery,
  PageTitle,
  CreatedAt,
}

#[derive(DeriveIden)]
enum Session {
  Table,
  Id,
  WebsiteId,
  Hostname,
  Browser,
  Os,
  Device,
  Screen,
  Language,
  Country,
  City,
  CreatedAt,
}
