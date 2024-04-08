use std::fmt::{Display, Formatter};
use crate::expr::Precedence;
use crate::slate;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Op {
    Plus,
    Minus,
    Times,
    Over,
}

pub(crate) struct Infix {
    pub(crate) op: Op,
    pub(crate) lhs: slate::Key,
    pub(crate) rhs: slate::Key,
}

impl Op {
    fn precedence(&self) -> Precedence {
        match self {
            Op::Plus | Op::Minus => Precedence::Sum,
            Op::Times | Op::Over => Precedence::Product,
        }
    }
    pub(crate) fn symbol(&self) -> &'static str {
        match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Times => "*",
            Op::Over => "/",
        }
    }
    pub(crate) fn rhs_peer_needs_parens(&self) -> bool {
        match self {
            Op::Plus | Op::Minus => false,
            Op::Times | Op::Over => true,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl Infix {
    pub(crate) fn precedence(&self) -> Precedence {
        self.op.precedence()
    }
}