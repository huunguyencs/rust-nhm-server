use mongodb::{bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{utils::date::{Date, self}, database::get_database};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
  #[serde(rename = "_id")]
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

  pub fn get_collection() -> Collection<Self> {
    get_database().collection::<Self>("users")
  }
}