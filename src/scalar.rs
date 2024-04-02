use crate::scalar::elem::Elem;
use crate::scalar::frac::Frac;

mod var;
mod elem;
mod frac;

pub enum Scalar {
    Elem(Elem),
    Frac(Frac),
    Plus(Box<Scalar>, Box<Scalar>),
    Minus(Box<Scalar>, Box<Scalar>),
    Times(Box<Scalar>, Box<Scalar>),
    By(Box<Scalar>, Box<Scalar>),
}

impl From<u64> for Scalar {
    fn from(value: u64) -> Self {
        Scalar::Frac(Frac::from(value))
    }
}

impl From<char> for Scalar {
    fn from(symbol: char) -> Self {

    }
}