use thiserror::Error;

#[derive(Error, Debug)]
pub enum UniformError {
    #[error("the `{0}` is not found")]
    NotFound(String),
    #[error("the `{0}` is duplicate")]
    Duplicate(String),

    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("serialize data error")]
    SerializeErr,
    #[error("deserialize data error")]
    DeserializeErr,
    #[error("unknown data store error")]
    Unknown,
    // todo add more error case
}

pub type OwnResult<T, E = UniformError> = Result<T, E>;
