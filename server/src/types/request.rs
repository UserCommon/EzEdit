use super::params::Params;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub method: String,
    pub params: Option<Params>,
}
