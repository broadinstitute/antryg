use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use crate::expr::infix::Op;

use crate::slate::{Key, Slate};

#[derive(Clone, Copy)]
pub struct ExprTag<'a> {
    slate: &'a Slate,
    key: Key
}

impl<'a> ExprTag<'a> {
    pub fn new(slate: &'a Slate, key: Key) -> Self {
        ExprTag { slate, key }
    }
}

impl<'a> Display for ExprTag<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.slate.fmt_expr(&self.key, f)
    }
}

impl PartialEq for ExprTag<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for ExprTag<'_> {}

impl PartialOrd for ExprTag<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExprTag<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<'a> Add for ExprTag<'a> {
    type Output = ExprTag<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        self.slate.new_infix(Op::Plus, self.key, rhs.key)
    }
}

impl<'a> Sub for ExprTag<'a> {
    type Output = ExprTag<'a>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.slate.new_infix(Op::Minus, self.key, rhs.key)
    }
}

impl<'a> Mul for ExprTag<'a> {
    type Output = ExprTag<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.slate.new_infix(Op::Times, self.key, rhs.key)
    }
}

impl<'a> Div for ExprTag<'a> {
    type Output = ExprTag<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        self.slate.new_infix(Op::Over, self.key, rhs.key)
    }
}

impl<'a> ExprTag<'a> {
    pub fn pow(self, rhs: Self) -> Self {
        self.slate.pow(self.key, rhs.key)
    }
}