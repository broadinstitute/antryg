pub(crate) mod var;
mod num;
pub(crate) mod infix;
pub(crate) mod tag;
pub(crate) mod fun;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum Precedence {
    Sum, Product, Atom
}

pub(crate) enum Expr {
    Var(var::Var),
    Num(num::Num),
    Infix(infix::Infix),
    Fun(fun::Fun)
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
