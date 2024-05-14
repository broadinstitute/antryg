use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::MargeConfig;
use crate::error::Error;
use crate::joidis::write_joint_likelihood;
use crate::out::OutWriter;

pub(crate) fn marge(config: MargeConfig) -> Result<(), Error> {
    let maxima_file = pick_tmp_file();
    write_maxima_file(&maxima_file, &config)?;
    run_maxima_file(&maxima_file)?;
    Ok(())
}

fn write_maxima_file(file: &PathBuf, config: &MargeConfig) -> Result<(), Error> {
    let mut writer = OutWriter::new_path(file)?;
    let n_endos = config.n_endos;
    let n_traits = config.n_traits;
    let (vars, args) = write_joint_likelihood(&mut writer, n_endos, n_traits)?;
    let mut marge = format!("L({args})");
    let mut vars_left = vars.clone();
    while let Some(var) = vars_left.pop() {
        marge = format!("integrate({marge},{var},-inf,inf)");
        writeln!(writer, "{};", marge)?;
    }
    Ok(())
}

fn run_maxima_file(file: &PathBuf) -> Result<(), Error> {
    let output =
        Command::new("maxima")
            .arg("--batch")
            .arg(file)
            .output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    let status = output.status;
    if status.success() {
        Ok(())
    } else {
        Err(Error::from(format!("Maxima failed with exit value {}.", status)) )
    }
}

fn pick_tmp_file() -> PathBuf {
    let tmp_dir = std::env::temp_dir();
    let time_stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    tmp_dir.join(format!("antryg_marge_{:x}.max", time_stamp))
}