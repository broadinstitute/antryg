use clap::{Arg, command};
use antryg::config::{Config, MahalConfig, MargeConfig};
use antryg::error::Error;

mod commands {
    pub(crate) const EXAMPLE: &str = "example";
    pub(crate) const MAHAL: &str = "mahal";
    pub(crate) const MARGE: &str = "marge";
    pub(crate) const VALID_LIST: &str =
        "currently the only valid commands are `example', 'mahal' and 'marge.";
}

struct MyArg {
    about: &'static str,
    name: &'static str,
    short: char,
}

mod args {
    use crate::cli::MyArg;

    pub(crate) const N_ENDOS: MyArg = MyArg {
        about: "Number of endophenotypes",
        name: "n_endos",
        short: 'e',
    };
    pub(crate) const N_TRAITS: MyArg = MyArg {
        about: "Number of traits",
        name: "n_traits",
        short: 't',
    };
    pub(crate) const OUT: MyArg = MyArg {
        about: "Output file",
        name: "out",
        short: 'o',
    };
}

fn new_arg(my_arg: &MyArg) -> Arg {
    Arg::new(my_arg.name).short(my_arg.short).long(my_arg.name).value_name(my_arg.name)
}

fn missing_option_error(my_arg: &MyArg) -> Error {
    Error::from(format!("Missing {} ({}) option ('--{}' or '-{}').", my_arg.name,
                        my_arg.about, my_arg.name, my_arg.short))
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!(commands::EXAMPLE).about("Run example")
        )
        .subcommand(
            command!(commands::MAHAL).about("Run Mahalanobis distance example")
                .arg(new_arg(&args::N_ENDOS).value_parser(clap::value_parser!(usize))
                    .required(true))
                .arg(new_arg(&args::N_TRAITS).value_parser(clap::value_parser!(usize))
                    .required(true))
                .arg(new_arg(&args::OUT))
        )
        .subcommand(
            command!(commands::MARGE).about("Run Marge example")
                .arg(new_arg(&args::N_ENDOS).value_parser(clap::value_parser!(usize))
                    .required(true))
                .arg(new_arg(&args::N_TRAITS).value_parser(clap::value_parser!(usize))
                    .required(true))
        )
        .get_matches();
    match matches.subcommand() {
        Some((commands::EXAMPLE, _)) => { Ok(Config::Example) }
        Some((commands::MAHAL, sub_matches)) => {
            let n_endos =
                sub_matches.get_one::<usize>("n_endos").cloned().ok_or_else(|| {
                    missing_option_error(&args::N_ENDOS)
                })?;
            let n_traits =
                sub_matches.get_one::<usize>("n_traits").cloned().ok_or_else(|| {
                    missing_option_error(&args::N_TRAITS)
                })?;
            let out = sub_matches.get_one::<String>("out").cloned();
            Ok(Config::Mahal(MahalConfig { n_endos, n_traits, out }))
        }
        Some((commands::MARGE, sub_matches)) => {
            let n_endos =
                sub_matches.get_one::<usize>("n_endos").cloned().ok_or_else(|| {
                    missing_option_error(&args::N_ENDOS)
                })?;
            let n_traits =
                sub_matches.get_one::<usize>("n_traits").cloned().ok_or_else(|| {
                    missing_option_error(&args::N_TRAITS)
                })?;
            Ok(Config::Marge(MargeConfig { n_endos, n_traits }))
        }
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