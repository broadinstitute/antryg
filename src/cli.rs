use clap::command;
use antryg::config::Config;
use antryg::error::Error;

mod commands {
    pub(crate) const EXAMPLE: &str = "example";
    pub(crate) const VALID_LIST: &str = "currently only valid command is `example'.";
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!(commands::EXAMPLE).about("Run example")
        )
        .get_matches();
    match matches.subcommand() {
        Some((commands::EXAMPLE, _)) => { Ok(Config::Example) }
        Some((subcommand, _)) => {
            Err(Error::from(
                format!("Unknown subcommand '{}', {}.", subcommand, commands::VALID_LIST)
            ))
        }
        None => {
            Err(Error::from(format!("No subcommand given, {}.", commands::VALID_LIST)))
        }
    }
}