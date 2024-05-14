use std::fmt::Display;
use std::io::Write;

use crate::error::Error;
use crate::out::OutWriter;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
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

pub(crate) fn write_joint_likelihood(writer: &mut OutWriter, n_endos: usize,
                                               n_traits: usize)
    -> Result<(Vec<Var>, String), Error> {
    let vars = Var::list(n_endos, n_traits);
    let args =
        vars.iter().map(|var| var.to_string()).collect::<Vec<String>>().join(",");
    for i_endo in 0..n_endos {
        writeln!(writer, "assume(tau_{i_endo} > 0);")?;
    }
    for i_trait in 0..n_traits {
        writeln!(writer, "assume(sigma_{i_trait} > 0);")?;
    }
    for i_trait in 0..n_traits {
        writeln!(writer, "assume(s_{i_trait} > 0);")?;
    }
    let e_sum =
        (0..n_endos).map(|i_endo|
            format!("((E_{i_endo} - mu_{i_endo})/tau_{i_endo})^2")
        ).collect::<Vec<String>>().join("+");
    let e_t_sum =
        (0..n_traits).map(|i_trait| {
            let beta_sum =
                (0..n_endos).map(|i_endo|
                    format!("beta_{i_endo}_{i_trait}*E_{i_endo}")
                ).collect::<Vec<String>>().join("+");
            format!("((T_{i_trait} - ({beta_sum}))/sigma_{i_trait})^2")
        }).collect::<Vec<String>>().join("+");
    let t_sum =
        (0..n_traits).map(|i_trait|
            format!("((T_{i_trait} - O_{i_trait})/s_{i_trait})^2")
        ).collect::<Vec<String>>().join("+");
    writeln!(writer, "L({args}) := exp(-(1/2)*({} + {} + {}));", e_sum, e_t_sum, t_sum)?;
    Ok((vars, args))
}
