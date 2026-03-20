//! JSON response envelope and process exit helpers.

use serde::Serialize;
use std::fmt;
use std::process;

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

    /// Builds a response from a `Result`, converting the error via `Display`.
    pub fn from_result<E: fmt::Display>(result: Result<T, E>) -> Self {
        match result {
            Ok(data) => Self::ok(data),
            Err(e) => Self::err(e.to_string()),
        }
    }

    /// Prints pretty JSON to stdout and exits with 0 (success) or 1 (error).
    pub fn print_and_exit(self) -> ! {
        let exit_code = i32::from(!self.success);
        match serde_json::to_string_pretty(&self) {
            Ok(json) => println!("{json}"),
            Err(serialize_err) => eprintln!("failed to serialize response: {serialize_err}"),
        }
        process::exit(exit_code)
    }
}

/// Converts a `Result` into a `Response`, prints JSON, and exits.
pub fn exit_with<T: Serialize>(result: Result<T, impl fmt::Display>) -> ! {
    Response::from_result(result).print_and_exit()
}
