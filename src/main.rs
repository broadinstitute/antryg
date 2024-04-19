mod cli;

use antryg::example;
use crate::cli::get_config;
use antryg::config::Config;
use antryg::error::Error;

fn main() -> Result<(), Error>{
  let config = get_config();
  match config {
    Ok(Config::Example) => {
      example();
      Ok(())
    },
    Err(error) => {
        eprintln!("{}", error);
        Err(error)
    }
  }
}