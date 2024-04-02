use crate::scalar::var::Var;

pub enum Elem {
    Var(Var),
    Indexed(Box<Elem>, Var)
}

impl From<char> for Elem {
    fn from(symbol: char) -> Self {
        Elem::Var(Var::from(symbol))
    }
}