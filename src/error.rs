use std::io::Error as IoError;
use std::error::Error as StdError;
use std::fmt::Display;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use chrono::format::ParseError;

/// Toornament API `Result` alias type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// A toornament service error type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToornamentErrorType {
    /// Duplicate email error type
    EmailDuplicate,
    /// Match integrity error type
    MatchIntegrity,
}

/// A toornament service error scope
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToornamentErrorScope {
    /// The error scope is the query
    Query,
    /// The error scope is the body
    Body,
}


/// A list of toornament service errors
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ToornamentError {
    /// Error message.
    pub message: String,
    /// The scope refers to an error in a part of the HTTP request. It can be located in the query
    /// string or in the message body data.
    pub scope: ToornamentErrorScope,
    /// Path of the error from your data which caused the error.
    pub property_path: Option<String>,
    /// This property is only available when the property "property_path" is itself available.
    /// Identify the incorrect value causing the error.
    pub invalid_value: Option<String>,
    /// Some data cannot be pre-validated by a client i.e. duplicate email participant. You can
    /// get the possible list for each endpoint.
    #[serde(rename = "type")]
    pub error_type: Option<ToornamentErrorType>,
}

/// A list of toornament service errors
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ToornamentErrors(pub Vec<ToornamentError>);

/// Toornament service error
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ToornamentServiceError {
    /// A list of toornament service errors
    pub errors: ToornamentErrors,
}

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
    /// A error common toornament service error
    Toornament(::reqwest::StatusCode, ToornamentServiceError),
    /// A generic non-success response from the REST API
    Status(::reqwest::StatusCode, String),
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
    pub fn from_response(mut response: ::reqwest::Response) -> Error {
        use std::io::Read;

        #[derive(Deserialize)]
        struct TooManyRequests {
            retry_after: u64,
        }

        let status = response.status().clone();
        let mut body = String::new();
        let _ = response.read_to_string(&mut body);
        if status == ::reqwest::StatusCode::TooManyRequests {
            if let Ok(value) = ::serde_json::from_str::<TooManyRequests>(&body) {
                return Error::RateLimited(value.retry_after)
            }
        } else if !status.is_success() {
            if let Ok(e) = ::serde_json::from_str::<ToornamentServiceError>(&body) {
                return Error::Toornament(status, e)
            }
        }
        Error::Status(status, body)
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
            Error::Toornament(status, _) |
            Error::Status(status, _) => status.canonical_reason().unwrap_or("Unknown bad HTTP status"),
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
