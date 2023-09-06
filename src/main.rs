use std::net::SocketAddr;
use dotenv::dotenv;
use std::env;

mod app;
mod database;
mod utils;
mod routes;
mod dao;
mod controllers;
mod models;
mod errors;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("3000".to_string()).parse::<u16>().unwrap();
    let app = app::create_app().await;

    let address = SocketAddr::from(([127, 0, 0, 1], port));
    
    println!("Server is running on {}", address);

    axum::Server::bind(&address).serve(app.into_make_service()).await.expect("Can not start server!");
}