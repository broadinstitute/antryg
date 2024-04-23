mod cli;

use antryg::example;
use crate::cli::get_config;
use antryg::config::Config;
use antryg::error::Error;
use antryg::mahal;

fn main() -> Result<(), Error> {
    let config = get_config();
    match config {
        Ok(Config::Example) => {
            example();
            Ok(())
        }
        Ok(Config::Mahal(config)) => { mahal::mahal(config) }
        Err(error) => {
            eprintln!("{}", error);
            Err(error)
        }
    }
}