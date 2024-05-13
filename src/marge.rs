use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use crate::config::MargeConfig;
use crate::error::Error;

pub(crate) fn marge(config: MargeConfig) -> Result<(), Error> {
    Ok(())
}

fn pick_tmp_file() -> PathBuf {
    let tmp_dir = std::env::temp_dir();
    let time = UNIX_EPOCH.elapsed().unwrap().as_millis() as u64;
    tmp_dir.join(format!("antryg_marge_{}.max", time))
}