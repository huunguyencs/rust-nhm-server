use axum::{
  extract::Path, Json
};
use bson::doc;

use crate::{models::users::User, utils::to_object_id::to_object_id};
use crate::dao::users::{find_one, query_user, };

pub async fn create_user() -> &'static str {
  "OK"
}

pub async fn get_by_id(Path(id): Path<String>) -> Result<Json<User>, String> {
  let user_id = to_object_id(id).expect("Id is not correct");
  let user = find_one(doc! {"_id": user_id}).await.expect("Some thing went wrong");
  let user = match user {
    Some(user) => user,
    None => {
      println!("Can not found this user");
      return Err(String::from("Not found"));
    }
  };

  Ok(Json(user))
}

pub async fn get_all() -> Result<Json<Vec<User>>, String> {
  let users = query_user().await.expect("Some thing went wrong");

  Ok(Json(users))
}