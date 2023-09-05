use axum::{Router, routing::{get, post, put, delete}};

use crate::controllers::users::Controller;

pub fn create_route() -> Router {
  Router::new()
    .route("/", post(Controller::create_user()))
}