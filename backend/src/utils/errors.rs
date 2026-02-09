pub type UtilsResult<T> = Result<T, UtilsError>;


#[derive(Debug, thiserror::Error)]
pub enum UtilsError {
    // b64u
    #[error("Failed to decode b64-url encoded string")]
    FailToB64uDecode,

    // Time
    #[error("Time is not RFC3339")]
    ParsingError,
} 