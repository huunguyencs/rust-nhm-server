use async_once::AsyncOnce;
use dotenv::dotenv;
use std::env;
use mongodb::{Client, Database};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(async {
    dotenv().ok();

    let database_uri = env::var("DATABASE_URL").expect("Please set DATABASE_URL variable in your env file.");
    let database_name = env::var("DATABASE_NAME").expect("Please set DATABASE_NAME variable in your env file.");

 
    Client::with_uri_str(database_uri)
      .await
      .expect("Failed to initialize MongoDB connection")
      .database(database_name.as_str())
  });
}