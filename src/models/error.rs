#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Something went wrong, contact the developer")]
    InternalError,

    #[error(transparent)]
    Generic(#[from] anyhow::Error)
}

impl From<rusqlite::Error> for DbError {
   fn from(value: rusqlite::Error) -> Self {
       match value {
           e => DbError::Generic(e.into())
       }
   } 
}
