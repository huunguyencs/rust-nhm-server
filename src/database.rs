
use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database};
use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE: Database = {
        let client_uri = "mongodb+srv://huunguyen:T254nouuOj78wQLo@cluster0.l1ldu.mongodb.net/?retryWrites=true&w=majority";
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
