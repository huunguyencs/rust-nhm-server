use dotenv::dotenv;
use std::env;
use mongodb::{options::ClientOptions, Client, Database};

use crate::errors::Error;

pub struct DB {
  pub database: Database
}

type Result<T> = std::result::Result<T, Error>;

impl DB {
  pub async fn init() -> Result<Self> {
    dotenv().ok();

    let database_uri = env::var("DATABASE_URL").expect("Please set DATABASE_URL variable in your env file.");
    let database_name = env::var("DATABASE_NAME").expect("Please set DATABASE_NAME variable in your env file.");

    let mut client_options = ClientOptions::parse(database_uri).await?;
    client_options.app_name = Some(database_name.to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database(database_name.as_str());

    Ok(Self {
      database
    })
  }
}