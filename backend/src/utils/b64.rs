use crate::utils::errors::{UtilsError, UtilsResult};

pub fn b64_encode(content: &str) -> String {
    base64_url::encode(content)
}

pub fn b64_decode(b64u: &str) -> UtilsResult<String> {
    let decoded_string = base64_url::decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(UtilsError::FailToB64uDecode)?;

    Ok(decoded_string)
}