use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Var {
    symbol: char
}

impl From<char> for Var {
    fn from(symbol: char) -> Self {
        Var { symbol }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.symbol.fmt(f)
    }
}