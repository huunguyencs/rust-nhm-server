
use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database};
use lazy_static::lazy_static;
use dotenv::dotenv;
use std::env;

lazy_static! {
    static ref DATABASE: Database = {
        dotenv().ok();
        let client_uri = env::var("DATABASE_URL").expect("Please set DATABASE_URL in your environment file.");
        let options = tokio::runtime::Runtime::new().unwrap().block_on(async {
          let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await;
          options
        }).unwrap();
        let client = Client::with_options(options).unwrap();
        client.database("nihaoma")
    };
}

pub fn get_database() -> &'static Database {
    &DATABASE
}
