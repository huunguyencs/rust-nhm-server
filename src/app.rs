use axum::{Router, routing::get};

use crate::routes;

pub async fn create_app() -> Router {
  Router::new()
    .merge(Router::new().nest(
      "/api/v1", 
      Router::new()
        .route("/hello", get(|| async {"Hello world"}))
        .merge(Router::new().nest("/users", routes::users::create_route()))
    ))
}