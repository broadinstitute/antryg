pub(crate) mod var;
mod num;
mod infix;
mod tag;

use std::fmt::Display;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Precedence {
    Sum, Product, Power, Call, Atom
}

pub(crate) enum Expr {
    Var(var::Var),
    Num(num::Num),
    Infix(infix::Infix),
}

impl From<u64> for Expr {
    fn from(value: u64) -> Self {
        Expr::Num(num::Num::from(value))
    }
}

impl From<&'static str> for Expr {
    fn from(name: &'static str) -> Self {
        Expr::Var(var::Var::from(name))
    }
}

impl From<String> for Expr {
    fn from(name: String) -> Self {
        Expr::Var(var::Var::from(name))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(var) => write!(f, "{}", var),
            Expr::Num(num) => write!(f, "{}", num),
            Expr::Infix(infix) => write!(f, "{}", infix),
        }
    }
}