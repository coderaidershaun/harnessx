//! JSON response envelope and process exit helpers.

use serde::Serialize;
use std::process;

use crate::errors::ParserError;

/// JSON envelope wrapping every CLI response; `success` is set by the constructor used.
#[derive(Serialize)]
pub struct Response<T: Serialize> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }

    /// Prints pretty JSON to stdout and exits with the given code.
    pub fn print_and_exit(self, exit_code: i32) -> ! {
        match serde_json::to_string_pretty(&self) {
            Ok(json) => println!("{json}"),
            Err(serialize_err) => eprintln!("failed to serialize response: {serialize_err}"),
        }
        process::exit(exit_code)
    }
}

/// Converts a `Result` into a `Response`, prints JSON, and exits.
///
/// Expected-state errors (like "no active project") exit with code 0
/// so the calling skill doesn't see a shell error for normal conditions.
/// Genuine failures exit with code 1.
pub fn exit_with<T: Serialize>(result: Result<T, ParserError>) -> ! {
    match result {
        Ok(data) => Response::ok(data).print_and_exit(0),
        Err(e) => {
            let exit_code = if e.is_expected_state() { 0 } else { 1 };
            Response::<T>::err(e.to_string()).print_and_exit(exit_code)
        }
    }
}
