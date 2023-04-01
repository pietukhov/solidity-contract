//! Tare down an existing database and then re-initialize it.
use common::*;

mod common;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = init().await?;

    Migrator::down(&db, None).await?;
    Migrator::up(&db, None).await?;

    Ok(())
}
