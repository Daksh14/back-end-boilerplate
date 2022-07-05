#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use bb8_diesel::{bb8, DieselConnectionManager};
use config::DATABASE_URL;
use diesel::pg::PgConnection;

pub use context::Context;

mod config;
mod context;

pub mod models;
pub mod schema;

pub type Pool = bb8::Pool<DieselConnectionManager<PgConnection>>;
pub type PooledConnection<'a> = bb8::PooledConnection<'a, DieselConnectionManager<PgConnection>>;

diesel_migrations::embed_migrations!();

/// Connect to the database and run any migrations. This will
/// log the migration ouput to stdout.
pub async fn connect() -> Result<Pool> {
    let manager = DieselConnectionManager::new(*DATABASE_URL);
    let pool = bb8::Pool::builder().max_size(16).build(manager).await?;

    {
        // Run migrations
        let conn = pool.get().await?;
        embedded_migrations::run_with_output(&*conn, &mut std::io::stdout())?;
    }

    Ok(pool)
}
