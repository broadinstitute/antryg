use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::sync::RwLock;
use uuid::Uuid;
use crate::expr::{Expr, matrix};
use crate::expr::var::Var;
use crate::expr::tag::ExprTag;
use crate::expr::Precedence;
use crate::expr::infix::{Infix, Op};
use crate::expr::fun::Fun;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key {
    uuid: Uuid
}
pub(crate) struct Slate {
    exprs: RwLock<BTreeMap<Key, Expr>>
}

impl Key {
    fn new() -> Self {
        Key { uuid: Uuid::new_v4() }
    }
}
impl Slate {
    pub(crate) fn new() -> Self {
        Slate { exprs: RwLock::new(BTreeMap::new()) }
    }

    fn new_var(&self, name: String) -> ExprTag {
        let key = Key::new();
        self.exprs.write().unwrap().insert(key, Expr::Var(Var::from(name)));
        ExprTag::new(self, key)
    }

    pub(crate) fn new_var_str(&self, name: &'static str) -> ExprTag {
        self.new_var(name.into())
    }

    pub(crate) fn precedence(&self, key: &Key) -> Precedence {
        match self.exprs.read().unwrap().get(key) {
            None => Precedence::Sum,
            Some(expr) => {
                match expr {
                    Expr::Var(_) => Precedence::Atom,
                    Expr::Num(_) => Precedence::Atom,
                    Expr::Infix(infix) => infix.precedence(),
                    Expr::Fun(_) => Precedence::Atom,
                    Expr::Matrix(_) => Precedence::Atom,
                }
            }
        }
    }
    const UNKNOWN_EXPR: &'static str = "???";
    pub(crate) fn fmt_expr(&self, key: &Key, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.exprs.read().unwrap().get(key) {
            None => { write!(f, "{}", Self::UNKNOWN_EXPR) }
            Some(expr) => {
                match expr {
                    Expr::Var(var) => write!(f, "{}", var),
                    Expr::Num(num) => write!(f, "{}", num),
                    Expr::Infix(infix) => {
                        let lhs_needs_parens =
                            self.precedence(&infix.lhs) < infix.precedence();
                        if lhs_needs_parens { write!(f, "(")?; }
                        self.fmt_expr(&infix.lhs, f)?;
                        if lhs_needs_parens { write!(f, ")")?; }
                        write!(f, "{}", infix.op)?;
                        let rhs_needs_parens =
                            self.precedence(&infix.rhs) < infix.precedence() ||
                            (self.precedence(&infix.rhs) == infix.precedence() &&
                                infix.op.rhs_peer_needs_parens());
                        if rhs_needs_parens { write!(f, "(")?; }
                        self.fmt_expr(&infix.rhs, f)?;
                        if rhs_needs_parens { write!(f, ")")?; }
                        Ok(())
                    },
                    Expr::Fun(fun) => {
                        match fun {
                            Fun::Pow(x, y) => {
                                write!(f, "pow(")?;
                                self.fmt_expr(x, f)?;
                                write!(f, ",")?;
                                self.fmt_expr(y, f)?;
                                write!(f, ")")
                            }
                        }
                    }
                    Expr::Matrix(matrix) => {
                        write!(f, "[")?;
                        for i_row in 0..matrix.n_rows {
                            if i_row > 0 { write!(f, ",")?; }
                            write!(f, "[")?;
                            for i_col in 0..matrix.n_cols {
                                if i_col > 0 { write!(f, ",")?; }
                                self.fmt_expr(&matrix[i_row][i_col], f)?;
                            }
                            write!(f, "]")?;
                        }
                        write!(f, "]")?;
                        Ok(())
                    }
                }
            }
        }
    }

    pub(crate) fn new_num(&self, value: u64) -> ExprTag {
        let key = Key::new();
        self.exprs.write().unwrap().insert(key, Expr::Num(value.into()));
        ExprTag::new(self, key)
    }

    pub(crate) fn new_infix(&self, op: Op, lhs: Key, rhs: Key) -> ExprTag {
        let key = Key::new();
        self.exprs.write().unwrap().insert(key, Expr::Infix(Infix { op, lhs, rhs }));
        ExprTag::new(self, key)
    }
    pub(crate) fn pow(&self, x: Key, y: Key) -> ExprTag {
        let key = Key::new();
        self.exprs.write().unwrap().insert(key, Expr::Fun(Fun::Pow(x, y)));
        ExprTag::new(self, key)
    }
    pub(crate) fn new_matrix_fill(&self, n_rows: usize, n_cols: usize,
                                  f: impl Fn(usize, usize) -> Key) -> ExprTag {
        let key = Key::new();
        let matrix = matrix::Matrix::fill(n_rows, n_cols, f);
        self.exprs.write().unwrap().insert(key, Expr::Matrix(matrix));
        ExprTag::new(self, key)
    }
}