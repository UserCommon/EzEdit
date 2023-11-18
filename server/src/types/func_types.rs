use super::params::Params;
use super::response::Response;
use super::Value;

pub enum FuncTypes {
    MutingFunction(Box<dyn FnMut(Params) -> Value>),
    ImmutingFunction(Box<dyn Fn(Params) -> Value>),
}

impl FuncTypes {
    pub fn is_muting(&self) -> bool {
        match self {
            MutingFunction => true,
            ImmutingFunction => false,
        }
    }

    pub fn is_immuting(&self) -> bool {
        match self {
            MutingFunction => false,
            ImmutingFunction => true,
        }
    }
}
