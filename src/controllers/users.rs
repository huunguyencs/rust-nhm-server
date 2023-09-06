use axum::{
  extract::Path, Json
};
use mongodb::results::InsertOneResult;
use serde::Deserialize;
use bson::doc;

use crate::models::users::PublicUser;
use crate::{models::users::User, utils::to_object_id::to_object_id};
use crate::dao::users::{find_one, query_user, create};

pub async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<InsertOneResult>, String> {
  let user = User::new(payload.name, payload.email, payload.password);
  let user = create(user).await.expect("Can not create user");

  Ok(Json(user))
}

pub async fn get_by_id(Path(id): Path<String>) -> Result<Json<PublicUser>, String> {
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

pub async fn get_all() -> Result<Json<Vec<PublicUser>>, String> {
  let users = query_user().await.expect("Some thing went wrong");

  Ok(Json(users))
}

#[derive(Deserialize)]
pub struct CreateUser {
  name: String,
  email: String,
  password: String
}