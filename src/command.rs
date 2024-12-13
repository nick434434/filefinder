use clap::{Arg, ArgAction, ArgMatches};
use clap::Command;


pub fn filefinder_cmd() -> Command {
    let command: Command = Command::new("filefinder")
        .arg(Arg::new("directory")
            .short('d')
            .required(false)
            .action(ArgAction::Set)
            .default_value("~"))
        .arg(Arg::new("name")
            .short('n')
            .required(false)
            .action(ArgAction::SetTrue))
        .arg(Arg::new("recursive")
            .short('r')
            .required(false)
            .action(ArgAction::SetTrue))
        .arg(Arg::new("query")
            .required(true));

    command
}

pub fn get_argument_value<'a>(arg_matches: &'a ArgMatches, argument_name: &str) -> &'a str {
    arg_matches.get_raw(argument_name).expect("").next().expect("Argument value is required").to_str().unwrap()
}
