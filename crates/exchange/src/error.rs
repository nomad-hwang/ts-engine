#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
