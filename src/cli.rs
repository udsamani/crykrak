use std::ffi::OsString;

use clap::{Arg, ArgMatches, Command};

pub fn get_cli_arguments<'a, I, T>(args: I) -> ArgMatches
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone + 'a,
{
    let command = build_command();
    command.get_matches_from(args)
}

fn build_command() -> Command<'static> {
    Command::new("crykrak")
        .version("1.0")
        .about("A command line tool for processing off chain transactions")
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .takes_value(true)
                .value_name("FILE")
                .help("Path to the CSV file containing the transactions."),
        )
}
