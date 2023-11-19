use super::params::Params;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub method: String,
    pub params: Option<Params>,
}

impl Request {
    pub fn new(method: String, params: Option<Params>) -> Self {
        Self { method, params }
    }
}
