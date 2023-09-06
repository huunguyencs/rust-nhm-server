use bson::Document;
use mongodb::results::InsertOneResult;
use mongodb::error::Error;
use futures::TryStreamExt;

use crate::models::users::{User, PublicUser};

pub async fn create(user: User) -> Result<InsertOneResult, Error> {
  let collection = User::get_collection().await;
  let result = collection.insert_one(user, None).await?;
  Ok(result)
}

pub async fn find_one(query: Document) -> Result<Option<PublicUser>, Error> {
  let collection = User::get_collection().await;
  let result = collection.find_one(query, None).await?.map(PublicUser::from);
  Ok(result)
}

pub async fn query_user() -> Result<Vec<PublicUser>, Error> {
  let collection = User::get_collection().await;
  let users = match collection.find(None, None).await {
    Ok(cursor) => cursor,
    Err(e) => {
      println!("{:?}", e);
      return Ok(vec![]);
    }
  };
  let users: Vec<User> = users.try_collect().await.unwrap();
  let users = users.into_iter().map(Into::into).collect::<Vec<PublicUser>>();
  Ok(users)
}