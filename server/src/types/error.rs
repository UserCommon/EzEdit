use super::Value;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ErrorKind {
    MethodNotFound,
    InvalidRequest,
    RemoteDisconnect,
    InvalidParams,
    ParseError,
    ServerShutDown,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub data: Option<Value>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            data: None,
        }
    }

    pub fn with_data(kind: ErrorKind, message: String, data: Option<Value>) -> Self {
        Self {
            kind,
            message,
            data,
        }
    }

    pub fn method_not_found() -> Self {
        Self::new(ErrorKind::MethodNotFound, "Method not found!".to_string())
    }

    pub fn invalid_request() -> Self {
        Self::new(ErrorKind::InvalidRequest, "Invalid Request!".to_string())
    }
}
