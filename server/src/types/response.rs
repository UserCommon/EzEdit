use super::error::Error;
use super::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub result: Option<Value>,
    pub error: Option<Error>,
}
