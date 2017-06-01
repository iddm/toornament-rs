#![warn(missing_docs)]
use std::io::Error as IoError;
use std::error::Error as StdError;
use std::fmt::Display;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use chrono::format::ParseError;

/// Toornament API `Result` alias type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Toornament API error type.
#[derive(Debug)]
pub enum Error {
    /// A `reqwest` crate error
    Reqwest(ReqwestError),
    /// A `serde_json` crate error
    Json(JsonError),
    /// A `std::io` module error
    Io(IoError),
    /// A date parse error (`chrono` crate error)
    Date(ParseError),
    /// A generic non-success response from the REST API
    Status(::reqwest::StatusCode),
    /// A rate limit error, with how many milliseconds to wait before retrying
    RateLimited(u64),
    /// A Toornament protocol error, with a description
    Protocol(&'static str),
    /// A command execution failure, with a command name and output
    Command(&'static str, ::std::process::Output),
    /// A miscellaneous error, with a description
    Other(&'static str),
}

impl Error {
    #[doc(hidden)]
    pub fn from_response(response: ::reqwest::Response) -> Error {
        #[derive(Deserialize)]
        struct TooManyRequests {
            retry_after: u64,
        }

        let status = response.status().clone();
        if let Ok(value) = ::serde_json::from_reader::<_, TooManyRequests>(response) {
            if status == ::reqwest::StatusCode::TooManyRequests {
                return Error::RateLimited(value.retry_after)
            }
        }
        Error::Status(status)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Error::Reqwest(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::Date(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::Reqwest(ref inner) => inner.fmt(f),
            Error::Json(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            Error::Date(ref inner) => inner.fmt(f),
            Error::Command(cmd, _) => write!(f, "Command failed: {}", cmd),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Reqwest(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            Error::Date(ref inner) => inner.description(),
            Error::Protocol(msg) |
            Error::Other(msg) => msg,
            Error::Status(status) => status.canonical_reason().unwrap_or("Unknown bad HTTP status"),
            Error::RateLimited(_) => "Rate limited",
            Error::Command(_, _) => "Command failed",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Reqwest(ref inner) => Some(inner),
            Error::Json(ref inner) => Some(inner),
            Error::Io(ref inner) => Some(inner),
            Error::Date(ref inner) => Some(inner),
            _ => None,
        }
    }
}
