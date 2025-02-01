mod config;
mod db;
mod model;
use anyhow::{Error, Result};
use config::Config;
use db::mongo_db_handler::{self, MongoDbHandler};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    tracing_subscriber::fmt().json().init();

    let config = Config::new(Some(".env"))?;

    let db_handler = MongoDbHandler::new(
        &config.db_user_name,
        &config.db_user_password,
        &config.db_name,
        &config.db_host,
    )
    .await?;

    Ok(())
}
