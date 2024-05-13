use crate::config::MahalConfig;
use crate::error::Error;
use crate::out::OutWriter;
use std::io::Write;
use crate::joydis::Var;


fn prefixed(pre: &str, list: &[Var]) -> String {
    list.iter().map(|v| format!("{}{}", pre, v)).collect::<Vec<String>>().join(",")
}

fn precision_matrix(list: &[Var], n_traits: usize) -> Vec<Vec<String>> {
    list.iter().map(|var_row| {
        list.iter().map(|var_col| {
            match (var_row, var_col) {
                (Var::E(i_endo1), Var::E(i_endo2)) => {
                    if i_endo1 == i_endo2 {
                        let beta_sum =
                            (0..n_traits).map(|i_trait|
                                format!("beta_{i_endo1}_{i_trait}^2/sigma_{i_trait}^2")
                            ).collect::<Vec<String>>().join("+");
                        format!("1/tau_{var_row}^2+{beta_sum}")
                    } else {
                        (0..n_traits).map(|i_trait|
                            format!("beta_{i_endo1}_{i_trait}*beta_{i_endo2}_{i_trait}/sigma_{i_trait}^2")
                        ).collect::<Vec<String>>().join("+")
                    }
                }
                (Var::T(i_trait1), Var::T(i_trait2)) => {
                    if i_trait1 == i_trait2 {
                        format!("1/sigma_{i_trait1}^2 + 1/s_{i_trait1}^2")
                    } else {
                        "0".to_string()
                    }
                }
                (Var::E(i_endo), Var::T(i_trait)) => {
                    format!("-beta_{i_endo}_{i_trait}/sigma_{i_trait}^2")
                }
                (Var::T(i_trait), Var::E(i_endo)) => {
                    format!("-beta_{i_endo}_{i_trait}/sigma_{i_trait}^2")
                }
            }
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

pub(crate) fn mahal(config: MahalConfig) -> Result<(), Error> {
    let mut writer = OutWriter::new(config.out)?;
    let n_endos = config.n_endos;
    let n_traits = config.n_traits;
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
    writeln!(writer, "L({args}) := {} + {} + {};", e_sum, e_t_sum, t_sum)?;
    let l_func = format!("L({args})");
    for var in &vars {
        writeln!(writer, "define(L_{var}({args}), diff({l_func}, {var}));")?;
    }
    let derivatives =
        vars.iter().map(|var| format!("L_{var}({args})")).collect::<Vec<String>>();
    writeln!(writer, "sols: solve([{}], [{args}]);", derivatives.join(","))?;
    writeln!(writer, "x({args}) := [{}];", prefixed("", &vars))?;
    let mus =
        vars.iter().enumerate().map(|(i_var, _)|
            format!("rhs(sols[1][{}])", i_var + 1)
        ).collect::<Vec<String>>();
    writeln!(writer, "mu: [{}];", mus.join(","))?;
    writeln!(writer, "xm({args}) := x({args}) - mu;")?;
    let matrix = precision_matrix(&vars, n_traits);
    writeln!(writer, "Lam: {};", matrix_to_max(matrix))?;
    writeln!(writer, "LN({args}) := xm({args}) . Lam . xm({args});")?;
    writeln!(writer, "D({args}) := LN({args}) - L({args});")?;
    writeln!(writer, "ratexpand(D({args}));")?;
    Ok(())
}