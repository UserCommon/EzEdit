use super::params::Params;
use super::response::Response;

pub enum FuncTypes {
    MutingFunction(Box<dyn FnMut(Params) -> Response>),
    ImmutingFunction(Box<dyn Fn(Params) -> Response>),
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
