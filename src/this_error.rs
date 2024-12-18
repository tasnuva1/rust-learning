use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error), // source of the error only
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, .limits.hi)]
    OutOfBounds { idx: usize, limits: Limits },
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Debug)]
struct Limits {
    lo: i8,
    hi: String,
}

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

// #[derive(Error, Debug)]
// pub struct MyError {
//     msg: String,
//     #[source] // #[source] attribute or can be named source is same as #[from] which can't have a filed.
//     source: anyhow::Error,
// }

fn make_request() -> Result<(), DataStoreError> {
    let key = std::fs::read_to_string("some-key-file")?;
    std::fs::remove_file(key)?;
    Ok(())
}

// How this crate is created: https://betterprogramming.pub/a-simple-guide-to-using-thiserror-crate-in-rust-eee6e442409b
