use std::fmt::Display;
use crate::config::MahalConfig;
use crate::error::Error;
use crate::out::OutWriter;
use std::io::Write;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Var { E(usize), T(usize) }

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
            Var::E(i_endo) => write!(f, "E{}", i_endo),
            Var::T(i_trait) => write!(f, "T{}", i_trait),
        }
    }
}

fn prefixed(pre: &str, list: &[Var]) -> String {
    list.iter().map(|v| format!("{}{}", pre, v)).collect::<Vec<String>>().join(",")
}

fn new_matrix(pre: &str, list: &[Var]) -> Vec<Vec<String>> {
    list.iter().map(|i_row| {
        list.iter().map(|i_col| {
            format!("{}_{}_{}", pre, i_row, i_col)
        }).collect::<Vec<String>>()
    }).collect::<Vec<Vec<String>>>()
}

fn matrix_to_max(matrix: Vec<Vec<String>>) -> String {
    let rows = matrix.iter().map(|row| {
        let elements = row.join(",");
        format!("[{}]", elements)
    }).collect::<Vec<String>>().join(",");
    format!("matrix({})", rows)
}

pub fn mahal(config: MahalConfig) -> Result<(), Error> {
    let mut writer = OutWriter::new(config.out)?;
    let vars = Var::list(config.n_endos, config.n_traits);
    writeln!(writer, "x: [{}];", prefixed("", &vars))?;
    writeln!(writer, "mu: [{}];", prefixed("mu_", &vars))?;
    writeln!(writer, "xm: x - mu;")?;
    let matrix = new_matrix("Lam", &vars);
    writeln!(writer, "Lam: {};", matrix_to_max(matrix))?;
    writeln!(writer, "L: xm . Lam . xm;")?;
    Ok(())
}