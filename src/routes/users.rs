use axum::{Router, routing::{get, post}};

use crate::controllers::users::{create_user, get_by_id, get_all};

pub fn create_route() -> Router {
  Router::new()
    .route("/", post(create_user))
    .route("/", get(get_all))
    .route("/:id", get(get_by_id))
}