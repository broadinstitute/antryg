use std::fmt::Display;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum Var { E(usize), T(usize) }

impl Var {
    pub(crate) fn list(n_endos: usize, n_traits: usize) -> Vec<Var> {
        (0..n_endos).map(Var::E)
            .chain((0..n_traits).map(Var::T))
            .collect()
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Var::E(i_endo) => write!(f, "E_{}", i_endo),
            Var::T(i_trait) => write!(f, "T_{}", i_trait),
        }
    }
}
