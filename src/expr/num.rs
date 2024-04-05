use std::fmt::{Display, Formatter};
use crate::expr::{Expr, Precedence};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Num {
    value: u64,
}

impl From<u64> for Num {
    fn from(value: u64) -> Self {
        Num { value }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
