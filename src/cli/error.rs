use std::io;

use crate::{models::error::DbError, util::error::UtilError};

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Cannot interact with user!")]
    CannotInteract,

    #[error("Cannot read user input!")]
    CannotReadUserInput,

    #[error("Invalid arguments")]
    InvalidArguments,

    #[error("Invalid digit")]
    InvalidDigit,

    #[error("Invalid utf-8 found in note!")]
    InvalidUtf8,

    #[error("Cannot open file: {0}")]
    CannotOpenFile(String),

    #[error("Note with provided id not found, or id not provided")]
    NoteNotFound,

    #[error("Provided reference must have either title or id")]
    InvalidReference,

    #[error("Reference with provided title: {0} does not exist.")]
    ReferenceDoesNotExist(String),

    #[error("Internal error - contact the developer")]
    InternalError,

    #[error("Object with provided id doesn't exist!")]
    ObjectNotFound,

    #[error("The title of note cannot be empty!")]
    NoteTitleEmpty,

    #[error(transparent)]
    Generic(#[from] anyhow::Error)
}

#[derive(Debug)]
pub enum Arguments {
    Path
}

impl From<DbError> for CliError {
    fn from(value: DbError) -> Self {
        match value {
            e => CliError::Generic(e.into())
        }
    }
}

impl From<UtilError> for CliError {
    fn from(value: UtilError) -> Self {
        match value {
            e => CliError::Generic(e.into())
        }
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        match value {
            e => CliError::Generic(e.into())
        }
    }
}
