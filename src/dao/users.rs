use bson::Document;
// use mongodb::results::InsertOneResult;
use mongodb::error::Error;
use futures::TryStreamExt;

use crate::models::users::User;

// pub async fn create_user(user: User) -> Result<InsertOneResult, Error> {
//   let collection = User::get_collection();
//   let result = collection.insert_one(user, None).await?;
//   Ok(result)
// }

pub async fn find_one(query: Document) -> Result<Option<User>, Error> {
  let collection = User::get_collection().await;
  let result = collection.find_one(query, None).await?;
  Ok(result)
}

pub async fn query_user() -> Result<Vec<User>, Error> {
  let collection = User::get_collection().await;
  let users = match collection.find(None, None).await {
    Ok(cursor) => cursor,
    Err(e) => {
      println!("{:?}", e);
      return Ok(vec![]);
    }
  };
  let users: Vec<User> = users.try_collect().await.unwrap();
  Ok(users)
}