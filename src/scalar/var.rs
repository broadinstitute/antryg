pub struct Var {
    symbol: char
}

impl From<char> for Var {
    fn from(symbol: char) -> Self {
        Var { symbol }
    }
}