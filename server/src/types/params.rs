use super::{Map, Value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Params {
    None,
    Positional(Vec<Value>),
    Named(Map<String, Value>),
}
