use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Var {
    name: Cow<'static, str>
}

impl From<String> for Var {
    fn from(name: String) -> Self {
        Var { name: name.into() }
    }
}

impl From<&'static str> for Var {
    fn from(name: &'static str) -> Self {
        Var { name: name.into() }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}