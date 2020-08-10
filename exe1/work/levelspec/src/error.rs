use thiserror::Error;

#[derive(Error, Debug)]
pub enum LevelSpecError {
    #[error("{0}")]
    InvalidInputError(String),
    #[error("{0}")]
    InvalidCharactersInInput(String),
}
