use crate::rbx::error::Error;

pub fn handle_http_err<T>(code: u16) -> Result<T, Error> {
    match code {
        400 => Err(Error::HttpStatusError {
            code,
            msg: "invalid argument".to_string(),
        }),
        403 => Err(Error::HttpStatusError {
            code,
            msg: "permission denied".to_string(),
        }),
        404 => Err(Error::HttpStatusError {
            code,
            msg: "not found".to_string(),
        }),
        409 => Err(Error::HttpStatusError {
            code,
            msg: "aborted".to_string(),
        }),
        429 => Err(Error::HttpStatusError {
            code,
            msg: "resource exhausted".to_string(),
        }),
        499 => Err(Error::HttpStatusError {
            code,
            msg: "cancelled".to_string(),
        }),
        500 => Err(Error::HttpStatusError {
            code,
            msg: "internal server error".to_string(),
        }),
        501 => Err(Error::HttpStatusError {
            code,
            msg: "not implemented".to_string(),
        }),
        503 => Err(Error::HttpStatusError {
            code,
            msg: "unavailable".to_string(),
        }),
        _ => Err(Error::HttpStatusError {
            code,
            msg: "unknown error".to_string(),
        }),
    }
}
