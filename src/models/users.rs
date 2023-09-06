use mongodb::{bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};
use validator::Validate;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use bson::serde_helpers::serialize_object_id_as_hex_string;

use crate::{utils::date::{Date, self}, database::DATABASE};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub name: String,
  #[validate(email)]
  pub email: String,
  pub password: String,
  pub created_at: Date,
  pub updated_at: Date
}

impl User {
  pub fn new<A, B, C>(name: A, email: B, password: C) -> Self 
  where
    A: Into<String>,
    B: Into<String>,
    C: Into<String>,
  {
    let now = date::now();
    Self {
      id: None,
      name: name.into(),
      email: email.into(),
      password: password.into(),
      created_at: now,
      updated_at: now
    }
  }

  pub async fn get_collection() -> Collection<Self> {
    let database = DATABASE.get().await;
    let collection = database.collection::<User>("users");

    collection
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUser {
  #[serde(alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
  pub id: ObjectId,
  pub name: String,
  pub email: String,
  #[serde(with = "bson_datetime_as_rfc3339_string")]
  pub created_at: Date,
  #[serde(with = "bson_datetime_as_rfc3339_string")]
  pub updated_at: Date
}

impl From<User> for PublicUser {
  fn from(user: User) -> Self {
    Self {
      id: user.id.unwrap(),
      name: user.name,
      email: user.email,
      created_at: user.created_at,
      updated_at: user.updated_at
    }
  }
}