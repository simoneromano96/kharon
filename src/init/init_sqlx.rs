use sqlx::{Pool, Postgres, Result, postgres::PgPoolOptions};

use crate::settings::APP_SETTINGS;

pub async fn init_sqlx() -> Result<Pool<Postgres>> {
  // Create a connection pool
  let pool: Pool<Postgres> = PgPoolOptions::new()
      // .max_connections(APP_CONFIG.database.poolsize)
      .connect(&APP_SETTINGS.database.uri)
      .await?;

  // info!("DB client initialised");

  sqlx::migrate!("src/sql/migrations")
      .run(&pool)
      .await?;

  Ok(pool)
}