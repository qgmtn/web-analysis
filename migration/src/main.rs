pub use sea_orm_migration::prelude::*;

mod init_table;

#[async_std::main]
async fn main() {
  cli::run_cli(migration::Migrator).await;
}
