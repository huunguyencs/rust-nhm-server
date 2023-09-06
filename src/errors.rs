use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("MongoDB error")]
  MongoError(#[from] mongodb::error::Error)
  
}

impl Into<StatusCode> for Error {
  fn into(self) -> StatusCode {
    let status = match self {
      Error::MongoError(_) => StatusCode::INTERNAL_SERVER_ERROR
    };

    status
  }
}