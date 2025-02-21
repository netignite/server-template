#[cfg(feature = "deserialize")]
use serde::Deserialize;

#[cfg(feature = "serialize")]
use serde::Serialize;

#[cfg(feature = "type_generation")]
use ts_rs::TS;

#[cfg_attr(feature = "type_generation", derive(TS))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[allow(dead_code)]
pub struct Response<T> {
    pub(crate) data: T,
}

#[cfg_attr(feature = "type_generation", derive(TS))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[allow(dead_code)]
pub struct ErrorResponse<T> {
    pub(crate) message: String,
    pub(crate) code: Option<u16>,
    pub(crate) data: Option<T>,
}

#[cfg_attr(feature = "type_generation", derive(TS))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(tag = "type", rename_all = "lowercase")
)]
pub enum ResponseType<T> {
    Success(Response<T>),
    Fail(Response<T>),
    Error(ErrorResponse<T>),
}

impl<T> ResponseType<T> {
    pub fn new_success(data: T) -> Self {
        ResponseType::Success(Response { data })
    }

    pub fn new_fail(data: T) -> Self {
        ResponseType::Fail(Response { data })
    }

    pub fn new_error(message: String) -> Self {
        ResponseType::Error(ErrorResponse {
            message,
            code: None,
            data: None,
        })
    }

    pub fn new_error_with_code(message: String, code: u16) -> Self {
        ResponseType::Error(ErrorResponse {
            message,
            code: Some(code),
            data: None,
        })
    }

    pub fn new_error_with_data(message: String, data: T) -> Self {
        ResponseType::Error(ErrorResponse {
            message,
            code: None,
            data: Some(data),
        })
    }

    pub fn new_error_with_code_and_data(message: String, code: u16, data: T) -> Self {
        ResponseType::Error(ErrorResponse {
            message,
            code: Some(code),
            data: Some(data),
        })
    }
}

#[cfg(feature = "type_generation")]
pub(crate) fn get_typescript_definitions() -> Vec<String> {
    vec![
        Response::<()>::export_to_string().expect("Failed to export Response"),
        ErrorResponse::<()>::export_to_string().expect("Failed to export ErrorResponse"),
        ResponseType::<()>::export_to_string().expect("Failed to export ResponseType"),
    ]
}
