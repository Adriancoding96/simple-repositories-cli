use std::fmt;
use std::io;
use reqwest;
use serde_json;

/*
* Combines std::io::Error, and reqwest::Error in to a single error type,
* this is needed as some operations can produce either one of the error types.
*/
#[derive(Debug)]
pub enum RequestError {
    Io(io::Error),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}
/*
* Implementation of display trait, formats the error for displaying
* it to the end user
*/
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Io(e) => write!(f, "IO error: {}", e),
            RequestError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            RequestError::Serde(e) => write!(f, "Serde json error: {}", e),
        }
    }
}

/*
* Implementation of standard error trait, making the request error
* compatible with rusts error handling utilities
*/
impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::Io(e) => Some(e),
            RequestError::Reqwest(e) => Some(e),
            RequestError::Serde(e) => Some(e),
        }
    }
}

/*
* Implemens automatic conversion of io error to custom error type
*/
impl From<io::Error> for RequestError {
    fn from(error: io::Error) -> Self {
        RequestError::Io(error)
    }
}

/*
* Implemens automatic conversion of reqwest error to custom error type
*/
impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        RequestError::Reqwest(error)
    }
}

/*
* Implemens automatic conversion of serde_json error to custom error type
*/
impl From<serde_json::Error> for RequestError {
    fn from(error: serde_json::Error) -> Self {
        RequestError::Serde(error)
    }
}
