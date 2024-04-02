use std::fmt::{Display, Formatter, Pointer};
use std::ops::{Add, Div, Mul, Sub};
use crate::scalar::elem::Elem;

mod var;
mod elem;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum InfixOp {
    Plus,
    Minus,
    Times,
    Over,
}

#[derive(Clone)]
pub struct InfixTerm {
    op: InfixOp,
    lhs: Box<Scalar>,
    rhs: Box<Scalar>,
}

#[derive(Clone)]
pub enum Scalar {
    Elem(Elem),
    Num(u64),
    Infix(InfixTerm),
}

impl Scalar {
    fn infix(self, op: InfixOp, rhs: Scalar) -> Scalar {
        let lhs = Box::new(self);
        let rhs = Box::new(rhs);
        Scalar::Infix(InfixTerm { lhs, op, rhs })
    }
}

impl From<u64> for Scalar {
    fn from(value: u64) -> Self {
        Scalar::Num(value)
    }
}

impl From<char> for Scalar {
    fn from(symbol: char) -> Self {
        Scalar::Elem(Elem::from(symbol))
    }
}

pub fn var(symbol: char) -> Scalar { Scalar::from(symbol) }

pub fn num(number: u64) -> Scalar { Scalar::from(number) }

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Self) -> Self::Output {
        self.infix(InfixOp::Plus, rhs)
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Self) -> Self::Output {
        self.infix(InfixOp::Minus, rhs)
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        self.infix(InfixOp::Times, rhs)
    }
}

impl Div for Scalar {
    type Output = Scalar;

    fn div(self, rhs: Self) -> Self::Output {
        self.infix(InfixOp::Over, rhs)
    }
}

impl Display for InfixOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOp::Plus => { write!(f, "+") }
            InfixOp::Minus => { write!(f, "-") }
            InfixOp::Times => { write!(f, "*") }
            InfixOp::Over => { write!(f, "/") }
        }
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Scalar::Elem(elem) => { elem.fmt(f) }
            Scalar::Num(num) => { num.fmt(f) }
            Scalar::Infix(InfixTerm { op, lhs, rhs }) => {
                let lhs_needs_parens =
                    match lhs.as_ref() {
                        Scalar::Elem(_) => { false }
                        Scalar::Num(_) => { false }
                        Scalar::Infix(InfixTerm{ op: lhs_op, ..}) => {

                        }
                    };
            }
        }
    }
}