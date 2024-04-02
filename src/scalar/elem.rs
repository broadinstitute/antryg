use std::fmt::{Display, Formatter};
use crate::scalar::Scalar;
use crate::scalar::var::Var;

#[derive(Clone)]
pub enum Elem {
    Var(Var),
    Indexed(Box<Elem>, Box<Scalar>)
}

impl From<char> for Elem {
    fn from(symbol: char) -> Self {
        Elem::Var(Var::from(symbol))
    }
}

impl Display for Elem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Var(var) => { var.fmt(f) }
            Elem::Indexed(elem, index) => {
                write!(f, "{elem}[{index}]")
            }
        }
    }
}