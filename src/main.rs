use antryg::error::Error;
use antryg::run;

use crate::cli::get_config;

mod cli;

fn main() -> Result<(), Error> {
    let config = get_config()?;
    let result = run(config);
    match result {
        Ok(_) => {
            println!("Done!");
            Ok(())
        }
        Err(error) => {
            eprintln!("{}", error);
            Err(error)
        }
    }
}