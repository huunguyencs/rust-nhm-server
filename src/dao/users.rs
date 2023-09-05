use mongodb::results::InsertOneResult;
use mongodb::error::Error;

use crate::models::users::User;

async fn create_user(user: User) -> Result<InsertOneResult, Error> {
    let collection = User::get_collection();
    let result = collection.insert_one(user, None).await?;
    Ok(result)
  }