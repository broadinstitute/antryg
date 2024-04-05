use std::fmt::{Display, Formatter};
use crate::expr::{Expr, Precedence};
use crate::slate;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Op {
    Plus,
    Minus,
    Times,
    Over,
    Power,
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
            Op::Power => Precedence::Power,
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Times => "*",
            Op::Over => "/",
            Op::Power => "^",
        }
    }
    fn rhs_peer_needs_parens(&self) -> bool {
        match self {
            Op::Plus | Op::Minus => false,
            Op::Times | Op::Over | Op::Power => true,
        }
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.lhs.precedence() < self.precedence() {
            write!(f, "({})", self.lhs)?;
        } else {
            write!(f, "{}", self.lhs)?;
        }
        write!(f, "{}", self.op.symbol())?;
        if (self.rhs.precedence() < self.precedence()) ||
            ((self.rhs.precedence() == self.precedence()) && self.op.rhs_peer_needs_parens()) {
            write!(f, "({})", self.rhs)
        } else {
            write!(f, "{}", self.rhs)
        }
    }
}

impl<L: Expr, R: Expr> Expr for Infix<L, R> {
    fn precedence(&self) -> Precedence {
        self.op.precedence()
    }
}