#[derive(thiserror::Error, Debug)]
pub enum UtilError {
    #[error("Error: Invalid note markdown!")]
    InvalidNoteMarkdown,

    #[error(transparent)]
    Generic(#[from] anyhow::Error)
}
