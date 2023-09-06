use std::net::SocketAddr;

use crate::database::DB;

mod app;
mod database;
mod utils;
mod routes;
mod dao;
mod controllers;
mod models;
mod errors;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let address = SocketAddr::from(([127, 0, 0, 1], PORT));
    
    println!("Server is running on {}", address);

    let db = DB::init().await;

    axum::Server::bind(&address).serve(app.into_make_service()).await.expect("Can not start server!");
}