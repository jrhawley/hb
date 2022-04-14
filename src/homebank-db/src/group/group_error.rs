use thiserror::Error;

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("Invalid group key.")]
    InvalidKey,
}
